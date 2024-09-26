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

- 抽屉 ✅
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

file_editor