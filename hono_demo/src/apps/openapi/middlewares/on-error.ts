import type { ErrorHandler } from "hono";
// import { HTTPResponseError } from "hono/types";
import type { ClientErrorStatusCode, InfoStatusCode, RedirectStatusCode, ServerErrorStatusCode, StatusCode, UnofficialStatusCode } from "hono/utils/http-status";
import { INTERNAL_SERVER_ERROR, OK } from "@/server/modules/openapi/http-status-codes";
import { AppEnv, AppOpenAPI } from "@/server/types";

export interface HTTPResponseError extends Error {
  getResponse: () => Response;
  status: StatusCode | InfoStatusCode | RedirectStatusCode | ClientErrorStatusCode | ServerErrorStatusCode | UnofficialStatusCode;
}

/**
 * 自定义HTTP错误类，简化错误处理
 * @example
 * throw new HttpError(404, '项目未找到');
 * throw new HttpError(403, '无权限操作此项目');
 */
export class HttpError extends Error implements HTTPResponseError {
  public status: StatusCode | InfoStatusCode | RedirectStatusCode | ClientErrorStatusCode | ServerErrorStatusCode | UnofficialStatusCode;

  constructor(
    status: StatusCode | InfoStatusCode | RedirectStatusCode | ClientErrorStatusCode | ServerErrorStatusCode | UnofficialStatusCode,
    message: string
  ) {
    super(message);
    this.name = 'HttpError';
    this.status = status;
    
    // 确保 instanceof 正常工作
    Object.setPrototypeOf(this, HttpError.prototype);
  }

  getResponse(): Response {
    return new Response(JSON.stringify({ message: this.message }), {
      status: this.status,
      headers: { 'Content-Type': 'application/json' }
    });
  }
}
// 
const onError: ErrorHandler<AppEnv> = (err: Error|HTTPResponseError, c) => {
  const currentStatus = "status" in err
    ? err.status
    : c.newResponse(null).status;
  const statusCode = currentStatus !== OK
    ? (currentStatus as UnofficialStatusCode|ClientErrorStatusCode | ServerErrorStatusCode)
    : INTERNAL_SERVER_ERROR;
  c.env
  // const env = c.env?.NODE_ENV || process.env?.NODE_ENV;
  const env = process.env?.NODE_ENV;
  c.var.logger.debug({ path: c.req.path,  message: err.message, stack: err.stack, status: statusCode });
  return c.json(
    {
      message: err.message,
      stack: env === "production" ? undefined : err.stack,
    },
    statusCode,
  );
};

export default onError;