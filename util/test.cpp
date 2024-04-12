#include <stdlib.h>
#include <iostream>
#include <string>
#include <locale>
#include <codecvt>

#include <unicode/ubidi.h>
#include <unicode/ushape.h>

// g++ -stdlib=libc++ -std=c++20 -w -fPIC -I/usr/local/include -L/usr/local/lib -l icuuc test.cpp -o test

int main () {
  std::wstring_convert<std::codecvt_utf8<char16_t>, char16_t> converter;
  // wstring_convert<codecvt_utf16<wchar_t, 0x10ffff, little_endian>, wchar_t> conv;

  // prep
  UErrorCode errorCode = U_ZERO_ERROR;
  // std::u16string inputStr(u"سلام۳۹");
  // ۳۹ﻡﻼﺳ

  // std::u16string inputStr(u" بَّترم بَّترم ");
  //  ﻡﺮﺘﹽﹷﺑ ﻡﺮﺘﹷﹽﺑ

  // std::u16string inputStr(u"مكتبة الإسكندرية‎‎ Maktabat al-Iskandarīyah");
  //  Maktabat al-Iskandarīyahﺔﻳﺭﺪﻨﻜﺳﻹﺍ ﺔﺒﺘﻜﻣ

  // std::u16string inputStr(u"\"سلام۳۹\" is not English");
  // is not English "۳۹ﻡﻼﺳ"
  
  // std::u16string inputStr(u"(سلام۳۹) is not English");
  // is not English (۳۹ﻡﻼﺳ)

  // std::u16string inputStr(u"An example of another language: سلام۳۹");
  // An example of another language: ۳۹ﻡﻼﺳ

  // std::u16string inputStr(u"Start سلام۳۹");
  // Start ۳۹ﻡﻼﺳ

  // std::u16string inputStr(u"مكتبة الإسكندرية (Maktabat al-Iskandarīyah)");

  // std::u16string inputStr(u"An example of another language: \nسلام۳۹ is not english");
  // An example of another language:
  // is not english ۳۹ﻡﻼﺳ

  // std::u16string inputStr(u"Start سلام۳۹ END بَّترم بَّترم ");
  // Start ۳۹ﻡﻼﺳ END ﻡﺮﺘﹽﹷﺑ ﻡﺮﺘﹷﹽﺑ

  // std::u16string inputStr(u"سلام۳۹ Start بَّترم بَّترم END");
  // END ﻡﺮﺘﹽﹷﺑ ﻡﺮﺘﹷﹽﺑ Start ۳۹ﻡﻼﺳ

  // std::u16string inputStr(u"سلام۳۹ Start          بَّترم بَّترم END");
  // END ﻡﺮﺘﹽﹷﺑ ﻡﺮﺘﹷﹽﺑ          Start ۳۹ﻡﻼﺳ

  std::u16string inputStr(u"سلام۳۹ is not English");

  const UChar* input = reinterpret_cast<const UChar*>(inputStr.c_str());
  uint32_t input_length = inputStr.length();

  // * STEP 1: Arabic shaping
  // get output size
  int32_t arabic_length = u_shapeArabic(input, input_length, NULL, 0,
                      (U_SHAPE_LETTERS_SHAPE & U_SHAPE_LETTERS_MASK) |
                          (U_SHAPE_TEXT_DIRECTION_LOGICAL & U_SHAPE_TEXT_DIRECTION_MASK),
                      &errorCode);
  // Pre-flighting will always set U_BUFFER_OVERFLOW_ERROR
  errorCode = U_ZERO_ERROR;

  // build output
  UChar* arabic = (UChar*)malloc(arabic_length * sizeof(UChar));
  u_shapeArabic(input, input_length, arabic, arabic_length,
                  (U_SHAPE_LETTERS_SHAPE & U_SHAPE_LETTERS_MASK) |
                      (U_SHAPE_TEXT_DIRECTION_LOGICAL & U_SHAPE_TEXT_DIRECTION_MASK),
                  &errorCode);
  // handle errors
  if (U_FAILURE(errorCode)) {
      free(arabic);
  }

  std::u16string arabicStr(arabic, arabic_length);
  // build string
  for (const auto& c : arabicStr) {
    printf("%#d ", c);
  }
  std::cout << std::endl << converter.to_bytes(arabicStr) << std::endl;

  // * STEP 2: BIDIRECTIONAL
  // setup ubidi
  UBiDi* bidiLine = ubidi_open();
  ubidi_setPara(bidiLine, arabic, arabic_length, UBIDI_DEFAULT_LTR, NULL, &errorCode);
  if (U_FAILURE(errorCode)) {
    ubidi_close(bidiLine);
    free(arabic);
  }

  // UBIDI_DO_MIRRORING: Apply unicode mirroring of characters like parentheses
  // UBIDI_REMOVE_BIDI_CONTROLS: Now that all the lines are set, remove control characters so that
  // they don't show up on screen (some fonts have glyphs representing them)
  int32_t output_length = ubidi_writeReordered(bidiLine, NULL, 0, UBIDI_DO_MIRRORING | UBIDI_REMOVE_BIDI_CONTROLS, &errorCode);
  // Pre-flighting will always set U_BUFFER_OVERFLOW_ERROR
  errorCode = U_ZERO_ERROR;

  UChar* output = (UChar*)malloc(output_length * sizeof(UChar));

  ubidi_writeReordered(bidiLine, output, output_length, UBIDI_DO_MIRRORING | UBIDI_REMOVE_BIDI_CONTROLS, &errorCode);

  if (U_FAILURE(errorCode)) {
    // printf("ubidi_writeReordered Error code: %u\n", errorCode);
    ubidi_close(bidiLine);
    free(output);
    free(arabic);
  }

  std::u16string outputStr(output, output_length);
  // build string
  for (const auto& c : outputStr) {
    printf("%#d ", c);
  }
  std::cout << std::endl << converter.to_bytes(outputStr) << std::endl;

  free(output);
  free(arabic);

  return 0;
}