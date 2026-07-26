---
url: https://docs.sunmi.com/zh-CN/cdixeghjk491/xfxqeghjk513
---

# 扫码头返回类型
更新时间：2025-09-24 18:37:35  
| Number  | Code Type  | CodeID Zebra  | CodeID Newland  | CodeID Fp/NL  | Notice  |   
 | **Example**  |   
 |  
| --- | --- | --- | --- | --- | --- | --- | --- | --- |  
| 1  | Code128  | D  | j  | j  | Newland,Fp/NL: <br>AIM-128 — f <br>SETTING 128 — t  |   
 |   
 |   
 |  
| 2  | UCC·EAN128(GS1-128)  |   
 | j  | u  |   
 |   
 |   
 |   
 |  
| 3  | ISBT 128  | D  | j  | j  | Zebra: <br>ISBT 128 Concatenated — D  |   
 |   
 |   
 |  
| 4  | EAN8  | A  | d  | g  |   
 |   
 |   
 |   
 |  
| 5  | EAN13  | A  | d  | d  |   
 |   
 |   
 |   
 |  
| 6  | UPC-E  | A  | c  | h  |   
 |   
 |   
 |   
 |  
| 7  | UPC-E1  | A  |   
 |   
 |   
 |   
 |   
 |   
 |  
| 8  | UPC-A  | A  | c  | c  |   
 | **Instruction**  |   
 |   
 |  
| 9  | Interleaved 2 of 5（ITF）  | F  | e  | e  | Newland,: <br>ITF-6 — e <br>ITF-14 — e <br>Fp/NL: <br>ITF-6 — r <br>ITF-14 — q  | **Head noun**  | **Paraphrase**  | **Scanner ID**  |  
| 10  | Matrix 2 of 5  | S  | v  | v  |   
 | **Nls**  | **NewLand (EM2096)**  | **]N**  |  
| 11  | Code39  | B  | b  | b  | Zebra: <br>Trioptic Code 39 — M  | **Zebra**  | **Zebra (4710)**  | **]Z**  |  
| 12  | Codabar  | C  | a  | a  |   
 | **Fp(Falcon)**  | **Falcon(BSM1825)**  | **]FN**  |  
| 13  | Code93  | E  | i  | y  |   
 | **NL**  | **NewLand(EM1365)**  | **]FN**  |  
| 14  | GS1 DataBar(RSS)  | R  | R  | R  |   
 |   
 |   
 |   
 |  
| 15  | Composite-UCC  | T  |   
 |   
 | Zebra: <br>MiroQRTCIF Linked Code 39(TLC 39) — T  |   
 |   
 |   
 |  
| 16  | Composite-UPC  |   
 |   
 |   
 |   
 |   
 |   
 |   
 |  
| 17  | Code11  | H  | H  | z  |   
 |   
 |   
 |   
 |  
| 18  | ISBN(Bookland EAN)  | L  | B  | B  |   
 | **Return：**  | **Scanner ID+Code ID**  |   
 |  
| 19  | Industrial 2 of 5  |   
 | D  | i  |   
 |   
 |   
 |   
 |  
| 20  | Standard 2 of 5  |   
 | s  | s  |   
 | **Example： <br>scanner：Zebra <br>code content：123456 <br>code type：code 128 <br> <br>return：]ZD123456**  |   
 |   
 |  
| 21  | Discrete 2 of 5（DTF）  | G  |   
 |   
 |   
 |   
 |   
 |   
 |  
| 22  | Chinese 2 of 5  | U  |   
 |   
 |   
 |   
 |   
 |   
 |  
| 23  | Korea 3 of 5  | V  |   
 |   
 |   
 |   
 |   
 |   
 |  
| 24  | Plessey  |   
 | p  | p  | Newland: <br>UK Plessey — p  |   
 |   
 |   
 |  
| 25  | MIS-Plessey  | J  | m  | m  |   
 |   
 |   
 |   
 |  
| 26  | Composite A/B  |   
 |   
 |   
 |   
 |   
 |   
 |   
 |  
| 27  | Composite C  |   
 |   
 |   
 |   
 |   
 |   
 |   
 |  
| 28  | ISSN EAN  | X  | n  | n  |   
 |   
 |   
 |   
 |  
| 29  | PDF417  | X  | r  |   
 |   
 |   
 |   
 |   
 |  
| 30  | QR Code  | P01  | Q  |   
 |   
 |   
 |   
 |   
 |  
| 31  | Aztec  | z  |   
 |   
 |   
 |   
 |   
 |   
 |  
| 32  | DataMatrix  | P00  | u  |   
 |   
 |   
 |   
 |   
 |  
| 33  | HanXin  | P0H  | h  |   
 |   
 |   
 |   
 |   
 |  
| 34  | MaxiCode  | P02  |   
 |   
 |   
 |   
 |   
 |   
 |  
| 35  | AustralinPostal  | P08  |   
 |   
 |   
 |   
 |   
 |   
 |  
| 36  | US Postnet  | P03  |   
 |   
 |   
 |   
 |   
 |   
 |  
| 37  | US Planet  | P04  |   
 |   
 |   
 |   
 |   
 |   
 |  
| 38  | Uk Postal  | P06  |   
 |   
 |   
 |   
 |   
 |   
 |  
| 39  | Japan Postal  | P05  |   
 |   
 |   
 |   
 |   
 |   
 |  
| 40  | Deutsche 12  |   
 | l  | l  |   
 |   
 |   
 |   
 |  
| 41  | Deutsche 14  |   
 | w  | w  |   
 |   
 |   
 |   
 |  
| 42  | Code32  | B  | b  | b  |   
 |   
 |   
 |   
 |  
| 43  | Netherlands KIX Code  | P08  |   
 |   
 |   
 |   
 |   
 |   
 |  
| 44  | USPS 4CB/One Code/Intelligent Mail  | P0A  |   
 |   
 |   
 |   
 |   
 |   
 |  
| 45  | UPU FICS Postal  | P0B  |   
 |   
 |   
 |   
 |   
 |   
 |  
| 46  | Signature Capture  | P0X  |   
 |   
 |   
 |   
 |   
 |   
 |  
| 47  | Coupon Code  | N  |   
 |   
 |   
 |   
 |   
 |   
 |  
上一篇：扫码底座
下一篇：扫码器使用指南
