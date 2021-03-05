# openvpn-auth-user-pass
A tool for 'Windows OpenVPN Server' verify the username and password provided by the client.
一个Windows版OpenVPN Server验证用户名和密码的工具。

## 1、发布此工具

编译后只需要一个文件：openvpn-auth-user-pass.exe，将此文件放到config目录。

同时在config目录下新建一个文件：psw.txt，用于保存用户名和密码，每行一个，格式：username,password

## 2、修改OpenVPN Server配置文件

增加这几行：

```shell
script-security 3
auth-user-pass-verify openvpn-auth-user-pass.exe via-env
client-cert-not-required
username-as-common-name
```

## 3、修改OpenVPN Client配置文件

增加这一行：

```shell
auth-user-pass
```

同时注释掉这两行：

```shell
cert xxxx.crt
key xxx.key
```

## 4、重启OpenVPN Server和Client

先在OpenVPN Server执行断开连接，然后重新连接。

然后在OpenVPN Client执行断开连接，然后重新连接。

最后连接成功。
