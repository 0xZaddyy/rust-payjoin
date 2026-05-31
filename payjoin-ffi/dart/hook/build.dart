import 'dart:io';

import 'package:hooks/hooks.dart';
import 'package:native_toolchain_rust/native_toolchain_rust.dart';

void main(List<String> args) async {
  await build(args, (input, output) async {
    final localCargoConfig = File.fromUri(
      input.packageRoot.resolve('.cargo/config.toml'),
    );

    await RustBuilder(
      assetName: 'uniffi:payjoin_ffi',
      features: localCargoConfig.existsSync() ? ['_test-utils'] : [],
    ).run(input: input, output: output);
  });
}
