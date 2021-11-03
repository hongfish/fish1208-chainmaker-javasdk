package com.fish1208.chainmaker.config;

import org.apache.commons.io.IOUtils;
import org.chainmaker.sdk.config.NodeConfig;
import org.chainmaker.sdk.config.SdkConfig;
import org.chainmaker.sdk.crypto.ChainMakerCryptoSuiteException;
import org.chainmaker.sdk.utils.FileUtils;
import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.chainmaker.sdk.*;
import org.springframework.core.io.Resource;
import org.yaml.snakeyaml.Yaml;

import java.io.BufferedInputStream;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.util.ArrayList;
import java.util.List;

@Configuration
@ConfigurationProperties(prefix = "sdk-config")
public class ChainSDKConfig {

    private static String ORG_ID1 = "wx-org1.chainmaker.org";
    private static String ORG_ID2 = "wx-org2.chainmaker.org";
    private static String ORG_ID3 = "wx-org3.chainmaker.org";

    private static String ADMIN1_TLS_KEY_PATH = "crypto-config/wx-org1.chainmaker.org/user/admin1/admin1.tls.key";
    private static String ADMIN1_TLS_CERT_PATH = "crypto-config/wx-org1.chainmaker.org/user/admin1/admin1.tls.crt";
    private static String ADMIN2_TLS_KEY_PATH = "crypto-config/wx-org2.chainmaker.org/user/admin1/admin1.tls.key";
    private static String ADMIN2_TLS_CERT_PATH = "crypto-config/wx-org2.chainmaker.org/user/admin1/admin1.tls.crt";
    private static String ADMIN3_TLS_KEY_PATH = "crypto-config/wx-org3.chainmaker.org/user/admin1/admin1.tls.key";
    private static String ADMIN3_TLS_CERT_PATH = "crypto-config/wx-org3.chainmaker.org/user/admin1/admin1.tls.crt";

    private static String ADMIN1_KEY_PATH = "crypto-config/wx-org1.chainmaker.org/user/admin1/admin1.sign.key";
    private static String ADMIN1_CERT_PATH = "crypto-config/wx-org1.chainmaker.org/user/admin1/admin1.sign.crt";
    private static String ADMIN2_KEY_PATH = "crypto-config/wx-org2.chainmaker.org/user/admin1/admin1.sign.key";
    private static String ADMIN2_CERT_PATH = "crypto-config/wx-org2.chainmaker.org/user/admin1/admin1.sign.crt";
    private static String ADMIN3_KEY_PATH = "crypto-config/wx-org3.chainmaker.org/user/admin1/admin1.sign.key";
    private static String ADMIN3_CERT_PATH = "crypto-config/wx-org3.chainmaker.org/user/admin1/admin1.sign.crt";

    private Resource configPath;

    @Bean
    public ChainClient getChainClient() throws IOException, SdkException {

        SdkConfig sdkConfig = getSdkConfig();
        for (NodeConfig nodeConfig : sdkConfig.getChain_client().getNodes()) {
            List<byte[]> tlsCaCertList = new ArrayList<>();
            for (String rootPath : nodeConfig.getTrustRootPaths()){
                List<String> filePathList = FileUtils.getFilesByPath(rootPath);
                for (String filePath : filePathList) {
                    tlsCaCertList.add(FileUtils.getFileBytes(filePath));
                }
            }
            byte[][] tlsCaCerts = new byte[tlsCaCertList.size()][];
            tlsCaCertList.toArray(tlsCaCerts);
            nodeConfig.setTrustRootBytes(tlsCaCerts);
        }

        ChainManager chainManager = ChainManager.getInstance();
        ChainClient chainClient = chainManager.getChainClient(sdkConfig.getChain_client().getChainId());

        if (chainClient == null) {
            chainClient = chainManager.createChainClient(sdkConfig);
        }
        return chainClient;
    }

    @Bean("adminUser1")
    public User getUser1() throws IOException, SdkException {
        User adminUser1 = new User(ORG_ID1, IOUtils.toByteArray(new FileInputStream(ADMIN1_KEY_PATH)),
                IOUtils.toByteArray(new FileInputStream(ADMIN1_CERT_PATH)),
                IOUtils.toByteArray(new FileInputStream(ADMIN1_TLS_KEY_PATH)),
                IOUtils.toByteArray(new FileInputStream(ADMIN1_TLS_CERT_PATH)));
        return adminUser1;
    }

    @Bean("adminUser2")
    public User getUser2() throws IOException, SdkException {
        User adminUser2 = new User(ORG_ID2, IOUtils.toByteArray(new FileInputStream(ADMIN2_KEY_PATH)),
                IOUtils.toByteArray(new FileInputStream(ADMIN2_CERT_PATH)),
                IOUtils.toByteArray(new FileInputStream(ADMIN2_TLS_KEY_PATH)),
                IOUtils.toByteArray(new FileInputStream(ADMIN2_TLS_CERT_PATH)));
        return adminUser2;
    }

    @Bean("adminUser3")
    public User getUser3() throws IOException, SdkException {
        User adminUser3 = new User(ORG_ID3, IOUtils.toByteArray(new FileInputStream(ADMIN3_KEY_PATH)),
                IOUtils.toByteArray(new FileInputStream(ADMIN3_CERT_PATH)),
                IOUtils.toByteArray(new FileInputStream(ADMIN3_TLS_KEY_PATH)),
                IOUtils.toByteArray(new FileInputStream(ADMIN3_TLS_CERT_PATH)));
        return adminUser3;
    }

    private SdkConfig getSdkConfig() throws IOException{
        Yaml yaml = new Yaml();
        InputStream in = configPath.getInputStream();
        SdkConfig sdkConfig;
        try{
            sdkConfig = yaml.loadAs(in, SdkConfig.class);
        } finally {
            in.close();
        }
        return sdkConfig;
    }

    public Resource getConfigPath() {
        return configPath;
    }

    public void setConfigPath(Resource configPath) {
        this.configPath = configPath;
    }

}
