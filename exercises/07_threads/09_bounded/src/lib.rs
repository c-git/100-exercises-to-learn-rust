// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{self, Receiver, Sender, TrySendError};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: mpsc::SyncSender<Command>,
}
#[derive(Debug, thiserror::Error)]
#[error("Channel is Full")]
pub struct ErrorChannelFull<T: Send> {
    input: T,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, ErrorChannelFull<TicketDraft>> {
        let (tx, rx) = mpsc::sync_channel(1);
        if let Err(e) = self.sender.try_send(Command::Insert {
            draft,
            response_channel: tx,
        }) {
            match e {
                TrySendError::Full(d) | TrySendError::Disconnected(d) => match d {
                    Command::Insert {
                        draft,
                        response_channel: _,
                    } => return Err(ErrorChannelFull { input: draft }),
                    _ => unreachable!(),
                },
            }
        }
        Ok(rx.recv().unwrap())
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, ErrorChannelFull<TicketId>> {
        let (tx, rx) = mpsc::sync_channel(1);
        if let Err(e) = self.sender.try_send(Command::Get {
            id,
            response_channel: tx,
        }) {
            match e {
                TrySendError::Full(x) | TrySendError::Disconnected(x) => match x {
                    Command::Insert { .. } => unreachable!(),
                    Command::Get {
                        id,
                        response_channel: _,
                    } => return Err(ErrorChannelFull { input: id }),
                },
            }
        }
        Ok(rx.recv().unwrap())
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (tx, rx) = mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(rx));
    TicketStoreClient { sender: tx }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: mpsc::SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: mpsc::SyncSender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.try_send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id).cloned();
                let _ = response_channel.try_send(ticket);
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
