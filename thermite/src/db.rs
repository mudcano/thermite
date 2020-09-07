use diesel::{
    prelude::*,
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool}
};
use crate::schema;
use tokio_diesel::*;
use chrono::NaiveDateTime;
use tokio::{
    sync::mpsc::{channel, Sender, Receiver}
};
use std::{
    error::Error
};

// Request creation of a new user with this struct.
pub struct ReqUserCreate {
    pub username: String,
    pub password: String
}

// Request adding an email to a new user with this struct.
pub struct ReqUserSetEmail {
    pub user_id: isize,
    pub email: Option<String>
}


pub struct ReqBanUser {
    pub user_id: isize,
    pub banned_by: isize,
    pub ban_reason: String,
    pub ban_until: Option<NaiveDateTime>
}

pub struct ReqUserActive {
    pub user_id: isize,
    pub active: bool
}

pub struct ReqUsetSetPassword {
    pub user_id: isize,
    pub new_password: String
}

// Set to None to delete it.
pub struct ReqUserStorageSet {
    pub user_id: isize,
    pub category: String,
    pub storage_name: String,
    pub json_data: Option<serde_json::Value>
}


pub struct ReqUserStorageWipe {
    pub user_id: isize,
    pub category: String
}

pub enum Msg2DbManager {
    CreateUser(ReqUserCreate),
    AddEmail(ReqAddEmail),
    RemEmail(ReqRemEmail),
    PrimaryEmail(ReqPrimEmail),
    EnableUser(ReqEnableUser),
    DisableUser(ReqDisableUser),
    SetPassword(ReqSetPassword),
    SetStorage(ReqSetStorage),
    DelStorage(ReqDelStorage),
    WipeStorage(ReqWipeStorage),
    BanUser(ReqBanUser),
    UnbanUser(ReqUnbanUser),
    Kill
}

pub struct DbManager{
    rx_dbmanager: Receiver<Msg2DbManager>,
    pub tx_dbmanager: Sender<Msg2DbManager>,
    pool: Pool<ConnectionManager<PgConnection>>
}

impl DbManager {
    pub fn new(url: String) -> Result<Self, Box<dyn Error>> {
        let (tx_dbmanager, rx_dbmanager) = channel(50);

        let mut manager = ConnectionManager::<PgConnection>::new(url);
        let mut pool = Pool::builder().build(manager);

        match pool {
            Ok(pool) => {
                Ok(Self {
                    tx_dbmanager,
                    rx_dbmanager,
                    pool
                })
            },
            Err(e) => {
                Err(e)
            }
        }

        
    }

    pub async fn run(&mut self) -> () {
        loop {
            if let Some(msg) = self.rx_dbmanager.recv().await {
                match msg {
                    Msg2DbManager::CreateUser(req) => {

                    },
                    Msg2DbManager::AddEmail(req) => {

                    },
                    Msg2DbManager::RemEmail(req) => {

                    },
                    Msg2DbManager::PrimaryEmail(req) => {

                    },
                    Msg2DbManager::EnableUser(req) => {

                    },
                    Msg2DbManager::DisableUser(req) => {

                    },
                    Msg2DbManager::SetPassword(req) => {

                    },
                    Msg2DbManager::SetStorage(req) => {

                    },
                    Msg2DbManager::DelStorage(req) => {

                    },
                    Msg2DbManager::WipeStorage(req) => {

                    },
                    Msg2DbManager::BanUser(req) => {

                    },
                    Msg2DbManager::UnbanUser(req) => {

                    }
                    Msg2DbManager::Kill => {
                        break;
                    }
                }
            }
        }
    }
}