// Transaktionslogik
use crate::utils::hash_data;

pub struct Transaction {
    sender: String,
    reciever: String,
    amount: u64,
    tx_id: String,
}

impl Transaction {
    fn calculate_tx_id(&self) -> String {
        let hash_value = format!("{}{}{}", self.sender, self.reciever, self.amount);
        hash_data(&hash_value)
    }

    pub fn new(sender: String, reciever: String, amount: u64) -> Transaction {
        let mut tx = Transaction {
            sender: sender,
            reciever: reciever,
            amount: amount,
            tx_id: String::from(""),
        };

        tx.tx_id = tx.calculate_tx_id();
        tx
    }

    pub fn reciever(&self) -> &str {
        &self.reciever
    }

    pub fn sender(&self) -> &str {
        &self.sender
    }
}
