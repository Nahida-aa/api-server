# 使用官方 OpenJDK 17 镜像作为基础镜像
FROM eclipse-temurin:17-jdk-alpine

# 设置工作目录
WORKDIR /app

# 将当前目录的内容复制到容器中
COPY . .

# 设置 JAVA_HOME 环境变量
ENV JAVA_HOME=/opt/java/openjdk
ENV PATH="$JAVA_HOME/bin:$PATH"

# 使用 Gradle 构建项目
RUN JAVA_HOME=/opt/java/openjdk ./gradlew build --no-daemon

# 暴露应用运行的端口（Spring Boot 默认端口为 8080）
EXPOSE 8080

# 运行 Spring Boot 应用
CMD ["java", "-jar", "build/libs/demo-0.0.1-SNAPSHOT.jar"]