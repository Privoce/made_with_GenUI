# GenUI S3 Cloud Drive

A simple AWS S3 cloud storage client written using Rust Makepad framework

## Prepare

For installation instructions, expand the section for your operating system.

### Download AWS Cli

[AWS Cli](https://docs.aws.amazon.com/zh_cn/cli/latest/userguide/getting-started-install.html)

#### Windows

To update your current installation of AWS CLI on Windows, download a new installer each time you update to overwrite previous versions. AWS CLI is updated regularly. To see when the latest version was released, see the AWS CLI version 2 Changelog on GitHub. 

Download and run the AWS CLI MSI installer for Windows (64-bit):[https://awscli.amazonaws.com/AWSCLIV2.msi](https://awscli.amazonaws.com/AWSCLIV2.msi)

Alternatively, you can run the msiexec command to run the MSI installer.

```shell
msiexec.exe /i https://awscli.amazonaws.com/AWSCLIV2.msi
# For various parameters that can be used with msiexec, see msiexec on the Microsoft Docs website. 
# For example, you can use the /qn flag for a silent installation.
msiexec.exe /i https://awscli.amazonaws.com/AWSCLIV2.msi /qn
```


#### Mac

If you have sudo permissions, you can install the AWS CLI for all users on the computer. We provide the steps in one easy to copy and paste group. See the descriptions of each line in the following steps. 

```shell
curl "https://awscli.amazonaws.com/AWSCLIV2.pkg" -o "AWSCLIV2.pkg"
sudo installer -pkg AWSCLIV2.pkg -target /
```

#### Linux (x86)

If this is your first time updating on Amazon Linux, to install the latest version of the AWS CLI, you must uninstall the pre-installed yum version using the following command:

```shell
sudo yum remove awscli
```

```shell
curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
unzip awscliv2.zip
sudo ./aws/install
```

### Config

After completing the download, you need to log in to AWS and obtain the assigned account for configuration
See [config the AWS Cli](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html)

```shell
aws configure

AWS Access Key ID [None]: YOUR ACCESS KEY ID
AWS Secret Access Key [None]: YOUR SECRET ACCESS KEY
Default region name [None]: REGION NAME
Default output format [None]: json
```
#### Regions

- 美国东部(俄亥俄) us-east-2
- 美国东部(弗吉尼亚州北部) us-east-1
- 美国西部(加利福尼亚州北部) us-west-1
- 美国西部(俄勒冈) us-west-2
- 亚太地区(首尔) ap-northeast-2
- 亚太地区(东京) ap-northeast-1
- 亚太地区(大阪) ap-northeast-3
- 亚太地区(香港) ap-east-1
- 亚太地区(孟买) ap-south-1
- 亚太地区(新加坡) ap-southeast-1
- 亚太地区(悉尼) ap-southeast-2
- 加拿大(中部) ca-central-1
- 中国(北京) cn-north-1
- 中国(宁夏) cn-northwest-1
- 欧洲(法兰克福) eu-central-1
- 欧洲(爱尔兰) eu-west-1
- 欧洲(伦敦) eu-west-2
- 欧洲(巴黎) eu-west-3
- 欧洲(斯德哥尔摩) eu-north-1
- 中东(巴林) me-south-1
- 南美洲(圣保罗) sa-east-1
- 非洲(开普敦) af-south-1
- AWS GovCloud(美国东部) us-gov-east-1
- AWS GovCloud(美国西部) us-gov-west-1

## features

- [x] Check S3 Config
- [x] Connect S3
- [x] Get Files, Folders from S3
- [x] Upload Files to S3
- [x] Delete Files, Folders from S3
- [x] Share Files and Folders
- [ ] Upload Folders
- [ ] Create Files, Folder
- [ ] Use AWS SDK
- [ ] Use AWS Cli