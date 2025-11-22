use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::wrap_pyfunction;
use pyo3::Bound;

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use tokenizers::Tokenizer;

/// Декодирование файла с ID обратно в текст.
///
/// tokenizer_dir   - папка, где лежит tokenizer.json
/// input_ids_path  - файл с ID (как у тебя wiki_00_ids.txt)
/// output_path     - файл, куда пишем восстановленный текст
/// max_tokens      - максимум ID, которые декодируем (0 = без ограничения)
#[pyfunction]
fn decode_file(
    tokenizer_dir: &str,
    input_ids_path: &str,
    output_path: &str,
    max_tokens: usize,
) -> PyResult<()> {
    // --- грузим tokenizer.json из директории ---
    let tok_path = Path::new(tokenizer_dir).join("tokenizer.json");
    let tok_path_str = tok_path.to_str().ok_or_else(|| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>("Некорректный путь к tokenizer.json")
    })?;

    let tokenizer = Tokenizer::from_file(tok_path_str).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            format!("Ошибка загрузки токенайзера: {e}")
        )
    })?;

    // --- читаем файл с ID ---
    let input_file = File::open(input_ids_path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(
            format!("Не удалось открыть файл ID: {e}")
        )
    })?;
    let mut reader = BufReader::new(input_file);

    let mut buf_line = String::new();
    let mut ids: Vec<u32> = Vec::new();
    let limit = if max_tokens == 0 { None } else { Some(max_tokens) };

    loop {
        buf_line.clear();
        let n = reader.read_line(&mut buf_line).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(
                format!("Ошибка чтения ID: {e}")
            )
        })?;
        if n == 0 {
            break;
        }

        for part in buf_line.split_whitespace() {
            if part.is_empty() {
                continue;
            }
            let id: u32 = part.parse().map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Некорректный ID '{part}': {e}")
                )
            })?;
            ids.push(id);

            if let Some(lim) = limit {
                if ids.len() >= lim {
                    break;
                }
            }
        }

        if let Some(lim) = limit {
            if ids.len() >= lim {
                break;
            }
        }
    }

    // --- декодируем ---
    let decoded = tokenizer
    .decode(&ids, /* skip_special_tokens = */ false)
    .map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            format!("Ошибка декодирования: {e}")
        )
    })?;

    // --- пишем текст ---
    let output_file = File::create(output_path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(
            format!("Не удалось создать выходной файл: {e}")
        )
    })?;
    let mut writer = BufWriter::new(output_file);

    writer.write_all(decoded.as_bytes()).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(
            format!("Ошибка записи текста: {e}")
        )
    })?;
    writer.flush().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(
            format!("Ошибка flush(): {e}")
        )
    })?;

    Ok(())
}

#[pymodule]
fn rust_hf_decoder(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decode_file, m)?)?;
    Ok(())
}

