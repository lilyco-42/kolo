# kolo 
采用了逻辑和视图分离的结构
目的是适配不同ui，我不喜欢egui 启动就400mb的内存，
但快速开发验证，这也是必要的。
- core 负责逻辑实现，数据传输。
- gui 负责前端的动画呈现
## 快速开始
- 安装依赖
```
cargo install cargo-binstall
cargo binstall kolo

```
- 运行
```
kolo
```
