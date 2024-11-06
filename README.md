# 🛠️ | Todo API

<img src="https://img.shields.io/badge/rust-red?style=for-the-badge&label=language&color=%23e0e0e0" /> <img src="https://img.shields.io/badge/rocket-red?style=for-the-badge&label=framework&color=%23e0e0e0" /> <img src="https://img.shields.io/badge/surrealdb-red?style=for-the-badge&label=database&color=%23e0e0e0" />

### 👀 What is this?
This is a sample for REST API in Rust 🦀 <br />
It provides creating, editing and fetching tasks from database.

### ✈️ How to run this?
1. Install SurrealDB from [official site](https://surrealdb.com/) and run it
2. Download this code and edit `.env` file:
```
DB_URL=127.0.0.1:8000
DB_USERNAME=root
DB_PASSWORD=toor
DB_NS=somenamespace
DB_DATABASE=mydb
```

> **DB_URL** - url adress to you'r database (on local machine it would be 127.0.0.1:8000) <br />
> **DB_USERNAME** - username for login (i've chosen 'root') <br />
> **DB_PASSWORD** - password for login <br />
> **DB_NS** - namespace (edit as you wish) <br />
> **DB_DATABASE** - database name (it will keep all you'r content) <br />

3. Run project by:
```
cargo run --release
```
4. Open it in browser: [127.0.0.1](http://127.0.0.1:8080)

### 🦛 Routes

| Request Type | Route Adress | Description |
| ------------ | ------------ | ----------- |
| <img src="https://img.shields.io/badge/get-green?style=for-the-badge" /> | / | Index Page |
| <img src="https://img.shields.io/badge/get-green?style=for-the-badge" /> | /tasks | All tasks |
| <img src="https://img.shields.io/badge/get-green?style=for-the-badge" /> | /tasks/\<id> | Get task by id |
| <img src="https://img.shields.io/badge/post-orange?style=for-the-badge" /> | /tasks/new | Create new task (JSON) |
| <img src="https://img.shields.io/badge/post-orange?style=for-the-badge" /> | /tasks/\<id>/edit | Edit task by id (JSON) |
| <img src="https://img.shields.io/badge/post-orange?style=for-the-badge" /> | /tasks/\<id>/finish | Finish task by id (JSON) |
| <img src="https://img.shields.io/badge/post-orange?style=for-the-badge" /> | /tasks/\<id>/definish | Unfinish task by id (JSON) |
| <img src="https://img.shields.io/badge/delete-red?style=for-the-badge" /> | /tasks/\<id> | Delete task by id |

> [!IMPORTANT]
> To create/edit tasks send JSON data like this:
> ```json
> {
>   "title": "TASK TITLE"
>   "description": "TASK DESCRIPTION"
> }
> ```

### 🦈 License
Project is licensed under the MIT License. <br />
Check the [LICENSE](https://github.com/mealet/todo_api.rs/blob/master/LICENSE) file for more.
