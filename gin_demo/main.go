package main

import (
	"log"
	"net/http"

	"github.com/gin-gonic/gin"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"gin_demo/apps/note"
    swaggerFiles "github.com/swaggo/files"
    ginSwagger "github.com/swaggo/gin-swagger"
)

var db = make(map[string]string)

func setupRouter() *gin.Engine {
	// Disable Console Color
	// gin.DisableConsoleColor()
	r := gin.Default()

	// Ping test
	r.GET("/ping", func(c *gin.Context) {
		c.String(http.StatusOK, "pong")
	})

	// Get user value
	r.GET("/user/:name", func(c *gin.Context) {
		user := c.Params.ByName("name")
		value, ok := db[user]
		if ok {
			c.JSON(http.StatusOK, gin.H{"user": user, "value": value})
		} else {
			c.JSON(http.StatusOK, gin.H{"user": user, "status": "no value"})
		}
	})

	// Authorized group (uses gin.BasicAuth() middleware)
	// Same than:
	// authorized := r.Group("/")
	// authorized.Use(gin.BasicAuth(gin.Credentials{
	//	  "foo":  "bar",
	//	  "manu": "123",
	//}))
	authorized := r.Group("/", gin.BasicAuth(gin.Accounts{
		"foo":  "bar", // user:foo password:bar
		"manu": "123", // user:manu password:123
	}))

	/* example curl for /admin with basicauth header
	Zm9vOmJhcg== is base64("foo:bar")

	curl -X POST \
	http://localhost:8080/admin \
	-H 'authorization: Basic Zm9vOmJhcg==' \
	-H 'content-type: application/json' \
	-d '{"value":"bar"}'
	*/
	authorized.POST("admin", func(c *gin.Context) {
		user := c.MustGet(gin.AuthUserKey).(string)

		// Parse JSON
		var json struct {
			Value string `json:"value" binding:"required"`
		}

		if c.Bind(&json) == nil {
			db[user] = json.Value
			c.JSON(http.StatusOK, gin.H{"status": "ok"})
		}
	})

	return r
}

// @title Notes API
// @version 1.0
// @description 笔记 CRUD 示例 API
// @host localhost:8080
// @BasePath /
func main() {
	db, err := gorm.Open(postgres.Open("postgres://aa@localhost:5432/web_demo"), &gorm.Config{})
	if err != nil {
		log.Fatal("failed to connect database:", err)
	}

	r := setupRouter()
	// Swagger
    r.GET("/swagger/*any", ginSwagger.WrapHandler(swaggerFiles.Handler))
	// 注册 note 路由
    service := &note.NoteService{DB: db}
    note.RegisterNoteRoutes(r, service)
	// Listen and Server in 0.0.0.0:8080  http://localhost:8080 
	r.Run(":8080")
}