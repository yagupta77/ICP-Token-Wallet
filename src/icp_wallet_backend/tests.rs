#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_transfer() {
        let mut wallet = Wallet::default();

        // Alice receives 100 tokens
        wallet.receive_tokens("alice".to_string(), 100);
        assert_eq!(wallet.get_balance(&"alice".to_string()), 100);

        // Alice sends 50 tokens to Bob
        wallet.send_tokens("alice".to_string(), "bob".to_string(), 50).unwrap();
        assert_eq!(wallet.get_balance(&"alice".to_string()), 50);
        assert_eq!(wallet.get_balance(&"bob".to_string()), 50);
    }
}
