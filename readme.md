# Excel Parser 项目

## 项目简介

这是一个用 Rust 编写的 Excel 解析器项目，能够解析 Excel 文件并根据模板生成 JSON 输出。

## 依赖

- Rust
- Cargo
- dotenv
- calamine

## 安装

1. 确保已安装 Rust 和 Cargo。
2. 克隆此仓库：
    ```sh
    git clone <仓库地址>
    cd <项目目录>
    ```
3. 安装依赖：
    ```sh
    cargo build
    ```

## 配置

在项目根目录下创建一个 `.env` 文件，并添加以下内容：

```env
EXCEL_SOURCE=./demo/test.xlsx
DEFAULT_SHEET=sheet1
JSON_TEMPLATE_PATH=./demo/template/template.json
OUT_PUT_FILE=./demo/
```

### 模板文件如下
+ 参考示例，./demo/template/template.json
  + ${列名:数据类型}
```json
[
   {
      "id": "${id:num}",
      "name": "${姓名}",
      "age": "${年龄:num}",
      "sub": [
         {
            "sub_name": "${sub_name}",
            "sub_age": "${sub_title}"
         }
      ]
   }
]
```

### 启动命令
  ```sh
   cargo run --bin parser_star
   ```