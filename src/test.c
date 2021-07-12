#include <stdio.h>
#include <assert.h>
#include <stdlib.h>
#include <node_api.h>

static napi_value call(napi_env env, napi_callback_info info) {
  size_t argc = 1;
  napi_value arg0;
  napi_get_cb_info(env, info, &argc, &arg0, NULL, NULL);
  napi_value result;
  napi_call_function(env, NULL, arg0, 0, NULL, &result);
  return result;
}

static napi_value read_file(napi_env env, napi_callback_info info) {
  FILE* f = fopen("/sandbox/hello.txt", "rb");
  assert(f);

  fseek(f, 0, SEEK_END);
  const long length = ftell(f);
  fseek(f, 0, SEEK_SET);

  char* buffer = (char*) malloc(length);
  assert(buffer);

  fread(buffer, 1, length, f);

  fclose(f);

  napi_value result;
  napi_create_string_utf8(env, buffer, length, &result);

  return result;
}

static napi_value init(napi_env env, napi_value exports) {
#define F(name, exp)                                  \
  do {                                                \
    napi_value f;                                     \
    napi_create_function(env, "", 0, name, NULL, &f); \
    napi_set_named_property(env, exports, exp, f);    \
  } while (false)

  F(call, "call");
  F(read_file, "readFile");

  return exports;
}
NAPI_MODULE(NODE_GYP_MODULE_NAME, init);
