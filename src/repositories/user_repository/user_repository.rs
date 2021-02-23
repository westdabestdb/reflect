use crate::config::strings::{
    EMAIL_EXIST, INVALID_TOKEN, REGISTRATION_FAIL, REGISTRATION_SUCCESS, SOMETHING_WRONG,
    USERNAME_EXIST,
};
use crate::config::{Config, IConfig};
use crate::models::response::{LoginResponse, Response};
use crate::models::user::{Claims, Register, User};
use chrono::{DateTime, Duration, Utc};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use mongodb::error::Error;
use mongodb::sync::Client;

pub trait IUserRepository {
    fn find_user_with_email(&self, email: String) -> Result<Option<User>, Error>;
    fn find_user_with_username(&self, username: String) -> Result<Option<User>, Error>;
    // fn login(&self, login: Login) -> Result<LoginResponse, Response>;
    fn register(&self, user: Register) -> Result<LoginResponse, Response>;
    fn user_informations(&self, token: &str) -> Result<Option<User>, Response>;
    fn protected_function(&self) -> bool;
}

pub struct UserRepository {
    pub connection: Client,
}

impl IUserRepository for UserRepository {
    fn find_user_with_email(&self, email: String) -> Result<Option<User>, Error> {
        let _config: Config = Config {};
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let collection_name = _config.get_config_with_key("USER_COLLECTION_NAME");
        let db = self.connection.database(database_name.as_str());
        let cursor = db
            .collection(collection_name.as_str())
            .find_one(doc! {"email": email}, None)
            .unwrap();
        match cursor {
            Some(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                Ok(model) => Ok(model),
                Err(e) => Err(Error::from(e)),
            },
            None => Ok(None),
        }
    }

    fn find_user_with_username(&self, username: String) -> Result<Option<User>, Error> {
        let _config: Config = Config {};
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let collection_name = _config.get_config_with_key("USER_COLLECTION_NAME");
        let db = self.connection.database(database_name.as_str());
        let cursor = db
            .collection(collection_name.as_str())
            .find_one(doc! {"username": username}, None)
            .unwrap();
        match cursor {
            Some(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                Ok(model) => Ok(model),
                Err(e) => Err(Error::from(e)),
            },
            None => Ok(None),
        }
    }
    // fn login(&self, user: Login) -> Result<LoginResponse, Response> {
    //     match self.find_user_with_email(user.email.to_string()).unwrap() {
    //         Some(x) => {
    //             let mut sha = Sha256::new();
    //             sha.input_str(user.password.as_str());
    //             if x.password == sha.result_str() {
    //                 // JWT
    // let _config: Config = Config {};
    // let _var = _config.get_config_with_key("SECRET_KEY");
    // let key = _var.as_bytes();

    // let mut _date: DateTime<Utc>;
    // // Remember Me
    // if !user.remember_me {
    //     _date = Utc::now() + Duration::hours(1);
    // } else {
    //     _date = Utc::now() + Duration::days(365);
    // }
    // let my_claims = Claims {
    //     sub: user.email,
    //     exp: _date.timestamp() as usize,
    // };
    // let token = encode(
    //     &Header::default(),
    //     &my_claims,
    //     &EncodingKey::from_secret(key),
    // )
    //     .unwrap();
    // Ok(LoginResponse {
    //     status: true,
    //     token,
    //     message: "You have successfully logged in.".to_string(),
    // })
    //             } else {
    //                 Err(Response {
    //                     status: false,
    //                     message: "Check your user informations.".to_string(),
    //                 })
    //             }
    //         }
    //         None => Err(Response {
    //             status: false,
    //             message: "Check your user informations.".to_string(),
    //         }),
    //     }
    // }
    fn register(&self, user: Register) -> Result<LoginResponse, Response> {
        let _email_exist = self
            .find_user_with_email((&user.email).parse().unwrap())
            .unwrap();
        match _email_exist {
            Some(_) => Err(Response {
                message: EMAIL_EXIST.to_string(),
                success: false,
            }),
            None => {
                let _username_exist = self
                    .find_user_with_username((&user.username).parse().unwrap())
                    .unwrap();

                match _username_exist {
                    Some(_) => Err(Response {
                        message: USERNAME_EXIST.to_string(),
                        success: false,
                    }),

                    None => {
                        let _config: Config = Config {};
                        let database_name = _config.get_config_with_key("DATABASE_NAME");
                        let collection_name = _config.get_config_with_key("USER_COLLECTION_NAME");
                        let db = self.connection.database(database_name.as_str());
                        let mut sha = Sha256::new();
                        sha.input_str(user.password.as_str());
                        let hash_pw = sha.result_str();
                        let user_id = uuid::Uuid::new_v4().to_string();
                        let _ex = db.collection(collection_name.as_str()).insert_one(doc! {"user_id": user_id.clone(), "name": user.fullname, "username": user.username, "email": user.email.clone(), "password": hash_pw}, None);
                        match _ex {
                            Ok(_) => {
                                let _config: Config = Config {};
                                let _var = _config.get_config_with_key("SECRET_KEY");
                                let key = _var.as_bytes();
                                let mut _date: DateTime<Utc>;
                                _date = Utc::now() + Duration::hours(1);
                                let my_claims = Claims {
                                    sub: user.email,
                                    user_id: user_id,
                                    exp: _date.timestamp() as usize,
                                };
                                let token = encode(
                                    &Header::default(),
                                    &my_claims,
                                    &EncodingKey::from_secret(key),
                                )
                                .unwrap();

                                return Ok(LoginResponse {
                                    success: true,
                                    token,
                                    message: REGISTRATION_SUCCESS.to_string(),
                                });
                            }
                            Err(_) => Err(Response {
                                success: false,
                                message: REGISTRATION_FAIL.to_string(),
                            }),
                        }
                    }
                }
            }
        }
    }

    fn user_informations(&self, token: &str) -> Result<Option<User>, Response> {
        let _config: Config = Config {};
        let _var = _config.get_config_with_key("SECRET_KEY");
        let key = _var.as_bytes();
        let _decode = decode::<Claims>(
            token,
            &DecodingKey::from_secret(key),
            &Validation::new(Algorithm::HS256),
        );
        match _decode {
            Ok(decoded) => {
                match self.find_user_with_email((decoded.claims.sub.to_string()).parse().unwrap()) {
                    Ok(user) => Ok(user),
                    Err(_) => Err(Response {
                        success: false,
                        message: SOMETHING_WRONG.to_string(),
                    }),
                }
            }
            Err(_) => Err(Response {
                success: false,
                message: INVALID_TOKEN.to_string(),
            }),
        }
    }

    fn protected_function(&self) -> bool {
        true
    }
}