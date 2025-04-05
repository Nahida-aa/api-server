package com.example.demo.controller;

import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.HashMap;
import java.util.Map;

@RestController
@RequestMapping("/api")
public class AuthController {

    // 模拟用户存储
    private final Map<String, String> users = new HashMap<>() {{
        put("admin", "admin123");
        put("aa", "aa");
    }};

    @PostMapping("/login")
    public ResponseEntity<?> login(@RequestBody Map<String, String> request) {
        String username = request.get("username");
        String password = request.get("password");

        if (!users.containsKey(username)) {
            return ResponseEntity.status(HttpStatus.NOT_FOUND).body(Map.of("error", "用户不存在"));
        }

        if (!users.get(username).equals(password)) {
            return ResponseEntity.status(HttpStatus.UNAUTHORIZED).body(Map.of("error", "密码或用户名错误"));
        }

        // 模拟生成 token
        String token = "mock-token";
        int expires = 3600;

        return ResponseEntity.ok(Map.of("token", token, "expires", expires));
    }

    @PostMapping("/sginup")
    public ResponseEntity<?> signup(@RequestBody Map<String, String> request) {
        String username = request.get("username");
        String password = request.get("password");

        if (users.containsKey(username)) {
            return ResponseEntity.status(HttpStatus.CONFLICT).body(Map.of("error", "用户名已存在"));
        }

        users.put(username, password);

        // 模拟生成 token
        String token = "mock-token";
        int expires = 3600;

        return ResponseEntity.ok(Map.of("token", token, "expires", expires));
    }
}