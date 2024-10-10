package com.chainbase.evm;

import com.esaulpaugh.headlong.util.FastHex;
import com.google.common.base.Strings;

public final class HexUtil {

  public static final String DEV_WALLET_ADDRESS = "0xfd99560c289f9FB3f7fbac1A4eB2226dF294BB91";

  public static final String DEFAULT_HOLE_ADDRESS = "0x0000000000000000000000000000000000000000";

  private static final char[] HEX_ARRAY = "0123456789ABCDEF".toCharArray();

  public static String to0xHex(String hex) {
    if (hex.startsWith("0x")) {
      return hex;
    } else {
      return "0x" + hex;
    }
  }

  public static byte[] decodeHex(String hex) {
    if (Strings.isNullOrEmpty(hex)) {
      return new byte[]{};
    }

    if (hex.startsWith("0x")) {
      return FastHex.decode(hex, 2, hex.length() - 2);
    } else {
      return FastHex.decode(hex);
    }
  }

  public static String toPureHex(String hex) {
    if (hex.startsWith("0x")) {
      return hex.substring(2);
    } else {
      return hex;
    }
  }

  public static byte[] hex2Bytes(String hex) {
    if (Strings.isNullOrEmpty(hex)) {
      return new byte[]{};
    }

    if (hex.startsWith("0x") || hex.startsWith("\\x")) {
      return plainHex2Bytes(hex.substring(2));
    } else {
      return plainHex2Bytes(hex);
    }
  }

  public static String bytes20xHex(byte[] bytes) {
    if (null == bytes) {
      return null;
    }
    return String.format("0x%s", bytes2PlainHex(bytes).toLowerCase());
  }

  public static String bytes2PlainHex(byte[] bytes) {
    char[] hexChars = new char[bytes.length * 2];
    for (int j = 0; j < bytes.length; j++) {
      int v = bytes[j] & 0xff;
      hexChars[j * 2] = HEX_ARRAY[v >>> 4];
      hexChars[j * 2 + 1] = HEX_ARRAY[v & 0x0f];
    }
    return new String(hexChars);
  }

  private static byte[] plainHex2Bytes(String hex) {
    int len = hex.length();
    byte[] data = new byte[len / 2];
    for (int i = 0; i < len; i += 2) {
      data[i / 2] =
          (byte)
              ((Character.digit(hex.charAt(i), 16) << 4)
                  + (Character.digit(hex.charAt(i + 1), 16)));
    }
    return data;
  }
}
