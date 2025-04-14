# Global WIL

# how to check assembly code compiled from rust

```shell
cargo rustc --bin <project_name> -- --emit asm
```
위 명령어 입력시 컴파일된 어셈블리코드 확인가능하다.