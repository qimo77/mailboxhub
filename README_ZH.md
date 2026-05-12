# mailboxhub

mailboxhub 是一个 macOS 桌面端 Outlook 邮箱批量管理客户端。它可以批量导入 Outlook 账号，通过 IMAP 同步收件箱邮件，提取可能的验证码，并把账号、邮件和设置保存在本地 SQLite 数据库中。

项目基于 Tauri 2、Rust、React、TypeScript、Tailwind CSS 和 SQLite 构建。

## 功能特性

- 按 `email----password----client_id----refresh_token` 格式批量导入 Outlook 账号。
- 支持粘贴账号列表，也支持把 `.txt` 文件拖入导入页面。
- 使用本地 SQLite 保存账号、设置、邮件摘要、邮件正文、未读状态和提取出的验证码。
- 使用 OAuth refresh token 和 XOAUTH2 连接 Outlook IMAP。
- 后台自动轮询，默认 5 秒刷新一次。
- 支持手动刷新当前选中的邮箱。
- 账号列表中可以一键复制邮箱地址。
- 支持把当前邮箱全部标记为已读，也支持一次性把所有已导入邮箱全部标记为已读。
- 邮件预览支持 MIME 解析、quoted-printable 解码、HTML 清理和正文兜底提取。
- 上下文感知的 4-8 位验证码提取，避免把页脚、地址、追踪参数、CSS 数字误识别为验证码。
- 支持 macOS 新邮件通知。
- 默认浅色主题，可配置刷新间隔、通知和启动自动连接。
- 支持构建 macOS `.app` 和 `.dmg` 安装包。

## 下载

`0.1.0` 版本的 macOS Apple Silicon DMG 已发布到 GitHub Releases：

- `mailboxhub_0.1.0_aarch64.dmg`

## 账号导入格式

每一行代表一个账号：

```text
email@outlook.com----password----client_id----refresh_token
```

其中 password 字段用于兼容已有导出格式。实际 IMAP 登录使用 OAuth client ID 和 refresh token。

## 开发环境要求

- macOS
- Node.js 和 npm
- Rust 和 Cargo
- Tauri 2 的 macOS 开发环境依赖

安装前端依赖：

```bash
npm install
```

启动桌面端开发模式：

```bash
npm run tauri:dev
```

只启动 Vite 前端：

```bash
npm run dev
```

## 测试和验证

运行前端测试：

```bash
npm test
```

运行前端构建：

```bash
npm run build
```

运行 Rust 测试和检查：

```bash
cargo test --manifest-path src-tauri/Cargo.toml
cargo check --manifest-path src-tauri/Cargo.toml
```

构建生产版 macOS 包：

```bash
npm run tauri:build
```

构建产物位置：

```text
src-tauri/target/release/bundle/macos/mailboxhub.app
src-tauri/target/release/bundle/dmg/mailboxhub_0.1.0_aarch64.dmg
```

## 数据和隐私

mailboxhub 会把账号记录、设置和同步到的邮件数据保存在本机应用数据目录下的 SQLite 数据库中。应用会从你的电脑直接连接 Outlook IMAP 和 Microsoft OAuth 服务。不要把本地数据库、refresh token、`.env` 文件或账号导入文件提交到源码仓库。

## 技术栈

- Tauri 2
- Rust
- React 19
- TypeScript
- Tailwind CSS
- SQLite / sqlx
- Zustand
- Vitest

## 许可证

当前项目还没有添加 License 文件。如果要公开分发或接受外部贡献，建议先补充许可证。
