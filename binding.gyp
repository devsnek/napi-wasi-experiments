{
  'make_global_settings': [
    {
      'CXX': '/home/snek/Desktop/tools/wasi-sdk/build/install/opt/wasi-sdk/bin/clang++',
      'CC': '/home/snek/Desktop/tools/wasi-sdk/build/install/opt/wasi-sdk/bin/clang',
    },
  ],
  'targets': [
    {
      'target_name': 'wasi_test',
      'sources': [
        './src/test.cc',
      ],
      'include_dirs': [
        '<!@(node -p \'require("node-addon-api").include\')',
      ],
      'defines': [
        'NAPI_DISABLE_CPP_EXCEPTIONS',
      ],
    },
  ],
}
