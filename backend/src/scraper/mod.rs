pub async fn fetch_html(url: &str) -> Result<String, String> {
    // Melakukan request GET
    let response = match reqwest::get(url)
        .await
        .map_err(|e| format!("Gagal Menghubungi URL : {}", e))
    {
        Ok(val) => val,
        Err(e) => return Err(e),
    };
    // Cek apakah status sukses
    if !response.status().is_success() {
        return Err(format!("Server mengembalikan error: {}", response.status()));
    }
    // ambil body response
    let html_content = response
        .text()
        .await
        .map_err(|e| format!("Gagal membaca isi HTML: {}", e))?;

    Ok(html_content)
}
//
//#[tokio::test]
//async fn test_fetch_html() {
//    let result = fetch_html("https://example.com").await;
//    assert!(result.is_ok());
//    let html = result.unwrap();
//    println!("HTML length: {} chars", html.len());
//    assert!(html.contains("<html"));
//}
