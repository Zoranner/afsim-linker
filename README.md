# AFSim 链接器

通过 URL Scheme 从网页拉起应用执行 AFSim 推演脚本。

## 功能特性

- 通过 URL 调用自动执行 AFSim 推演脚本
- 支持配置 AFSim 可执行程序路径和脚本基础路径
- 图形化配置界面，支持路径浏览选择
- 自动注册 `afsim://` URL 协议
- URL 模式下显示加载界面，脚本启动后自动退出

## 使用说明

### 首次配置

启动程序后，在界面中配置：

- **可执行程序路径**: AFSim 的 exe 文件路径，例如 `C:\AFSim\afsim.exe`
- **脚本基础路径**: 存放推演场景的根目录，例如 `D:\projects\scenarios`
- 点击 **💾 保存配置**

### URL 调用格式

```
afsim://run?scene=场景名称&script=脚本入口
```

**示例：**

```
afsim://run?scene=test-scenario&script=setup.txt
```

**实际执行的命令：**

```
C:\AFSim\afsim.exe D:\projects\scenarios\test-scenario\setup.txt
```

### 在网页中使用

```html
<a href="afsim://run?scene=test-scenario&script=setup.txt">执行推演脚本</a>
```

**说明：**
- 通过 URL 拉起时，程序会显示加载界面
- 脚本启动成功后，程序自动退出
- 如果启动失败，程序会显示错误信息，不会自动退出

### URL Scheme 注册地址

首次启动会自动注册 URL 协议：

- **Windows**: `HKEY_CURRENT_USER\Software\Classes\afsim`
- **Linux**: `~/.local/share/applications/afsim.desktop`

### 5. 配置文件位置

配置自动保存在：

- **Windows**: `%APPDATA%\afsim-linker\config.json`
- **Linux**: `~/.config/afsim-linker/config.json`

## 构建

```bash
# 开发模式
cargo run

# 发布模式
cargo build --release
```

## 命令行参数

```bash
# 卸载 URL Scheme
cargo run -- --unregister

# 检查注册状态
cargo run -- --check
```

## 技术栈

- Rust + egui (GUI 框架)
- rfd (文件选择对话框)
- serde (配置文件序列化)