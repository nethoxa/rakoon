pub fn legacy_contract_creation(conf: &TxConf) -> Result<TransactionRequest, SpammerError> {
    let tx = TransactionRequest {
        from: Some(conf.from),
        to: Some(TxKind::Call(conf.to)),
        gas: Some(30_000_000),
    };
}

pub fn legacy_tx(conf: &TxConf) -> Result<TransactionRequest, SpammerError> {
    let tx = TransactionRequest {
        from: Some(conf.from),
        to: Some(TxKind::Call(conf.to)),
        gas: Some(30_000_000),
    };
}   