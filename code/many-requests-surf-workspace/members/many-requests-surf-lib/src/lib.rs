pub async fn many_requests(urls: &[String])
                           -> Vec<Result<String, surf::Error>>
{
    let client = surf::Client::new();

    let mut handles = vec![];
    for url in urls {
        let request = client.get(&url).recv_string();
        handles.push(async_std::task::spawn(request));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }

    results
}

#[cfg(test)]
mod tests {
    // use super::*;


}
