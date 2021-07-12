#include <string>
#include <fstream>
#include <streambuf>
#include <cassert>

#include <napi.h>

Napi::Value Call(const Napi::CallbackInfo& info) {
  Napi::Function f = info[0].As<Napi::Function>();
  return f({});
}

Napi::Value ReadFile(const Napi::CallbackInfo& info) {
  std::ifstream t("/sandbox/hello.txt");
  assert(t);
  std::string str((std::istreambuf_iterator<char>(t)),
                   std::istreambuf_iterator<char>());
  return Napi::String::New(info.Env(), str);
}

Napi::Object Init(Napi::Env env, Napi::Object exports) {
  exports["call"] = Napi::Function::New(env, Call);
  exports["readFile"] = Napi::Function::New(env, ReadFile);
  return exports;
}

NODE_API_MODULE(test, Init)
