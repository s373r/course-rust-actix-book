# Course: Actix

Course link: https://actix.rs/book/actix

Status: ✅*

(* - Some pages are under construction, need to come back a bit later)

### Index legend

- 📝 - a link to a book page
- ✏️ - a link to an `.rs` file (code)
- 👷 - a page under construction in the course
- 🚧 - not finished

## Index

- [✏️ 1. Getting Started](01_getting_started/src/main.rs)
- [✏️ 2. Actor](02_actor/src/main.rs)
- [✏️ 3. Address](03_address/src/main.rs)
- [✏️ 4. Context](04_context/src/main.rs)
- [✏️ 5. Arbiter](05_arbiter/src/main.rs)
- [✏️ 6. SyncArbiter](06_sync_arbiter/src/main.rs)
- 👷 7. Stream
- 👷 8. IO helpers
- 👷 9. Supervisor
- 👷 10. Registry
- 👷 11. Helper actors

## Notes

### Comments

- Some of my thoughts are prefixed with `NOTE:`
  - Example: `// NOTE: Algorithm complexity: O(n)`
- Resolved course TODOs are prefixed with `DONE:`
  - Example: `// DONE: ^ Uncomment the above 2 lines to see the compiler error`
- Other comments copied from the course
                                        
### A new chapter

> ℹ️ Cargo projects cannot be named leading from a digit

To create a new chapter-related subfolder, please use the following format: `cargo new N_name --name _N_name` 

#### Quick commands

> ℹ️ Update N and NAME variable values

Unix-like:
```shell
N=01; NAME=getting_started; cargo new "${N}_${NAME}" --name "_${N}_${NAME}"
```

Windows (Powershell):
```shell
$N='01'; $NAME='getting_started'; cargo new ${N}_${NAME} --name _${N}_${NAME}
```

## Code conduction

This project uses [Gitmoji](https://gitmoji.carloscuesta.me) for commit messages

## License

[GPLv3+](LICENSE)
