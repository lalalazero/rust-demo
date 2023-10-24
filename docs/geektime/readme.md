## String 和 &str 的区别

String 内部存储的是 Unicode 字符串的 UTF8 编码，而 char 直接存的是 Unicode Scalar Value。

这两个的区别解释：

1. unicode 字符集用巨量的字符集来表示全世界所有的文字和字符，每个文字和符号对应一个单独的 unicode 字符编码，也就是 U+XXX 这样的编码。
比如英文的 `A` 对应 `U+0041`，中文的 `严` 对应 `U+4E25`

2. 这些 U+ 字符集范围从 U+0 开始直到  U+10FFFF（65536位）

3. 每一个 unicode 字符叫做 code point

4. 因为 unicode 字符集实在太大太大了，因此为了更好的区分，划分了多个基础类型，叫做 code point type，包含 Graphic, Format, Control, Private-Use, Surrogate, Noncharacter, Reserved

5. 在 unicode 字符的传输和存储中，根据不同的算法划分出了 UTF-8,UTF-16,UTF-32。比如中文的 `严` 对应 `U+4E25`，转成二进制是 `0002 0005`，很显然这是8位二进制，也就是需要1个字节来存。再来看英文的 `A` 对应 `U+0041` 也就是二进制的 `0041`，很显然不同的字符需要不同的字节数来存。

6. 因为每个语言对应的 unicode 字符编码长度不一样，因此可以采用定长的位数来存 unicode 字符，比如 UTF-32 表示固定用 32位数字（二进制）来存，UTF-8 则是变长的，首先 UTF-8 把 unicode 字符划分成了几个范围，然后根据标志位来判断，这是一个 8/16/24/32 位的字符。这样节省了空间，又能完全表示所有的 unicode 字符。

7. surrogate 有两个范围的 unicode 编码组成，高的叫做 high-surrogate，低的叫做 low-surrogate，不在这个范围的编码叫做 scalar value

8. 也就是说一个 unicode 字符编码，要么是 surrogate 要么是 scalar value 

9. surrogate 在 UTF-16 中使用,UTF-8 没用到，所以不需要这个概念