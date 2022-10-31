# Xbox SpeedTest

## [解决 Xbox 在国内下载速度慢的问题](https://baiyun.me/fix-slow-xbox-download-speed)

## 使用方法

到 [Release](https://github.com/liby/xbox-speed-test/releases/latest) 页面下载最新的二进制文件并给予执行权限：
```
curl -fLO https://github.com/liby/xbox-speed-test/releases/latest/download/xst
chmod +x xst
```

在下载的目标文件夹中运行此命令来将脚本临时添加进环境变量中：
```shell
export PATH="$PATH:`pwd`"
```

### 运行测试

```shell
xst
```

测试运行中：
![测试中](https://i.imgur.com/xNTnxkF.png)

测试结束后会输出各节点的速度排序表：

![下载速度排序](https://i.imgur.com/pKRmk5Z.png)

## 本地开发

确保你本地已经设置了正确的 Rust 开发环境

### 克隆仓库

```shell
git@github.com:liby/xbox-speed-test.git
cd xbox-speed-test
```

### 调试

```shell
cargo run --
```

### 构建

```shell
cargo build --release
```

## 感谢

- [wmltogether/XboxSpeedTest](https://github.com/wmltogether/XboxSpeedTest)
- [isbasex/xbox-speed-test](https://github.com/isbasex/xbox-speed-test)