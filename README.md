# 基于ChainMaker的java-sdk实现区块链服务
## 部署ChainMaker集群
https://docs.chainmaker.org.cn/tutorial/%E5%BF%AB%E9%80%9F%E5%85%A5%E9%97%A8.html#id5

## 生成java-sdk的jar包
https://docs.chainmaker.org.cn/v2.0.0/html/dev/SDK.html#id13

## 项目结构
基于spring-boot的mvn项目

![001.png](https://github.com/hongfish/fish1208-chainmaker-javasdk/blob/main/src/main/resources/image/001.png)
* crypto-config   ca证书、user证书

![002.png](https://github.com/hongfish/fish1208-chainmaker-javasdk/blob/main/src/main/resources/image/002.png)
* contract 应用合约，sol、bin文件

![003.png](https://github.com/hongfish/fish1208-chainmaker-javasdk/blob/main/src/main/resources/image/003.png)
* application-dev.yml
```
sdk-config:
  config-path: classpath:sdk_config.yml  #区块链集群的配置文件
```

* sdk_config.yml 
```
chain_client:
  # 链ID
  chain_id: "chain1"
  # 组织ID
  org_id: "wx-org1.chainmaker.org"
  # 客户端用户私钥路径
  user_key_file_path: "crypto-config/wx-org1.chainmaker.org/user/admin1/admin1.tls.key"
  # 客户端用户证书路径
  user_crt_file_path: "crypto-config/wx-org1.chainmaker.org/user/admin1/admin1.tls.crt"
  # 客户端用户交易签名私钥路径
  user_sign_key_file_path: "crypto-config/wx-org1.chainmaker.org/user/admin1/admin1.sign.key"
  # 客户端用户交易签名证书路径
  user_sign_crt_file_path: "crypto-config/wx-org1.chainmaker.org/user/admin1/admin1.sign.crt"

  nodes:
    - # 节点地址，格式为：IP:端口:连接数
      node_addr: "192.168.160.159:12301"
      # 节点连接数
      conn_cnt: 10
      # RPC连接是否启用双向TLS认证
      enable_tls: true
      # 信任证书池路径
      trust_root_paths:
        - "crypto-config/wx-org1.chainmaker.org/ca"
#        - "crypto-config/wx-org2.chainmaker.org/ca"
      # TLS hostname
      tls_host_name: "chainmaker.org"
#    - # 节点地址，格式为：IP:端口:连接数
#      node_addr: "127.0.0.1:12302"
#      # 节点连接数
#      conn_cnt: 10
#      # RPC连接是否启用双向TLS认证
#      enable_tls: true
#      # 信任证书池路径
#      trust_root_paths:
#        - "crypto-config/wx-org1.chainmaker.org/ca"
#        - "crypto-config/wx-org2.chainmaker.org/ca"
#      # TLS hostname
#      tls_host_name: "chainmaker.org"
#  archive:
#    # 数据归档链外存储相关配置
#    type: "mysql"
#    dest: "root:123456:localhost:3306"
#    secret_key: xxx

  rpc_client:
    # grpc客户端最大接受容量(MB)
    max_receive_message_size: 16
```

* pom.xml
引用chainmaker-javasdk的2.0.0版本

![004.png](https://github.com/hongfish/fish1208-chainmaker-javasdk/blob/main/src/main/resources/image/004.png)

## 拷贝javasdk的jar包
### javasdk的jar包拷贝到项目src/main/resources/lib目录下
jar文件在sdk-java/build/libs目录下
![008.png](https://github.com/hongfish/fish1208-chainmaker-javasdk/blob/main/src/main/resources/image/008.png)

## 拷贝证书
### 集群的证书文件夹crypto-config拷贝到项目根目录下
证书文件在chainmaker-go/build目录下
![005.png](https://github.com/hongfish/fish1208-chainmaker-javasdk/blob/main/src/main/resources/image/005.png)

## 代码开发
### ChainSDKConfig.java
通过sdk-config.yml配置文件获取ChainClient、User对象

### ChainController.java
根据区块高度来获取区块信息。

## 项目启动
![006.png](https://github.com/hongfish/fish1208-chainmaker-javasdk/blob/main/src/main/resources/image/006.png)

## 调用接口
### 根据区块高度来获取区块信息
####传参blockHeight=0，返回创始区块的区块时间
http://127.0.0.1:7022/chain/getBlockInfo?blockHeight=0

**请求**
```$xslt
GET /contract/person/get HTTP/1.1  
blockHeight=0
```
![007.png](https://github.com/hongfish/fish1208-chainmaker-javasdk/blob/main/src/main/resources/image/007.png)

## Github地址
https://github.com/hongfish/fish1208-chainmaker-javasdk
