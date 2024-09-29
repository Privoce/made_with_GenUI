# Errors and Defects

- GInput: 缺少read_only模式 ✅
- GTable need to be virtual table!!! ✅
- GUpload 需要可以发送文件夹地址 ✅

- GDropDown 会阻碍事件冒泡
- GLink重新构建，目标为GButton
- 缺少selector ✅
- 缺少radio_group ✅

- start page 聚焦按钮 ✅
- GInput 光标初始不显示 

## 功能

1. 校验登录
2. 连接 s3 Bucket
3. 查看 aws 配置
4. 修改 aws 配置
5. 查看 s3 上的文件
6. 选择自己系统上的文件夹作为上传列表
7. 上传文件到s3
8. 从s3上下载已有的文件
9. 从s3上删除文件
10. 分享
11. 提示，准备工作

## 页面

1. loading page
2. sigin page (optional)
   1. 校验登录 ✅
   2. 查看 aws 配置 ✅
   3. 连接 s3 Bucket ✅
   4. 提示，准备工作 ✅
3. main page 
   1. 查看 s3 上的文件 ✅
   2. 选择自己系统上的文件夹作为上传列表 ✅
   3. 上传文件到s3 (异步或多线程处理, 多线程更好)
   4. 从s3上下载已有的文件 (异步或多线程处理, 多线程更好)
   5. 从s3上删除文件 ✅
   6. 分享(右上角图标分享) ✅
4. settings page
   1. 查看 aws 配置 ✅
   2. 修改 aws 配置 ✅
   3. 提示，准备工作 ✅

- 抽屉 ✅ (根据内部元素控制展开高度)
- 单文件上传
- 用户选择上传文件夹
- 手机Api调用
- 生成文件夹(虚拟)
- 生成文件(生成完就已经在云盘里)

- 对bucket处理（只处理bucket）
- 分享文件URL，提供其他人下载
- 文件大小修改
- 去除刷新按钮改为轮询刷新
- loading中自动连接获取配置跳转到Bucket首页
- 英文
- lazy_static 会导致页面速度变慢，尝试传输

file_editor


是的，AWS S3 支持通过预签名 URL 分享文件，其他人可以通过该链接直接下载文件，而不需要 AWS 账户或权限。这种方式生成的 URL 有一个有效期，过期后链接将失效。

### 如何生成预签名 URL

可以通过 AWS CLI 或 SDK 生成预签名 URL。以下是通过 AWS CLI 和 Rust SDK 两种方式生成预签名 URL 的示例。

### 1. **使用 AWS CLI 生成预签名 URL**
运行以下命令来生成一个预签名 URL，该 URL 有效期为 1 小时（可以通过 `--expires-in` 参数自定义有效期，以秒为单位）：

```bash
aws s3 presign s3://your-bucket-name/your-file-name --expires-in 3600
```

解释：
- `s3://your-bucket-name/your-file-name`：是文件所在的 S3 存储桶路径。
- `--expires-in`：指定链接有效期（3600 秒 = 1 小时）。

命令运行成功后，终端会输出一个预签名的 URL，其他人可以通过这个 URL 下载文件。

### 2. **使用 Rust SDK 生成预签名 URL**

如果你想在 Rust 中生成预签名 URL，可以使用 `aws-sdk-s3`。以下是一个示例：

首先在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
aws-config = "0.5"
aws-sdk-s3 = "0.5"
tokio = { version = "1", features = ["full"] }
```

然后编写代码生成预签名 URL：

```rust
use aws_sdk_s3::{Client, Region};
use aws_sdk_s3::model::BucketLocationConstraint;
use aws_sdk_s3::presigning::config::PresigningConfig;
use aws_config::meta::region::RegionProviderChain;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), aws_sdk_s3::Error> {
    let region_provider = RegionProviderChain::default_provider().or_else(Region::new("us-east-1"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let bucket_name = "your-bucket-name";
    let object_key = "your-file-name";

    let presigning_config = PresigningConfig::expires_in(Duration::from_secs(3600))?; // 有效期为 1 小时
    let presigned_req = client.get_object()
        .bucket(bucket_name)
        .key(object_key)
        .presigned(presigning_config)
        .await?;

    let url = presigned_req.uri().to_string();
    println!("Presigned URL: {}", url);

    Ok(())
}
```

### 解释：
- `PresigningConfig::expires_in(Duration::from_secs(3600))`：设置预签名链接的有效期为 3600 秒（1 小时）。
- `client.get_object()`：指定要生成预签名 URL 的对象。
- `presigned()`：生成预签名的请求，返回一个包含 URL 的请求。

生成的 URL 可以通过浏览器或下载工具直接访问和下载文件。

### 注意事项：
- 生成的预签名 URL 有效期有限，过期后需要重新生成。
- 预签名 URL 只允许访问指定的 S3 对象（文件），其他文件依然受到权限保护。