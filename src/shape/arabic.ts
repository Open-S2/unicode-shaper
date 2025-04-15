import {
  ALEFTYPE,
  APRESENT,
  ARA_LINK,
  COMBINE,
  IRRELEVANT,
  IRRELEVANT_POS,
  LAMALEF_SPACE_SUB,
  LAMTYPE,
  LAM_CHAR,
  LINKL,
  LINKR,
  NEW_TAIL_CHAR,
  PRES_ALINK,
  PRES_BLINK,
  SHADDA06_CHAR,
  SHADDA_CHAR,
  SHADDA_TATWEEL_CHAR,
  SHAPE_TABLE,
  SPACE_CHAR,
  TASHKEEL_MEDIAL,
  TASHKEEL_SPACE_SUB,
  TATWEEL_CHAR,
  U_SHAPE_AGGREGATE_TASHKEEL,
  U_SHAPE_AGGREGATE_TASHKEEL_MASK,
  U_SHAPE_LAMALEF_MASK,
  U_SHAPE_LAMALEF_RESIZE,
  U_SHAPE_LETTERS_MASK,
  U_SHAPE_LETTERS_SHAPE,
  U_SHAPE_LETTERS_SHAPE_TASHKEEL_ISOLATED,
  U_SHAPE_LETTERS_UNSHAPE,
  U_SHAPE_TASHKEEL_MASK,
  U_SHAPE_TASHKEEL_REPLACE_BY_TATWEEL,
  U_SHAPE_TASHKEEL_RESIZE,
  U_SHAPE_TEXT_DIRECTION_LOGICAL,
  U_SHAPE_TEXT_DIRECTION_MASK,
  U_SHAPE_TEXT_DIRECTION_VISUAL_LTR,
} from './internal.js';

/**
 * Check if a character is Arabic
 * @param unicode - input unicode character
 * @returns - True if Arabic
 */
export function isArabic(unicode: number): boolean {
  // Arabic (0600–06FF, 256 characters)
  if (unicode >= 0x0600 && unicode <= 0x06ff) return true;
  // Arabic Supplement (0750–077F, 48 characters)
  if (unicode >= 0x0750 && unicode <= 0x077f) return true;
  // Arabic Extended-B (0870–089F, 42 characters)
  if (unicode >= 0x0870 && unicode <= 0x089f) return true;
  // Arabic Extended-A (08A0–08FF, 96 characters)
  if (unicode >= 0x08a0 && unicode <= 0x08ff) return true;
  // Arabic Presentation Forms-A (FB50–FDFF, 631 characters)
  if (unicode >= 0xfb50 && unicode <= 0xfdff) return true;
  // Arabic Presentation Forms-B (FE70–FEFF, 141 characters)
  if (unicode >= 0xfe70 && unicode <= 0xfeff) return true;
  return false;
}

/**
 * Converts the Alef characters into an equivalent
 * LamAlef location in the 0x06xx Range, this is an
 * intermediate stage in the operation of the program
 * later it'll be converted into the 0xFExx LamAlefs
 * in the shaping function.
 * @param ch - Alef character
 * @returns - LamAlef character
 */
function changeLamAlef(ch: number): number {
  if (ch === 0x0622) return 0x065c;
  if (ch === 0x0623) return 0x065d;
  if (ch === 0x0625) return 0x065e;
  if (ch === 0x0627) return 0x065f;
  return 0;
}

/**
 * Checks if a character is Tashkeel
 * @param ch - Tashkeel unicode character
 * @returns - true for Tashkeel characters in 06 range else return false
 */
function isTashkeelChar(ch: number): boolean {
  return ch >= 0x064b && ch <= 0x0652;
}

/**
 * Checks if a character is Tashkeel Fe char
 * @param ch - Tashkeel unicode character
 * @returns - true for Tashkeel characters in FE range else return false
 */
function isTashkeelCharFe(ch: number): boolean {
  return ch >= 0xfe70 && ch <= 0xfe7f;
}

/**
 * Checks if a character is Alef
 * @param ch - Alef unicode character
 * @returns - true for Alef characters in 06 range else return false
 */
function isAlefChar(ch: number): boolean {
  return ch === 0x0622 || ch === 0x0623 || ch === 0x0625 || ch === 0x0627;
}

/**
 * Checks if a character is LamAlef
 * @param ch - LamAlef unicode character
 * @returns - true for LamAlef characters in 06 range else return false
 */
function isLamAlefChar(ch: number): boolean {
  return ch >= 0xfef5 && ch <= 0xfefc;
}

/**
 * Resolves the link between the characters as
 * Arabic characters have four forms :
 * Isolated, Initial, Middle and Final Form
 * @param ch - Unicode character
 * @returns - Link
 */
function getLink(ch: number): number {
  if (ch >= 0x0622 && ch <= 0x06d3) {
    return ARA_LINK[ch - 0x0622];
  } else if (ch === 0x200d) {
    return 3;
  } else if (ch >= 0x206d && ch <= 0x206f) {
    return 4;
  } else if (ch >= 0xfb50 && ch <= 0xfc62) {
    return PRES_ALINK[ch - 0xfb50];
  } else if (ch >= 0xfe70 && ch <= 0xfefc) {
    return PRES_BLINK[ch - 0xfe70];
  }
  return 0;
}

/**
 * Checks if the Tashkeel Character is on Tatweel or not,if the
 * Tashkeel on tatweel (FE range), it returns 1 else if the
 * Tashkeel with shadda on tatweel (FC range)return 2 otherwise
 * returns 0
 * @param ch - Tashkeel character
 * @returns - 1 for Tashkeel on Tatweel, 2 for Tashkeel with Shadda on Tatweel else return 0
 */
function isTashkeelOnTatweelChar(ch: number): number {
  if (
    ch >= 0xfe70 &&
    ch <= 0xfe7f &&
    ch !== NEW_TAIL_CHAR &&
    ch !== 0xfe75 &&
    ch !== SHADDA_TATWEEL_CHAR
  ) {
    return TASHKEEL_MEDIAL[ch - 0xfe70];
  } else if ((ch >= 0xfcf2 && ch <= 0xfcf4) || ch === SHADDA_TATWEEL_CHAR) {
    return 2;
  }

  return 0;
}

/**
 * Checks if the Tashkeel Character is in the isolated form
 * (i.e. Unicode FE range) returns 1 else if the Tashkeel
 * with shadda is in the isolated form (i.e. Unicode FC range)
 * returns 2 otherwise returns 0
 * @param ch - Tashkeel character
 * @returns - 1 for Tashkeel in isolated form, 2 for Tashkeel with Shadda in isolated form else return 0
 */
function isIsolatedTashkeelChar(ch: number): number {
  if (ch >= 0xfe70 && ch <= 0xfe7f && ch !== NEW_TAIL_CHAR && ch !== 0xfe75) {
    return 1 - TASHKEEL_MEDIAL[ch - 0xfe70];
  } else if (ch >= 0xfc5e && ch <= 0xfc63) {
    return 1;
  }

  return 0;
}

/**
 * Replaces Tashkeel as following:
 * Case 1: if the Tashkeel on tatweel, replace it with Tatweel.
 * Case 2: if the Tashkeel aggregated with Shadda on Tatweel, replace
 *         it with Shadda on Tatweel.
 * Case 3: if the Tashkeel is isolated replace it with Space.
 * @param dest - Tashkeel array
 */
function handleTashkeelWithTatweel(dest: number[]): void {
  let i = 0;
  const destLen = dest.length;
  while (i < destLen) {
    if (isTashkeelOnTatweelChar(dest[i]) === 1) {
      dest[i] = TATWEEL_CHAR;
    } else if (isTashkeelOnTatweelChar(dest[i]) === 2) {
      dest[i] = SHADDA_TATWEEL_CHAR;
    } else if (isIsolatedTashkeelChar(dest[i]) !== 0 && dest[i] !== SHADDA_CHAR) {
      dest[i] = SPACE_CHAR;
    }
    i++;
  }
}

/**
 * Counts the number of spaces at each end of the logical buffer
 * @param dest - Logical buffer
 * @returns - [spacesCountl, spacesCountr]
 */
function countSpaces(dest: number[]): [spacesCountl: number, spacesCountr: number] {
  let s = dest.length;
  let i = 0;
  let countl = 0;
  let countr = 0;
  while (dest[i] === SPACE_CHAR && countl < s) {
    countl++;
    i++;
  }
  if (countl < s) {
    // the entire buffer is not all space
    while (dest[s - 1] === SPACE_CHAR) {
      countr++;
      s--;
    }
  }

  return [countl, countr];
}

/**
 * This function inverts the buffer, it's used
 * in case the user specifies the buffer to be
 * U_SHAPE_TEXT_DIRECTION_LOGICAL
 * @param buffer - input unicode buffer
 * @param lowlimit - lowlimit
 * @param highlimit - highlimit
 */
function invertBuffer(buffer: number[], lowlimit: number, highlimit: number): void {
  // let mut tmp: u16 = 0;
  let i = lowlimit;
  let j = buffer.length - highlimit - 1;
  while (i < j) {
    [buffer[i], buffer[j]] = [buffer[j], buffer[i]];
    i++;
    j--;
  }
}

/**
 * Calculates the size of the output buffer
 * @param source - input unicode buffer
 * @param options - shaping options
 * @returns - output buffer size
 */
function calculateSize(source: number[], options: number): number {
  let destSize = source.length;
  let i: number;

  let lamAlefOption = false;
  let tashkeelOption = false;

  if (
    ((options & U_SHAPE_LETTERS_MASK) === U_SHAPE_LETTERS_SHAPE ||
      (options & U_SHAPE_LETTERS_MASK) === U_SHAPE_LETTERS_SHAPE_TASHKEEL_ISOLATED) &&
    (options & U_SHAPE_LAMALEF_MASK) === U_SHAPE_LAMALEF_RESIZE
  ) {
    lamAlefOption = true;
  }
  if (
    (options & U_SHAPE_LETTERS_MASK) === U_SHAPE_LETTERS_SHAPE &&
    (options & U_SHAPE_TASHKEEL_MASK) === U_SHAPE_TASHKEEL_RESIZE
  ) {
    tashkeelOption = true;
  }

  if (lamAlefOption || tashkeelOption) {
    if ((options & U_SHAPE_TEXT_DIRECTION_MASK) === U_SHAPE_TEXT_DIRECTION_VISUAL_LTR) {
      i = 0;
      while (i < source.length) {
        if (
          ((isAlefChar(source[i]) && i < source.length - 1 && source[i + 1] === LAM_CHAR) ||
            isTashkeelCharFe(source[i])) &&
          destSize > 0
        ) {
          destSize--;
        }
        i++;
      }
    } else if ((options & U_SHAPE_TEXT_DIRECTION_MASK) === U_SHAPE_TEXT_DIRECTION_LOGICAL) {
      i = 0;
      while (i < source.length) {
        if (
          ((source[i] === LAM_CHAR && i < source.length - 1 && isAlefChar(source[i + 1])) ||
            isTashkeelCharFe(source[i])) &&
          destSize > 0
        ) {
          destSize--;
        }
        i++;
      }
    }
  }

  if (
    (options & U_SHAPE_LETTERS_MASK) === U_SHAPE_LETTERS_UNSHAPE &&
    (options & U_SHAPE_LAMALEF_MASK) === U_SHAPE_LAMALEF_RESIZE
  ) {
    i = 0;
    while (i < source.length) {
      if (isLamAlefChar(source[i]) && destSize > 0) {
        destSize++;
      }
      i++;
    }
  }

  return destSize;
}

/**
 * Converts an Arabic Unicode buffer in 06xx Range into a shaped
 * arabic Unicode buffer in FExx Range
 * @param dest - destination unicode buffer to modify
 * @param tashkeelFlag - tashkeel flag
 */
function _shapeArabic(
  dest: number[],
  // options: u32,
  tashkeelFlag: number,
  // shapeVars: UShapeVariables
) {
  let shape: number;
  let i: number;
  let ii: number;
  const I_END: number = -1;
  let lastPos: number;
  let nx = -2;
  let nw: number;
  let prevLink = 0;
  let lastLink = 0;
  let currLink: number;
  let nextLink = 0;
  let wLamalef: number;

  // sets the index to the end of the buffer
  i = dest.length - 1;
  lastPos = i;

  // This function resolves the link between the characters .
  // Arabic characters have four forms :
  // Isolated Form, Initial Form, Middle Form and Final Form
  currLink = getLink(dest[i]);

  while (true) {
    // If high byte of currLink > 0 then more than one shape
    if ((currLink & 0xff00) > 0 || (getLink(dest[i]) & IRRELEVANT) !== 0) {
      nw = i - 1;
      while (nx < 0) {
        // we need to know about next char
        if (nw === I_END) {
          nextLink = 0;
          nx = 3000;
        } else {
          nextLink = getLink(dest[nw]);
          if ((nextLink & IRRELEVANT) === 0) {
            nx = nw;
          } else {
            nw--;
          }
        }
      }

      if ((currLink & ALEFTYPE) > 0 && (lastLink & LAMTYPE) > 0) {
        // lamalef_found = true;
        wLamalef = changeLamAlef(dest[i]); // get from 0x065C-0x065f
        if (wLamalef !== 0) {
          dest[i] = LAMALEF_SPACE_SUB; // The default case is to drop the Alef and replace
          dest[lastPos] = wLamalef; // it by LAMALEF_SPACE_SUB which is the last character in the
          i = lastPos; // unicode private use area, this is done to make
        } // sure that removeLamAlefSpaces() handles only the
        lastLink = prevLink; // spaces generated during lamalef generation.
        currLink = getLink(wLamalef); // LAMALEF_SPACE_SUB is added here and is replaced by spaces
      } // in removeLamAlefSpaces()

      // get the proper shape according to link ability of neighbors
      // and of character; depends on the order of the shapes
      // (isolated, initial, middle, final) in the compatibility area
      const si = nextLink & (LINKR + LINKL);
      const sj = lastLink & (LINKR + LINKL);
      const sk = currLink & (LINKR + LINKL);
      shape = SHAPE_TABLE[si][sj][sk];

      if ((currLink & (LINKR + LINKL)) === 1) {
        shape &= 1;
      } else if (isTashkeelChar(dest[i])) {
        if (
          (lastLink & LINKL) > 0 &&
          (nextLink & LINKR) > 0 &&
          tashkeelFlag === 1 &&
          dest[i] !== 0x064c &&
          dest[i] !== 0x064d
        ) {
          shape = 1;
          if ((nextLink & ALEFTYPE) === ALEFTYPE && (lastLink & LAMTYPE) === LAMTYPE) {
            shape = 0;
          }
        } else if (tashkeelFlag === 2 && dest[i] === SHADDA06_CHAR) {
          shape = 1;
        } else {
          shape = 0;
        }
      }
      if ((dest[i] ^ 0x0600) < 0x100) {
        if (isTashkeelChar(dest[i])) {
          if (tashkeelFlag === 2 && dest[i] !== SHADDA06_CHAR) {
            dest[i] = TASHKEEL_SPACE_SUB;
            // tashkeel_found = true;
          } else {
            const ind = dest[i] - 0x064b;
            // ensure the array index is within the range
            if (dest[i] < 0x064b || ind >= IRRELEVANT_POS.length) {
              // unreachable!();
            }
            dest[i] = 0xfe70 + IRRELEVANT_POS[ind] + shape;
          }
        } else if ((currLink & APRESENT) > 0) {
          dest[i] = 0xfb50 + (currLink >> 8) + shape;
        } else if (currLink >> 8 > 0 && (currLink & IRRELEVANT) === 0) {
          dest[i] = 0xfe70 + (currLink >> 8) + shape;
        }
      }
    }

    // move one notch forward
    if ((currLink & IRRELEVANT) === 0) {
      prevLink = lastLink;
      lastLink = currLink;
      lastPos = i;
    }

    ii = i - 1;
    // safety check
    if (ii >= 0) i--;
    if (ii === nx) {
      currLink = nextLink;
      nx = -2;
    } else if (ii !== I_END) {
      currLink = getLink(dest[i]);
    }
    if (ii === I_END) {
      break;
    }
  }
}

/**
 * Converts Arabic Unicode buffer into a shaped Arabic Unicode buffer
 * @param input - input unicode buffer
 * @param options - shaping options
 * @returns - shaped unicode buffer
 */
export function shapeArabic(input: number[], options: number): number[] {
  let sourcePtr = input;
  let tempsource: number[] = [];

  if ((options & U_SHAPE_AGGREGATE_TASHKEEL_MASK) !== 0) {
    tempsource = new Array(input.length * 2);
    const logicalOrder = (options & U_SHAPE_TEXT_DIRECTION_MASK) === U_SHAPE_TEXT_DIRECTION_LOGICAL;
    const aggregateTashkeel =
      (options & (U_SHAPE_AGGREGATE_TASHKEEL_MASK + U_SHAPE_LETTERS_SHAPE_TASHKEEL_ISOLATED)) ===
      U_SHAPE_AGGREGATE_TASHKEEL + U_SHAPE_LETTERS_SHAPE_TASHKEEL_ISOLATED;
    let j = 2 * input.length;
    if (logicalOrder) {
      j = 0;
    }
    let i = input.length;
    if (logicalOrder) {
      i = 0;
    }
    let end = input.length - 1;
    if (logicalOrder) {
      end = input.length;
    }
    let aggregation_possible = true;
    let prev = 0;
    let prevLink: number;
    let currLink = 0;
    let new_source_length = 0;

    while (i !== end) {
      prevLink = currLink;
      currLink = getLink(input[i]);
      if (
        aggregateTashkeel &&
        ((prevLink | currLink) & COMBINE) === COMBINE &&
        aggregation_possible
      ) {
        aggregation_possible = false;
        if (prev < input[i]) {
          tempsource[j] = prev - 0x064c + 0xfc5e;
        } else {
          tempsource[j] = input[i] - 0x064c + 0xfc5e;
        }
        currLink = getLink(tempsource[j]);
      } else {
        new_source_length++;
        aggregation_possible = true;
        tempsource[j] = input[i];
        if (logicalOrder) {
          j++;
        } else {
          j--;
        }
        prev = input[i];
      }
      // move one notch forward
      if (logicalOrder) {
        i++;
      } else {
        i--;
      }
    }
    if (logicalOrder) {
      sourcePtr = tempsource.slice(0, new_source_length);
    } else {
      sourcePtr = tempsource.slice(j, new_source_length);
    }
  }

  // prep output
  const outputSize = calculateSize(sourcePtr, options);
  const output: number[] = new Array(Math.max(outputSize, sourcePtr.length));
  output.push(...sourcePtr);

  if ((options & U_SHAPE_TEXT_DIRECTION_MASK) === U_SHAPE_TEXT_DIRECTION_LOGICAL) {
    const [spacesCountl, spacesCountr] = countSpaces(output);
    invertBuffer(output, spacesCountl, spacesCountr);
  }

  // Arabic shaping
  if ((options & U_SHAPE_LETTERS_MASK) === U_SHAPE_LETTERS_SHAPE) {
    if (
      (options & U_SHAPE_TASHKEEL_MASK) > 0 &&
      (options & U_SHAPE_TASHKEEL_MASK) !== U_SHAPE_TASHKEEL_REPLACE_BY_TATWEEL
    ) {
      // Call the shaping function with tashkeel flag == 2 for removal of tashkeel
      _shapeArabic(output, 2);
    } else {
      // default Call the shaping function with tashkeel flag == 1
      _shapeArabic(output, 1);

      // After shaping text check if user wants to remove tashkeel and replace it with tatweel
      if ((options & U_SHAPE_TASHKEEL_MASK) === U_SHAPE_TASHKEEL_REPLACE_BY_TATWEEL) {
        handleTashkeelWithTatweel(output);
      }
    }
  } else if ((options & U_SHAPE_LETTERS_MASK) === U_SHAPE_LETTERS_SHAPE_TASHKEEL_ISOLATED) {
    // Call the shaping function with tashkeel flag == 0
    _shapeArabic(output, 0);
  }

  if ((options & U_SHAPE_TEXT_DIRECTION_MASK) === U_SHAPE_TEXT_DIRECTION_LOGICAL) {
    const [spacesCountl, spacesCountr] = countSpaces(output);
    invertBuffer(output, spacesCountl, spacesCountr);
  }
  // End of Arabic letter shaping part

  // copy a slice to a new slice "arabicOutput" of outputSize
  // and run through output, skip every LAMALEF_SPACE_SUB and TASHKEEL_SPACE_SUB
  const arabicOutput: number[] = new Array(outputSize);
  for (const ch of output) {
    if (ch !== LAMALEF_SPACE_SUB && ch !== TASHKEEL_SPACE_SUB) arabicOutput.push(ch);
  }

  return arabicOutput;
}
