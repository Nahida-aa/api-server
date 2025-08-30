import { drizzle } from "drizzle-orm/node-postgres";
import { Pool } from "pg";
import * as schema from "./schema/index";

// 数据库连接池
const pool = new Pool({
  connectionString: process.env.DATABASE_URL || "postgresql://localhost:5432/chat_db",
  max: 10,
  idleTimeoutMillis: 30000,
  connectionTimeoutMillis: 2000,
});

// Drizzle 数据库实例
export const db = drizzle(pool, { schema });

// 数据库连接测试
export async function testConnection() {
  try {
    const client = await pool.connect();
    await client.query('SELECT NOW()');
    client.release();
    console.log('✅ 数据库连接成功');
    return true;
  } catch (error) {
    console.error('❌ 数据库连接失败:', error);
    return false;
  }
}

// 优雅关闭数据库连接
export async function closeConnection() {
  await pool.end();
  console.log('🔌 数据库连接已关闭');
}