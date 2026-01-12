# batis4sqlx

*A personal Rust ORM project built on SQLx, inspired by MyBatis-Plus.*

### How to use

<br/>

#### 1.use crate

```rust
use batis4sqlx::ServiceImpl;
use batis4sqlx::batis4sqlx_macros::{entity, Entity, repository};
use batis4sqlx::repository::MySqlRepository;
use serde::Serialize;
use sqlx::{FromRow, MySqlPool};
use std::collections::HashSet;
use std::rc::Rc;
```

#### 2.definition entity struct

```rust
/// user struct
///
/// all table fields need to be wrapped with Option.
/// #[entity] has 1 attribute
/// `table_name` used to specify the table name.
/// 
/// #[entity_field] has 3 attribute, If there are multiple #[entity_field] in the same field, only the first #[entity_field] will take effect.
/// `primary_key` used to specify the primary key name. If not used, it will be named "id" by default.
/// `name` used for field aliases.(equivalent to #[sqlx(rename = "alias")])
/// `skip` used to ignore lambda field function.
#[entity(table_name = "user")]
#[derive(Serialize, FromRow, Default, Debug, Entity)]
struct User {
    #[entity_field(primary_key, name = "ID")]
    // #[entity_field(skip)] other #[entity_field] invalid.
    id: Option<u64>,
    username: Option<String>,
    #[entity_field(name = "pwd")]
    password: Option<String>,
    #[sqlx(skip)]
    #[entity_field(skip)]
    ignore: String,
}
```

#### 3.definition repository

```rust
/// user repository
#[repository(db_type = "mysql", entity_path = "User")]
struct UserRepository {
    db: Rc<MySqlPool>,
}

/// implements MySqlRepository
impl MySqlRepository<User> for UserRepository {
    fn borrow_db(&self) -> &MySqlPool {
        &*self.db
    }
}
```

#### 4.definition service

```rust
/// user service
struct UserService {
    user_repository: Rc<UserRepository>,
}

/// implements ServiceImpl
impl<'a, 'd> ServiceImpl<'a, 'd, User> for UserService {
    fn borrow_db(&self) -> &MySqlPool {
        self.user_repository.borrow_db()
    }
}
```

#### 5.start use
```rust
 #[tokio::test]
 async fn test_async() {
     // connect mysql database
     let uri = "mysql://root:password@127.0.0.1:3306/db_name";
     let mysql_pool = Rc::new(MySqlPool::connect(&uri).await.unwrap());

     // new repository
     let user_repository = Rc::new(UserRepository {
         db: mysql_pool.clone(),
     });

     // new service
     let user_service = UserService {
         user_repository: user_repository.clone(),
     };

     // new user
     let mut user = User::default();
     user.username = Some("test".to_string());
     user.password = Some("123456".to_string());

     // save operation, automatically ignore null value fields.
     // when the primary key type is u64 or Option<u64>, saving successfully will automatically set the primary key.
     assert_eq!(user.id, None);
     let save_result = user_repository.save(&mut user).await;
     if let Ok(rows) = save_result {
         println!(
             "save success! rows:{rows}, last insert id: {}",
             user.id.unwrap()
         );
     } else {
         let error = save_result.unwrap_err();
         println!("save error: {}", error);
     }

     user.username = Some("admin".to_string());
     user.password = Some("admin123456".to_string());
     // update operation, automatically ignore null value fields.
     let update_result = user_repository.update_by_primary_key(&user).await;
     if let Ok(rows) = update_result {
         println!("update success! rows: {rows}");
     } else {
         let error = update_result.unwrap_err();
         println!("update error: {}", error);
     }

     let mut primary_keys = HashSet::new();
     primary_keys.insert(1u64);
     // delete operation
     let delete_result = user_repository.delete_in_primary_keys(primary_keys).await;
     if let Ok(rows) = delete_result {
         println!("delete success! rows: {rows}");
     } else {
         let error = delete_result.unwrap_err();
         println!("delete error: {}", error);
     }

     // for some of the following lambda operations, reference Mybatis-Plus, I won't go into details here.

     // lambda query
     let user_opt = user_service
         .lambda_query()
         .eq(User::id_field, 1)
         .eq(User::username_field, "test")
         .eq_flag(User::id_field, 2, false)
         .or()
         .ne_opt(User::username_field, Some("admin"))
         .opt()
         .await
         .unwrap();
     if let Some(user) = user_opt {
         println!("{user:?}");
     }

     // lambda update
     let rows = user_service
         .lambda_update()
         .eq(User::id_field, 1)
         .eq(User::username_field, "test")
         .set(User::password_field, "123456")
         .execute()
         .await
         .unwrap();
     println!("update success! rows: {rows}");

     // lambda delete
     let rows = user_service
         .lambda_delete()
         .eq(User::id_field, 1)
         .eq(User::username_field, "test")
         .execute()
         .await
         .unwrap();
     println!("delete success! rows: {rows}");

     // lambda query by primary key
     let _user_opt = user_service.get_by_primary_key(1).await.unwrap();

     // lambda query all data
     let _user_vec = user_service.vec().await.unwrap();
 }
```