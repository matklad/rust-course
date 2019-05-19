fn uppercase(data: &str) -> &'static str {
    // unicode, размер новой строки может быть больше,
    // in-place uppercase не работает
    let data: String = data.to_uppercase();

    // убрали поле capacity
    let data: Box<str> = data.into_boxed_str();

    // Вызвали "метод"
    Box::leak(data)
}
