struct Channel {
    id : Uuid,
    agent_id: String,
    mode: Interactive | Beacon,
    to_agent: mpsc::Sender<Frame>,
    from_agent: broadcast::Sender<Frame>,
}
