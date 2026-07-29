
sieve-native:     file format elf64-x86-64


Disassembly of section .init:

0000000000001000 <_init>:
    1000:	f3 0f 1e fa          	endbr64
    1004:	48 83 ec 08          	sub    rsp,0x8
    1008:	48 8b 05 d9 2f 01 00 	mov    rax,QWORD PTR [rip+0x12fd9]        # 13fe8 <__gmon_start__@Base>
    100f:	48 85 c0             	test   rax,rax
    1012:	74 02                	je     1016 <_init+0x16>
    1014:	ff d0                	call   rax
    1016:	48 83 c4 08          	add    rsp,0x8
    101a:	c3                   	ret

Disassembly of section .plt:

0000000000001020 <getenv@plt-0x10>:
    1020:	ff 35 2a 2f 01 00    	push   QWORD PTR [rip+0x12f2a]        # 13f50 <_GLOBAL_OFFSET_TABLE_+0x8>
    1026:	ff 25 2c 2f 01 00    	jmp    QWORD PTR [rip+0x12f2c]        # 13f58 <_GLOBAL_OFFSET_TABLE_+0x10>
    102c:	0f 1f 40 00          	nop    DWORD PTR [rax+0x0]

0000000000001030 <getenv@plt>:
    1030:	ff 25 2a 2f 01 00    	jmp    QWORD PTR [rip+0x12f2a]        # 13f60 <getenv@GLIBC_2.2.5>
    1036:	68 00 00 00 00       	push   0x0
    103b:	e9 e0 ff ff ff       	jmp    1020 <_init+0x20>

0000000000001040 <free@plt>:
    1040:	ff 25 22 2f 01 00    	jmp    QWORD PTR [rip+0x12f22]        # 13f68 <free@GLIBC_2.2.5>
    1046:	68 01 00 00 00       	push   0x1
    104b:	e9 d0 ff ff ff       	jmp    1020 <_init+0x20>

0000000000001050 <strlen@plt>:
    1050:	ff 25 1a 2f 01 00    	jmp    QWORD PTR [rip+0x12f1a]        # 13f70 <strlen@GLIBC_2.2.5>
    1056:	68 02 00 00 00       	push   0x2
    105b:	e9 c0 ff ff ff       	jmp    1020 <_init+0x20>

0000000000001060 <__stack_chk_fail@plt>:
    1060:	ff 25 12 2f 01 00    	jmp    QWORD PTR [rip+0x12f12]        # 13f78 <__stack_chk_fail@GLIBC_2.4>
    1066:	68 03 00 00 00       	push   0x3
    106b:	e9 b0 ff ff ff       	jmp    1020 <_init+0x20>

0000000000001070 <snprintf@plt>:
    1070:	ff 25 0a 2f 01 00    	jmp    QWORD PTR [rip+0x12f0a]        # 13f80 <snprintf@GLIBC_2.2.5>
    1076:	68 04 00 00 00       	push   0x4
    107b:	e9 a0 ff ff ff       	jmp    1020 <_init+0x20>

0000000000001080 <fputc@plt>:
    1080:	ff 25 02 2f 01 00    	jmp    QWORD PTR [rip+0x12f02]        # 13f88 <fputc@GLIBC_2.2.5>
    1086:	68 05 00 00 00       	push   0x5
    108b:	e9 90 ff ff ff       	jmp    1020 <_init+0x20>

0000000000001090 <memcmp@plt>:
    1090:	ff 25 fa 2e 01 00    	jmp    QWORD PTR [rip+0x12efa]        # 13f90 <memcmp@GLIBC_2.2.5>
    1096:	68 06 00 00 00       	push   0x6
    109b:	e9 80 ff ff ff       	jmp    1020 <_init+0x20>

00000000000010a0 <_setjmp@plt>:
    10a0:	ff 25 f2 2e 01 00    	jmp    QWORD PTR [rip+0x12ef2]        # 13f98 <_setjmp@GLIBC_2.2.5>
    10a6:	68 07 00 00 00       	push   0x7
    10ab:	e9 70 ff ff ff       	jmp    1020 <_init+0x20>

00000000000010b0 <fprintf@plt>:
    10b0:	ff 25 ea 2e 01 00    	jmp    QWORD PTR [rip+0x12eea]        # 13fa0 <fprintf@GLIBC_2.2.5>
    10b6:	68 08 00 00 00       	push   0x8
    10bb:	e9 60 ff ff ff       	jmp    1020 <_init+0x20>

00000000000010c0 <memcpy@plt>:
    10c0:	ff 25 e2 2e 01 00    	jmp    QWORD PTR [rip+0x12ee2]        # 13fa8 <memcpy@GLIBC_2.14>
    10c6:	68 09 00 00 00       	push   0x9
    10cb:	e9 50 ff ff ff       	jmp    1020 <_init+0x20>

00000000000010d0 <malloc@plt>:
    10d0:	ff 25 da 2e 01 00    	jmp    QWORD PTR [rip+0x12eda]        # 13fb0 <malloc@GLIBC_2.2.5>
    10d6:	68 0a 00 00 00       	push   0xa
    10db:	e9 40 ff ff ff       	jmp    1020 <_init+0x20>

00000000000010e0 <realloc@plt>:
    10e0:	ff 25 d2 2e 01 00    	jmp    QWORD PTR [rip+0x12ed2]        # 13fb8 <realloc@GLIBC_2.2.5>
    10e6:	68 0b 00 00 00       	push   0xb
    10eb:	e9 30 ff ff ff       	jmp    1020 <_init+0x20>

00000000000010f0 <longjmp@plt>:
    10f0:	ff 25 ca 2e 01 00    	jmp    QWORD PTR [rip+0x12eca]        # 13fc0 <longjmp@GLIBC_2.2.5>
    10f6:	68 0c 00 00 00       	push   0xc
    10fb:	e9 20 ff ff ff       	jmp    1020 <_init+0x20>

0000000000001100 <exit@plt>:
    1100:	ff 25 c2 2e 01 00    	jmp    QWORD PTR [rip+0x12ec2]        # 13fc8 <exit@GLIBC_2.2.5>
    1106:	68 0d 00 00 00       	push   0xd
    110b:	e9 10 ff ff ff       	jmp    1020 <_init+0x20>

0000000000001110 <fwrite@plt>:
    1110:	ff 25 ba 2e 01 00    	jmp    QWORD PTR [rip+0x12eba]        # 13fd0 <fwrite@GLIBC_2.2.5>
    1116:	68 0e 00 00 00       	push   0xe
    111b:	e9 00 ff ff ff       	jmp    1020 <_init+0x20>

Disassembly of section .plt.got:

0000000000001120 <__cxa_finalize@plt>:
    1120:	ff 25 d2 2e 01 00    	jmp    QWORD PTR [rip+0x12ed2]        # 13ff8 <__cxa_finalize@GLIBC_2.2.5>
    1126:	66 90                	xchg   ax,ax

Disassembly of section .text:

0000000000001130 <_start>:
    1130:	f3 0f 1e fa          	endbr64
    1134:	31 ed                	xor    ebp,ebp
    1136:	49 89 d1             	mov    r9,rdx
    1139:	5e                   	pop    rsi
    113a:	48 89 e2             	mov    rdx,rsp
    113d:	48 83 e4 f0          	and    rsp,0xfffffffffffffff0
    1141:	50                   	push   rax
    1142:	54                   	push   rsp
    1143:	45 31 c0             	xor    r8d,r8d
    1146:	31 c9                	xor    ecx,ecx
    1148:	48 8d 3d b8 59 00 00 	lea    rdi,[rip+0x59b8]        # 6b07 <main>
    114f:	ff 15 83 2e 01 00    	call   QWORD PTR [rip+0x12e83]        # 13fd8 <__libc_start_main@GLIBC_2.34>
    1155:	f4                   	hlt
    1156:	66 2e 0f 1f 84 00 00 	cs nop WORD PTR [rax+rax*1+0x0]
    115d:	00 00 00 

0000000000001160 <deregister_tm_clones>:
    1160:	48 8d 3d c1 2e 01 00 	lea    rdi,[rip+0x12ec1]        # 14028 <__TMC_END__>
    1167:	48 8d 05 ba 2e 01 00 	lea    rax,[rip+0x12eba]        # 14028 <__TMC_END__>
    116e:	48 39 f8             	cmp    rax,rdi
    1171:	74 15                	je     1188 <deregister_tm_clones+0x28>
    1173:	48 8b 05 66 2e 01 00 	mov    rax,QWORD PTR [rip+0x12e66]        # 13fe0 <_ITM_deregisterTMCloneTable@Base>
    117a:	48 85 c0             	test   rax,rax
    117d:	74 09                	je     1188 <deregister_tm_clones+0x28>
    117f:	ff e0                	jmp    rax
    1181:	0f 1f 80 00 00 00 00 	nop    DWORD PTR [rax+0x0]
    1188:	c3                   	ret
    1189:	0f 1f 80 00 00 00 00 	nop    DWORD PTR [rax+0x0]

0000000000001190 <register_tm_clones>:
    1190:	48 8d 3d 91 2e 01 00 	lea    rdi,[rip+0x12e91]        # 14028 <__TMC_END__>
    1197:	48 8d 35 8a 2e 01 00 	lea    rsi,[rip+0x12e8a]        # 14028 <__TMC_END__>
    119e:	48 29 fe             	sub    rsi,rdi
    11a1:	48 89 f0             	mov    rax,rsi
    11a4:	48 c1 ee 3f          	shr    rsi,0x3f
    11a8:	48 c1 f8 03          	sar    rax,0x3
    11ac:	48 01 c6             	add    rsi,rax
    11af:	48 d1 fe             	sar    rsi,1
    11b2:	74 14                	je     11c8 <register_tm_clones+0x38>
    11b4:	48 8b 05 35 2e 01 00 	mov    rax,QWORD PTR [rip+0x12e35]        # 13ff0 <_ITM_registerTMCloneTable@Base>
    11bb:	48 85 c0             	test   rax,rax
    11be:	74 08                	je     11c8 <register_tm_clones+0x38>
    11c0:	ff e0                	jmp    rax
    11c2:	66 0f 1f 44 00 00    	nop    WORD PTR [rax+rax*1+0x0]
    11c8:	c3                   	ret
    11c9:	0f 1f 80 00 00 00 00 	nop    DWORD PTR [rax+0x0]

00000000000011d0 <__do_global_dtors_aux>:
    11d0:	f3 0f 1e fa          	endbr64
    11d4:	80 3d 8d 2e 01 00 00 	cmp    BYTE PTR [rip+0x12e8d],0x0        # 14068 <completed.0>
    11db:	75 2b                	jne    1208 <__do_global_dtors_aux+0x38>
    11dd:	55                   	push   rbp
    11de:	48 83 3d 12 2e 01 00 	cmp    QWORD PTR [rip+0x12e12],0x0        # 13ff8 <__cxa_finalize@GLIBC_2.2.5>
    11e5:	00 
    11e6:	48 89 e5             	mov    rbp,rsp
    11e9:	74 0c                	je     11f7 <__do_global_dtors_aux+0x27>
    11eb:	48 8b 3d 16 2e 01 00 	mov    rdi,QWORD PTR [rip+0x12e16]        # 14008 <__dso_handle>
    11f2:	e8 29 ff ff ff       	call   1120 <__cxa_finalize@plt>
    11f7:	e8 64 ff ff ff       	call   1160 <deregister_tm_clones>
    11fc:	c6 05 65 2e 01 00 01 	mov    BYTE PTR [rip+0x12e65],0x1        # 14068 <completed.0>
    1203:	5d                   	pop    rbp
    1204:	c3                   	ret
    1205:	0f 1f 00             	nop    DWORD PTR [rax]
    1208:	c3                   	ret
    1209:	0f 1f 80 00 00 00 00 	nop    DWORD PTR [rax+0x0]

0000000000001210 <frame_dummy>:
    1210:	f3 0f 1e fa          	endbr64
    1214:	e9 77 ff ff ff       	jmp    1190 <register_tm_clones>
    1219:	0f 1f 80 00 00 00 00 	nop    DWORD PTR [rax+0x0]

0000000000001220 <zero?>:
    1220:	55                   	push   rbp
    1221:	48 89 e5             	mov    rbp,rsp
    1224:	48 83 ec 10          	sub    rsp,0x10
    1228:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    122c:	4c 89 6c 24 08       	mov    QWORD PTR [rsp+0x8],r13
    1231:	48 89 f3             	mov    rbx,rsi
    1234:	49 89 d5             	mov    r13,rdx
    1237:	bf 01 00 00 00       	mov    edi,0x1
    123c:	4c 8d 15 51 59 00 00 	lea    r10,[rip+0x5951]        # 6b94 <cljn_gc_enter>
    1243:	41 ff d2             	call   r10
    1246:	49 89 d8             	mov    r8,rbx
    1249:	49 83 f8 01          	cmp    r8,0x1
    124d:	0f 84 2a 00 00 00    	je     127d <zero?+0x5d>
    1253:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    125a:	48 8d 05 1b 66 00 00 	lea    rax,[rip+0x661b]        # 787c <cljn_check_arity>
    1261:	4c 89 c7             	mov    rdi,r8
    1264:	ff d0                	call   rax
    1266:	b8 02 00 00 00       	mov    eax,0x2
    126b:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    126f:	4c 8b 6c 24 08       	mov    r13,QWORD PTR [rsp+0x8]
    1274:	48 83 c4 10          	add    rsp,0x10
    1278:	48 89 ec             	mov    rsp,rbp
    127b:	5d                   	pop    rbp
    127c:	c3                   	ret
    127d:	4c 89 ea             	mov    rdx,r13
    1280:	48 8b 3a             	mov    rdi,QWORD PTR [rdx]
    1283:	48 8d 08             	lea    rcx,[rax]
    1286:	49 89 c5             	mov    r13,rax
    1289:	48 8d 05 f0 2d 01 00 	lea    rax,[rip+0x12df0]        # 14080 <gc_stack>
    1290:	48 6b c9 08          	imul   rcx,rcx,0x8
    1294:	48 89 3c 08          	mov    QWORD PTR [rax+rcx*1],rdi
    1298:	be 01 00 00 00       	mov    esi,0x1
    129d:	48 8d 15 ab c9 00 00 	lea    rdx,[rip+0xc9ab]        # dc4f <cljn_eq>
    12a4:	ff d2                	call   rdx
    12a6:	48 89 c3             	mov    rbx,rax
    12a9:	48 8d 15 9a 59 00 00 	lea    rdx,[rip+0x599a]        # 6c4a <cljn_gc_leave>
    12b0:	4c 89 ef             	mov    rdi,r13
    12b3:	ff d2                	call   rdx
    12b5:	48 89 d8             	mov    rax,rbx
    12b8:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    12bc:	4c 8b 6c 24 08       	mov    r13,QWORD PTR [rsp+0x8]
    12c1:	48 83 c4 10          	add    rsp,0x10
    12c5:	48 89 ec             	mov    rsp,rbp
    12c8:	5d                   	pop    rbp
    12c9:	c3                   	ret
    12ca:	00 00                	add    BYTE PTR [rax],al
    12cc:	00 00                	add    BYTE PTR [rax],al
	...

00000000000012d0 <pos?>:
    12d0:	55                   	push   rbp
    12d1:	48 89 e5             	mov    rbp,rsp
    12d4:	48 83 ec 10          	sub    rsp,0x10
    12d8:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    12dc:	4c 89 74 24 08       	mov    QWORD PTR [rsp+0x8],r14
    12e1:	48 89 d3             	mov    rbx,rdx
    12e4:	49 89 f6             	mov    r14,rsi
    12e7:	bf 01 00 00 00       	mov    edi,0x1
    12ec:	48 8d 15 a1 58 00 00 	lea    rdx,[rip+0x58a1]        # 6b94 <cljn_gc_enter>
    12f3:	ff d2                	call   rdx
    12f5:	4c 89 f2             	mov    rdx,r14
    12f8:	48 83 fa 01          	cmp    rdx,0x1
    12fc:	0f 84 2b 00 00 00    	je     132d <pos?+0x5d>
    1302:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    1309:	4c 8d 0d 6c 65 00 00 	lea    r9,[rip+0x656c]        # 787c <cljn_check_arity>
    1310:	48 89 d7             	mov    rdi,rdx
    1313:	41 ff d1             	call   r9
    1316:	b8 02 00 00 00       	mov    eax,0x2
    131b:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    131f:	4c 8b 74 24 08       	mov    r14,QWORD PTR [rsp+0x8]
    1324:	48 83 c4 10          	add    rsp,0x10
    1328:	48 89 ec             	mov    rsp,rbp
    132b:	5d                   	pop    rbp
    132c:	c3                   	ret
    132d:	48 89 da             	mov    rdx,rbx
    1330:	48 8b 3a             	mov    rdi,QWORD PTR [rdx]
    1333:	4c 8d 18             	lea    r11,[rax]
    1336:	49 89 c6             	mov    r14,rax
    1339:	48 8d 35 40 2d 01 00 	lea    rsi,[rip+0x12d40]        # 14080 <gc_stack>
    1340:	4d 6b db 08          	imul   r11,r11,0x8
    1344:	4a 89 3c 1e          	mov    QWORD PTR [rsi+r11*1],rdi
    1348:	be 01 00 00 00       	mov    esi,0x1
    134d:	48 89 f8             	mov    rax,rdi
    1350:	48 83 e0 01          	and    rax,0x1
    1354:	48 a9 01 00 00 00    	test   rax,0x1
    135a:	0f 85 11 00 00 00    	jne    1371 <pos?+0xa1>
    1360:	48 8d 0d 04 c1 00 00 	lea    rcx,[rip+0xc104]        # d46b <cljn_gt>
    1367:	ff d1                	call   rcx
    1369:	48 89 c3             	mov    rbx,rax
    136c:	e9 19 00 00 00       	jmp    138a <pos?+0xba>
    1371:	48 d1 ff             	sar    rdi,1
    1374:	48 d1 fe             	sar    rsi,1
    1377:	b8 06 00 00 00       	mov    eax,0x6
    137c:	48 3b fe             	cmp    rdi,rsi
    137f:	48 0f 4f 05 29 00 00 	cmovg  rax,QWORD PTR [rip+0x29]        # 13b0 <pos?+0xe0>
    1386:	00 
    1387:	48 89 c3             	mov    rbx,rax
    138a:	4c 8d 0d b9 58 00 00 	lea    r9,[rip+0x58b9]        # 6c4a <cljn_gc_leave>
    1391:	4c 89 f7             	mov    rdi,r14
    1394:	41 ff d1             	call   r9
    1397:	48 89 d8             	mov    rax,rbx
    139a:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    139e:	4c 8b 74 24 08       	mov    r14,QWORD PTR [rsp+0x8]
    13a3:	48 83 c4 10          	add    rsp,0x10
    13a7:	48 89 ec             	mov    rsp,rbp
    13aa:	5d                   	pop    rbp
    13ab:	c3                   	ret
    13ac:	00 00                	add    BYTE PTR [rax],al
    13ae:	00 00                	add    BYTE PTR [rax],al
    13b0:	0a 00                	or     al,BYTE PTR [rax]
    13b2:	00 00                	add    BYTE PTR [rax],al
    13b4:	00 00                	add    BYTE PTR [rax],al
	...

00000000000013b8 <neg?>:
    13b8:	55                   	push   rbp
    13b9:	48 89 e5             	mov    rbp,rsp
    13bc:	48 83 ec 10          	sub    rsp,0x10
    13c0:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    13c4:	4c 89 74 24 08       	mov    QWORD PTR [rsp+0x8],r14
    13c9:	48 89 d3             	mov    rbx,rdx
    13cc:	49 89 f6             	mov    r14,rsi
    13cf:	bf 01 00 00 00       	mov    edi,0x1
    13d4:	48 8d 15 b9 57 00 00 	lea    rdx,[rip+0x57b9]        # 6b94 <cljn_gc_enter>
    13db:	ff d2                	call   rdx
    13dd:	4c 89 f2             	mov    rdx,r14
    13e0:	48 83 fa 01          	cmp    rdx,0x1
    13e4:	0f 84 2b 00 00 00    	je     1415 <neg?+0x5d>
    13ea:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    13f1:	4c 8d 0d 84 64 00 00 	lea    r9,[rip+0x6484]        # 787c <cljn_check_arity>
    13f8:	48 89 d7             	mov    rdi,rdx
    13fb:	41 ff d1             	call   r9
    13fe:	b8 02 00 00 00       	mov    eax,0x2
    1403:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    1407:	4c 8b 74 24 08       	mov    r14,QWORD PTR [rsp+0x8]
    140c:	48 83 c4 10          	add    rsp,0x10
    1410:	48 89 ec             	mov    rsp,rbp
    1413:	5d                   	pop    rbp
    1414:	c3                   	ret
    1415:	48 89 da             	mov    rdx,rbx
    1418:	48 8b 3a             	mov    rdi,QWORD PTR [rdx]
    141b:	4c 8d 18             	lea    r11,[rax]
    141e:	49 89 c6             	mov    r14,rax
    1421:	48 8d 35 58 2c 01 00 	lea    rsi,[rip+0x12c58]        # 14080 <gc_stack>
    1428:	4d 6b db 08          	imul   r11,r11,0x8
    142c:	4a 89 3c 1e          	mov    QWORD PTR [rsi+r11*1],rdi
    1430:	be 01 00 00 00       	mov    esi,0x1
    1435:	48 89 f8             	mov    rax,rdi
    1438:	48 83 e0 01          	and    rax,0x1
    143c:	48 a9 01 00 00 00    	test   rax,0x1
    1442:	0f 85 11 00 00 00    	jne    1459 <neg?+0xa1>
    1448:	48 8d 0d 68 bf 00 00 	lea    rcx,[rip+0xbf68]        # d3b7 <cljn_lt>
    144f:	ff d1                	call   rcx
    1451:	48 89 c3             	mov    rbx,rax
    1454:	e9 19 00 00 00       	jmp    1472 <neg?+0xba>
    1459:	48 d1 ff             	sar    rdi,1
    145c:	48 d1 fe             	sar    rsi,1
    145f:	b8 06 00 00 00       	mov    eax,0x6
    1464:	48 3b fe             	cmp    rdi,rsi
    1467:	48 0f 4c 05 29 00 00 	cmovl  rax,QWORD PTR [rip+0x29]        # 1498 <neg?+0xe0>
    146e:	00 
    146f:	48 89 c3             	mov    rbx,rax
    1472:	4c 8d 0d d1 57 00 00 	lea    r9,[rip+0x57d1]        # 6c4a <cljn_gc_leave>
    1479:	4c 89 f7             	mov    rdi,r14
    147c:	41 ff d1             	call   r9
    147f:	48 89 d8             	mov    rax,rbx
    1482:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    1486:	4c 8b 74 24 08       	mov    r14,QWORD PTR [rsp+0x8]
    148b:	48 83 c4 10          	add    rsp,0x10
    148f:	48 89 ec             	mov    rsp,rbp
    1492:	5d                   	pop    rbp
    1493:	c3                   	ret
    1494:	00 00                	add    BYTE PTR [rax],al
    1496:	00 00                	add    BYTE PTR [rax],al
    1498:	0a 00                	or     al,BYTE PTR [rax]
    149a:	00 00                	add    BYTE PTR [rax],al
    149c:	00 00                	add    BYTE PTR [rax],al
	...

00000000000014a0 <even?>:
    14a0:	55                   	push   rbp
    14a1:	48 89 e5             	mov    rbp,rsp
    14a4:	48 83 ec 10          	sub    rsp,0x10
    14a8:	4c 89 2c 24          	mov    QWORD PTR [rsp],r13
    14ac:	4c 89 7c 24 08       	mov    QWORD PTR [rsp+0x8],r15
    14b1:	49 89 f5             	mov    r13,rsi
    14b4:	49 89 d7             	mov    r15,rdx
    14b7:	bf 01 00 00 00       	mov    edi,0x1
    14bc:	4c 8d 0d d1 56 00 00 	lea    r9,[rip+0x56d1]        # 6b94 <cljn_gc_enter>
    14c3:	41 ff d1             	call   r9
    14c6:	4c 89 ea             	mov    rdx,r13
    14c9:	48 83 fa 01          	cmp    rdx,0x1
    14cd:	0f 84 2b 00 00 00    	je     14fe <even?+0x5e>
    14d3:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    14da:	4c 8d 1d 9b 63 00 00 	lea    r11,[rip+0x639b]        # 787c <cljn_check_arity>
    14e1:	48 89 d7             	mov    rdi,rdx
    14e4:	41 ff d3             	call   r11
    14e7:	b8 02 00 00 00       	mov    eax,0x2
    14ec:	4c 8b 2c 24          	mov    r13,QWORD PTR [rsp]
    14f0:	4c 8b 7c 24 08       	mov    r15,QWORD PTR [rsp+0x8]
    14f5:	48 83 c4 10          	add    rsp,0x10
    14f9:	48 89 ec             	mov    rsp,rbp
    14fc:	5d                   	pop    rbp
    14fd:	c3                   	ret
    14fe:	4c 89 fa             	mov    rdx,r15
    1501:	48 8b 3a             	mov    rdi,QWORD PTR [rdx]
    1504:	48 8d 08             	lea    rcx,[rax]
    1507:	49 89 c7             	mov    r15,rax
    150a:	48 8d 05 6f 2b 01 00 	lea    rax,[rip+0x12b6f]        # 14080 <gc_stack>
    1511:	48 6b c9 08          	imul   rcx,rcx,0x8
    1515:	48 89 3c 08          	mov    QWORD PTR [rax+rcx*1],rdi
    1519:	be 05 00 00 00       	mov    esi,0x5
    151e:	48 89 f9             	mov    rcx,rdi
    1521:	48 83 e1 05          	and    rcx,0x5
    1525:	48 f7 c1 01 00 00 00 	test   rcx,0x1
    152c:	0f 84 59 00 00 00    	je     158b <even?+0xeb>
    1532:	48 89 f8             	mov    rax,rdi
    1535:	48 d1 f8             	sar    rax,1
    1538:	49 89 f1             	mov    r9,rsi
    153b:	49 d1 f9             	sar    r9,1
    153e:	4d 85 c9             	test   r9,r9
    1541:	0f 84 44 00 00 00    	je     158b <even?+0xeb>
    1547:	48 99                	cqo
    1549:	49 83 f9 ff          	cmp    r9,0xffffffffffffffff
    154d:	0f 85 0a 00 00 00    	jne    155d <even?+0xbd>
    1553:	ba 00 00 00 00       	mov    edx,0x0
    1558:	e9 03 00 00 00       	jmp    1560 <even?+0xc0>
    155d:	49 f7 f9             	idiv   r9
    1560:	48 85 d2             	test   rdx,rdx
    1563:	40 0f 95 c6          	setne  sil
    1567:	49 89 d3             	mov    r11,rdx
    156a:	4d 33 d9             	xor    r11,r9
    156d:	49 c1 eb 3f          	shr    r11,0x3f
    1571:	4a 8d 3c 0a          	lea    rdi,[rdx+r9*1]
    1575:	41 84 f3             	test   r11b,sil
    1578:	48 0f 45 d7          	cmovne rdx,rdi
    157c:	48 d1 e2             	shl    rdx,1
    157f:	48 83 ca 01          	or     rdx,0x1
    1583:	48 89 d7             	mov    rdi,rdx
    1586:	e9 0c 00 00 00       	jmp    1597 <even?+0xf7>
    158b:	48 8d 05 4a bc 00 00 	lea    rax,[rip+0xbc4a]        # d1dc <cljn_mod>
    1592:	ff d0                	call   rax
    1594:	48 89 c7             	mov    rdi,rax
    1597:	be 01 00 00 00       	mov    esi,0x1
    159c:	48 8d 0d ac c6 00 00 	lea    rcx,[rip+0xc6ac]        # dc4f <cljn_eq>
    15a3:	ff d1                	call   rcx
    15a5:	49 89 c5             	mov    r13,rax
    15a8:	48 8d 0d 9b 56 00 00 	lea    rcx,[rip+0x569b]        # 6c4a <cljn_gc_leave>
    15af:	4c 89 ff             	mov    rdi,r15
    15b2:	ff d1                	call   rcx
    15b4:	4c 89 e8             	mov    rax,r13
    15b7:	4c 8b 2c 24          	mov    r13,QWORD PTR [rsp]
    15bb:	4c 8b 7c 24 08       	mov    r15,QWORD PTR [rsp+0x8]
    15c0:	48 83 c4 10          	add    rsp,0x10
    15c4:	48 89 ec             	mov    rsp,rbp
    15c7:	5d                   	pop    rbp
    15c8:	c3                   	ret

00000000000015c9 <odd?>:
    15c9:	55                   	push   rbp
    15ca:	48 89 e5             	mov    rbp,rsp
    15cd:	48 83 ec 20          	sub    rsp,0x20
    15d1:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    15d5:	4c 89 6c 24 08       	mov    QWORD PTR [rsp+0x8],r13
    15da:	4c 89 74 24 10       	mov    QWORD PTR [rsp+0x10],r14
    15df:	4c 89 7c 24 18       	mov    QWORD PTR [rsp+0x18],r15
    15e4:	49 89 f5             	mov    r13,rsi
    15e7:	49 89 d7             	mov    r15,rdx
    15ea:	bf 01 00 00 00       	mov    edi,0x1
    15ef:	4c 8d 05 9e 55 00 00 	lea    r8,[rip+0x559e]        # 6b94 <cljn_gc_enter>
    15f6:	41 ff d0             	call   r8
    15f9:	4c 89 ea             	mov    rdx,r13
    15fc:	48 83 fa 01          	cmp    rdx,0x1
    1600:	0f 84 35 00 00 00    	je     163b <odd?+0x72>
    1606:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    160d:	4c 8d 15 68 62 00 00 	lea    r10,[rip+0x6268]        # 787c <cljn_check_arity>
    1614:	48 89 d7             	mov    rdi,rdx
    1617:	41 ff d2             	call   r10
    161a:	b8 02 00 00 00       	mov    eax,0x2
    161f:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    1623:	4c 8b 6c 24 08       	mov    r13,QWORD PTR [rsp+0x8]
    1628:	4c 8b 74 24 10       	mov    r14,QWORD PTR [rsp+0x10]
    162d:	4c 8b 7c 24 18       	mov    r15,QWORD PTR [rsp+0x18]
    1632:	48 83 c4 20          	add    rsp,0x20
    1636:	48 89 ec             	mov    rsp,rbp
    1639:	5d                   	pop    rbp
    163a:	c3                   	ret
    163b:	4c 89 fa             	mov    rdx,r15
    163e:	48 8b 32             	mov    rsi,QWORD PTR [rdx]
    1641:	48 8d 38             	lea    rdi,[rax]
    1644:	49 89 c6             	mov    r14,rax
    1647:	48 8d 05 32 2a 01 00 	lea    rax,[rip+0x12a32]        # 14080 <gc_stack>
    164e:	48 6b ff 08          	imul   rdi,rdi,0x8
    1652:	48 89 34 38          	mov    QWORD PTR [rax+rdi*1],rsi
    1656:	48 8d 05 23 2a 01 02 	lea    rax,[rip+0x2012a23]        # 2014080 <gc_sp>
    165d:	48 8b 08             	mov    rcx,QWORD PTR [rax]
    1660:	48 8d 15 19 2a 01 00 	lea    rdx,[rip+0x12a19]        # 14080 <gc_stack>
    1667:	4c 6b c1 08          	imul   r8,rcx,0x8
    166b:	4a 89 34 02          	mov    QWORD PTR [rdx+r8*1],rsi
    166f:	48 81 c1 01 00 00 00 	add    rcx,0x1
    1676:	48 89 08             	mov    QWORD PTR [rax],rcx
    1679:	bb 01 00 00 00       	mov    ebx,0x1
    167e:	48 8d 15 ca 61 00 00 	lea    rdx,[rip+0x61ca]        # 784f <cljn_argv>
    1685:	48 89 df             	mov    rdi,rbx
    1688:	ff d2                	call   rdx
    168a:	bf 02 00 00 00       	mov    edi,0x2
    168f:	48 89 c2             	mov    rdx,rax
    1692:	48 89 de             	mov    rsi,rbx
    1695:	e8 06 fe ff ff       	call   14a0 <even?>
    169a:	48 8d 15 df 29 01 02 	lea    rdx,[rip+0x20129df]        # 2014080 <gc_sp>
    16a1:	48 83 02 ff          	add    QWORD PTR [rdx],0xffffffffffffffff
    16a5:	4c 8d 05 d4 29 01 02 	lea    r8,[rip+0x20129d4]        # 2014080 <gc_sp>
    16ac:	4d 8b 08             	mov    r9,QWORD PTR [r8]
    16af:	4c 8d 15 ca 29 01 00 	lea    r10,[rip+0x129ca]        # 14080 <gc_stack>
    16b6:	4d 6b d9 08          	imul   r11,r9,0x8
    16ba:	4b 89 04 1a          	mov    QWORD PTR [r10+r11*1],rax
    16be:	49 81 c1 01 00 00 00 	add    r9,0x1
    16c5:	4d 89 08             	mov    QWORD PTR [r8],r9
    16c8:	4c 8d 15 d8 c5 00 00 	lea    r10,[rip+0xc5d8]        # dca7 <cljn_not>
    16cf:	48 89 c7             	mov    rdi,rax
    16d2:	41 ff d2             	call   r10
    16d5:	49 89 c5             	mov    r13,rax
    16d8:	4c 8d 15 a1 29 01 02 	lea    r10,[rip+0x20129a1]        # 2014080 <gc_sp>
    16df:	49 83 02 ff          	add    QWORD PTR [r10],0xffffffffffffffff
    16e3:	4c 8d 1d 60 55 00 00 	lea    r11,[rip+0x5560]        # 6c4a <cljn_gc_leave>
    16ea:	4c 89 f7             	mov    rdi,r14
    16ed:	41 ff d3             	call   r11
    16f0:	4c 89 e8             	mov    rax,r13
    16f3:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    16f7:	4c 8b 6c 24 08       	mov    r13,QWORD PTR [rsp+0x8]
    16fc:	4c 8b 74 24 10       	mov    r14,QWORD PTR [rsp+0x10]
    1701:	4c 8b 7c 24 18       	mov    r15,QWORD PTR [rsp+0x18]
    1706:	48 83 c4 20          	add    rsp,0x20
    170a:	48 89 ec             	mov    rsp,rbp
    170d:	5d                   	pop    rbp
    170e:	c3                   	ret
	...

0000000000001710 <__lambda_0>:
    1710:	55                   	push   rbp
    1711:	48 89 e5             	mov    rbp,rsp
    1714:	48 83 ec 30          	sub    rsp,0x30
    1718:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    171c:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    1721:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    1726:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    172b:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
    1730:	49 89 f4             	mov    r12,rsi
    1733:	49 89 d6             	mov    r14,rdx
    1736:	bf 02 00 00 00       	mov    edi,0x2
    173b:	48 8d 05 52 54 00 00 	lea    rax,[rip+0x5452]        # 6b94 <cljn_gc_enter>
    1742:	ff d0                	call   rax
    1744:	4c 89 e2             	mov    rdx,r12
    1747:	48 83 fa 02          	cmp    rdx,0x2
    174b:	0f 84 3a 00 00 00    	je     178b <__lambda_0+0x7b>
    1751:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    1758:	4c 8d 05 1d 61 00 00 	lea    r8,[rip+0x611d]        # 787c <cljn_check_arity>
    175f:	48 89 d7             	mov    rdi,rdx
    1762:	41 ff d0             	call   r8
    1765:	b8 02 00 00 00       	mov    eax,0x2
    176a:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    176e:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    1773:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    1778:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    177d:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    1782:	48 83 c4 30          	add    rsp,0x30
    1786:	48 89 ec             	mov    rsp,rbp
    1789:	5d                   	pop    rbp
    178a:	c3                   	ret
    178b:	4c 89 f2             	mov    rdx,r14
    178e:	4c 8b 2a             	mov    r13,QWORD PTR [rdx]
    1791:	4c 8d 08             	lea    r9,[rax]
    1794:	4c 8d 15 e5 28 01 00 	lea    r10,[rip+0x128e5]        # 14080 <gc_stack>
    179b:	4d 6b c9 08          	imul   r9,r9,0x8
    179f:	4f 89 2c 0a          	mov    QWORD PTR [r10+r9*1],r13
    17a3:	4c 8b 7a 08          	mov    r15,QWORD PTR [rdx+0x8]
    17a7:	4c 8d 50 01          	lea    r10,[rax+0x1]
    17ab:	48 89 c3             	mov    rbx,rax
    17ae:	4c 8d 1d cb 28 01 00 	lea    r11,[rip+0x128cb]        # 14080 <gc_stack>
    17b5:	4d 6b d2 08          	imul   r10,r10,0x8
    17b9:	4f 89 3c 13          	mov    QWORD PTR [r11+r10*1],r15
    17bd:	4c 89 ee             	mov    rsi,r13
    17c0:	49 23 f7             	and    rsi,r15
    17c3:	48 f7 c6 01 00 00 00 	test   rsi,0x1
    17ca:	0f 85 14 00 00 00    	jne    17e4 <__lambda_0+0xd4>
    17d0:	48 8d 05 94 bc 00 00 	lea    rax,[rip+0xbc94]        # d46b <cljn_gt>
    17d7:	4c 89 fe             	mov    rsi,r15
    17da:	4c 89 ef             	mov    rdi,r13
    17dd:	ff d0                	call   rax
    17df:	e9 1c 00 00 00       	jmp    1800 <__lambda_0+0xf0>
    17e4:	4c 89 e9             	mov    rcx,r13
    17e7:	48 d1 f9             	sar    rcx,1
    17ea:	4c 89 fa             	mov    rdx,r15
    17ed:	48 d1 fa             	sar    rdx,1
    17f0:	b8 06 00 00 00       	mov    eax,0x6
    17f5:	48 3b ca             	cmp    rcx,rdx
    17f8:	48 0f 4f 05 a0 00 00 	cmovg  rax,QWORD PTR [rip+0xa0]        # 18a0 <__lambda_0+0x190>
    17ff:	00 
    1800:	48 83 f8 06          	cmp    rax,0x6
    1804:	41 0f 95 c1          	setne  r9b
    1808:	48 83 f8 02          	cmp    rax,0x2
    180c:	41 0f 95 c2          	setne  r10b
    1810:	45 84 ca             	test   r10b,r9b
    1813:	0f 85 2b 00 00 00    	jne    1844 <__lambda_0+0x134>
    1819:	48 8d 35 60 28 01 02 	lea    rsi,[rip+0x2012860]        # 2014080 <gc_sp>
    1820:	48 8b 3e             	mov    rdi,QWORD PTR [rsi]
    1823:	48 8d 05 56 28 01 00 	lea    rax,[rip+0x12856]        # 14080 <gc_stack>
    182a:	48 6b cf 08          	imul   rcx,rdi,0x8
    182e:	4c 89 3c 08          	mov    QWORD PTR [rax+rcx*1],r15
    1832:	4d 89 fc             	mov    r12,r15
    1835:	48 81 c7 01 00 00 00 	add    rdi,0x1
    183c:	48 89 3e             	mov    QWORD PTR [rsi],rdi
    183f:	e9 26 00 00 00       	jmp    186a <__lambda_0+0x15a>
    1844:	48 8d 0d 35 28 01 02 	lea    rcx,[rip+0x2012835]        # 2014080 <gc_sp>
    184b:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    184e:	4c 8d 05 2b 28 01 00 	lea    r8,[rip+0x1282b]        # 14080 <gc_stack>
    1855:	4c 6b ca 08          	imul   r9,rdx,0x8
    1859:	4f 89 2c 08          	mov    QWORD PTR [r8+r9*1],r13
    185d:	48 81 c2 01 00 00 00 	add    rdx,0x1
    1864:	48 89 11             	mov    QWORD PTR [rcx],rdx
    1867:	4d 89 ec             	mov    r12,r13
    186a:	4c 8d 0d d9 53 00 00 	lea    r9,[rip+0x53d9]        # 6c4a <cljn_gc_leave>
    1871:	48 89 df             	mov    rdi,rbx
    1874:	41 ff d1             	call   r9
    1877:	4c 89 e0             	mov    rax,r12
    187a:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    187e:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    1883:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    1888:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    188d:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    1892:	48 83 c4 30          	add    rsp,0x30
    1896:	48 89 ec             	mov    rsp,rbp
    1899:	5d                   	pop    rbp
    189a:	c3                   	ret
    189b:	00 00                	add    BYTE PTR [rax],al
    189d:	00 00                	add    BYTE PTR [rax],al
    189f:	00 0a                	add    BYTE PTR [rdx],cl
    18a1:	00 00                	add    BYTE PTR [rax],al
    18a3:	00 00                	add    BYTE PTR [rax],al
    18a5:	00 00                	add    BYTE PTR [rax],al
	...

00000000000018a8 <max>:
    18a8:	55                   	push   rbp
    18a9:	48 89 e5             	mov    rbp,rsp
    18ac:	48 83 ec 30          	sub    rsp,0x30
    18b0:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    18b4:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    18b9:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    18be:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    18c3:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
    18c8:	49 89 f4             	mov    r12,rsi
    18cb:	49 89 d6             	mov    r14,rdx
    18ce:	bf 02 00 00 00       	mov    edi,0x2
    18d3:	4c 8d 0d ba 52 00 00 	lea    r9,[rip+0x52ba]        # 6b94 <cljn_gc_enter>
    18da:	41 ff d1             	call   r9
    18dd:	4c 89 e6             	mov    rsi,r12
    18e0:	48 83 fe 01          	cmp    rsi,0x1
    18e4:	0f 8d 3a 00 00 00    	jge    1924 <max+0x7c>
    18ea:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    18f1:	4c 8d 1d 84 5f 00 00 	lea    r11,[rip+0x5f84]        # 787c <cljn_check_arity>
    18f8:	4c 89 e7             	mov    rdi,r12
    18fb:	41 ff d3             	call   r11
    18fe:	b8 02 00 00 00       	mov    eax,0x2
    1903:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    1907:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    190c:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    1911:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    1916:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    191b:	48 83 c4 30          	add    rsp,0x30
    191f:	48 89 ec             	mov    rsp,rbp
    1922:	5d                   	pop    rbp
    1923:	c3                   	ret
    1924:	4c 89 f2             	mov    rdx,r14
    1927:	4c 89 e7             	mov    rdi,r12
    192a:	4c 8b 22             	mov    r12,QWORD PTR [rdx]
    192d:	48 8d 08             	lea    rcx,[rax]
    1930:	49 89 c5             	mov    r13,rax
    1933:	48 8d 15 46 27 01 00 	lea    rdx,[rip+0x12746]        # 14080 <gc_stack>
    193a:	48 6b c9 08          	imul   rcx,rcx,0x8
    193e:	4c 89 24 0a          	mov    QWORD PTR [rdx+rcx*1],r12
    1942:	ba 01 00 00 00       	mov    edx,0x1
    1947:	48 8d 0d ca 5f 00 00 	lea    rcx,[rip+0x5fca]        # 7918 <cljn_collect_rest>
    194e:	4c 89 f6             	mov    rsi,r14
    1951:	ff d1                	call   rcx
    1953:	48 89 c6             	mov    rsi,rax
    1956:	4c 89 e8             	mov    rax,r13
    1959:	48 8d 48 01          	lea    rcx,[rax+0x1]
    195d:	48 8d 05 1c 27 01 00 	lea    rax,[rip+0x1271c]        # 14080 <gc_stack>
    1964:	48 6b c9 08          	imul   rcx,rcx,0x8
    1968:	48 89 f3             	mov    rbx,rsi
    196b:	48 89 1c 08          	mov    QWORD PTR [rax+rcx*1],rbx
    196f:	48 8d 3d 9a fd ff ff 	lea    rdi,[rip+0xfffffffffffffd9a]        # 1710 <__lambda_0>
    1976:	be 02 00 00 00       	mov    esi,0x2
    197b:	48 33 d2             	xor    rdx,rdx
    197e:	4c 8d 05 a1 5d 00 00 	lea    r8,[rip+0x5da1]        # 7726 <cljn_make_fn>
    1985:	41 ff d0             	call   r8
    1988:	48 8d 15 f1 26 01 02 	lea    rdx,[rip+0x20126f1]        # 2014080 <gc_sp>
    198f:	4c 8b 02             	mov    r8,QWORD PTR [rdx]
    1992:	4c 8d 0d e7 26 01 00 	lea    r9,[rip+0x126e7]        # 14080 <gc_stack>
    1999:	4d 6b d0 08          	imul   r10,r8,0x8
    199d:	4b 89 04 11          	mov    QWORD PTR [r9+r10*1],rax
    19a1:	49 81 c0 01 00 00 00 	add    r8,0x1
    19a8:	4c 89 02             	mov    QWORD PTR [rdx],r8
    19ab:	4c 8d 0d ce 26 01 02 	lea    r9,[rip+0x20126ce]        # 2014080 <gc_sp>
    19b2:	4d 8b 11             	mov    r10,QWORD PTR [r9]
    19b5:	4c 8d 1d c4 26 01 00 	lea    r11,[rip+0x126c4]        # 14080 <gc_stack>
    19bc:	49 6b f2 08          	imul   rsi,r10,0x8
    19c0:	4d 89 24 33          	mov    QWORD PTR [r11+rsi*1],r12
    19c4:	49 81 c2 01 00 00 00 	add    r10,0x1
    19cb:	4d 89 11             	mov    QWORD PTR [r9],r10
    19ce:	4c 8d 1d ab 26 01 02 	lea    r11,[rip+0x20126ab]        # 2014080 <gc_sp>
    19d5:	49 8b 33             	mov    rsi,QWORD PTR [r11]
    19d8:	48 8d 3d a1 26 01 00 	lea    rdi,[rip+0x126a1]        # 14080 <gc_stack>
    19df:	48 6b c6 08          	imul   rax,rsi,0x8
    19e3:	48 89 1c 07          	mov    QWORD PTR [rdi+rax*1],rbx
    19e7:	48 81 c6 01 00 00 00 	add    rsi,0x1
    19ee:	49 89 33             	mov    QWORD PTR [r11],rsi
    19f1:	41 bf 03 00 00 00    	mov    r15d,0x3
    19f7:	48 8d 05 51 5e 00 00 	lea    rax,[rip+0x5e51]        # 784f <cljn_argv>
    19fe:	4c 89 ff             	mov    rdi,r15
    1a01:	ff d0                	call   rax
    1a03:	bf 02 00 00 00       	mov    edi,0x2
    1a08:	48 89 c2             	mov    rdx,rax
    1a0b:	4c 89 fe             	mov    rsi,r15
    1a0e:	e8 c9 03 00 00       	call   1ddc <reduce>
    1a13:	48 8d 3d 66 26 01 02 	lea    rdi,[rip+0x2012666]        # 2014080 <gc_sp>
    1a1a:	48 83 07 fd          	add    QWORD PTR [rdi],0xfffffffffffffffd
    1a1e:	48 8d 0d 5b 26 01 02 	lea    rcx,[rip+0x201265b]        # 2014080 <gc_sp>
    1a25:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    1a28:	4c 8d 05 51 26 01 00 	lea    r8,[rip+0x12651]        # 14080 <gc_stack>
    1a2f:	4c 6b ca 08          	imul   r9,rdx,0x8
    1a33:	4b 89 04 08          	mov    QWORD PTR [r8+r9*1],rax
    1a37:	49 89 c4             	mov    r12,rax
    1a3a:	48 81 c2 01 00 00 00 	add    rdx,0x1
    1a41:	48 89 11             	mov    QWORD PTR [rcx],rdx
    1a44:	48 8d 15 ff 51 00 00 	lea    rdx,[rip+0x51ff]        # 6c4a <cljn_gc_leave>
    1a4b:	4c 89 ef             	mov    rdi,r13
    1a4e:	ff d2                	call   rdx
    1a50:	4c 89 e0             	mov    rax,r12
    1a53:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    1a57:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    1a5c:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    1a61:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    1a66:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    1a6b:	48 83 c4 30          	add    rsp,0x30
    1a6f:	48 89 ec             	mov    rsp,rbp
    1a72:	5d                   	pop    rbp
    1a73:	c3                   	ret
    1a74:	00 00                	add    BYTE PTR [rax],al
	...

0000000000001a78 <__lambda_1>:
    1a78:	55                   	push   rbp
    1a79:	48 89 e5             	mov    rbp,rsp
    1a7c:	48 83 ec 30          	sub    rsp,0x30
    1a80:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    1a84:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    1a89:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    1a8e:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    1a93:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
    1a98:	49 89 f4             	mov    r12,rsi
    1a9b:	49 89 d6             	mov    r14,rdx
    1a9e:	bf 02 00 00 00       	mov    edi,0x2
    1aa3:	48 8d 05 ea 50 00 00 	lea    rax,[rip+0x50ea]        # 6b94 <cljn_gc_enter>
    1aaa:	ff d0                	call   rax
    1aac:	4c 89 e2             	mov    rdx,r12
    1aaf:	48 83 fa 02          	cmp    rdx,0x2
    1ab3:	0f 84 3a 00 00 00    	je     1af3 <__lambda_1+0x7b>
    1ab9:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    1ac0:	4c 8d 05 b5 5d 00 00 	lea    r8,[rip+0x5db5]        # 787c <cljn_check_arity>
    1ac7:	48 89 d7             	mov    rdi,rdx
    1aca:	41 ff d0             	call   r8
    1acd:	b8 02 00 00 00       	mov    eax,0x2
    1ad2:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    1ad6:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    1adb:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    1ae0:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    1ae5:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    1aea:	48 83 c4 30          	add    rsp,0x30
    1aee:	48 89 ec             	mov    rsp,rbp
    1af1:	5d                   	pop    rbp
    1af2:	c3                   	ret
    1af3:	4c 89 f2             	mov    rdx,r14
    1af6:	4c 8b 2a             	mov    r13,QWORD PTR [rdx]
    1af9:	4c 8d 08             	lea    r9,[rax]
    1afc:	4c 8d 15 7d 25 01 00 	lea    r10,[rip+0x1257d]        # 14080 <gc_stack>
    1b03:	4d 6b c9 08          	imul   r9,r9,0x8
    1b07:	4f 89 2c 0a          	mov    QWORD PTR [r10+r9*1],r13
    1b0b:	4c 8b 7a 08          	mov    r15,QWORD PTR [rdx+0x8]
    1b0f:	4c 8d 50 01          	lea    r10,[rax+0x1]
    1b13:	48 89 c3             	mov    rbx,rax
    1b16:	4c 8d 1d 63 25 01 00 	lea    r11,[rip+0x12563]        # 14080 <gc_stack>
    1b1d:	4d 6b d2 08          	imul   r10,r10,0x8
    1b21:	4f 89 3c 13          	mov    QWORD PTR [r11+r10*1],r15
    1b25:	4c 89 ee             	mov    rsi,r13
    1b28:	49 23 f7             	and    rsi,r15
    1b2b:	48 f7 c6 01 00 00 00 	test   rsi,0x1
    1b32:	0f 85 14 00 00 00    	jne    1b4c <__lambda_1+0xd4>
    1b38:	48 8d 05 78 b8 00 00 	lea    rax,[rip+0xb878]        # d3b7 <cljn_lt>
    1b3f:	4c 89 fe             	mov    rsi,r15
    1b42:	4c 89 ef             	mov    rdi,r13
    1b45:	ff d0                	call   rax
    1b47:	e9 1c 00 00 00       	jmp    1b68 <__lambda_1+0xf0>
    1b4c:	4c 89 e9             	mov    rcx,r13
    1b4f:	48 d1 f9             	sar    rcx,1
    1b52:	4c 89 fa             	mov    rdx,r15
    1b55:	48 d1 fa             	sar    rdx,1
    1b58:	b8 06 00 00 00       	mov    eax,0x6
    1b5d:	48 3b ca             	cmp    rcx,rdx
    1b60:	48 0f 4c 05 a0 00 00 	cmovl  rax,QWORD PTR [rip+0xa0]        # 1c08 <__lambda_1+0x190>
    1b67:	00 
    1b68:	48 83 f8 06          	cmp    rax,0x6
    1b6c:	41 0f 95 c1          	setne  r9b
    1b70:	48 83 f8 02          	cmp    rax,0x2
    1b74:	41 0f 95 c2          	setne  r10b
    1b78:	45 84 ca             	test   r10b,r9b
    1b7b:	0f 85 2b 00 00 00    	jne    1bac <__lambda_1+0x134>
    1b81:	48 8d 35 f8 24 01 02 	lea    rsi,[rip+0x20124f8]        # 2014080 <gc_sp>
    1b88:	48 8b 3e             	mov    rdi,QWORD PTR [rsi]
    1b8b:	48 8d 05 ee 24 01 00 	lea    rax,[rip+0x124ee]        # 14080 <gc_stack>
    1b92:	48 6b cf 08          	imul   rcx,rdi,0x8
    1b96:	4c 89 3c 08          	mov    QWORD PTR [rax+rcx*1],r15
    1b9a:	4d 89 fc             	mov    r12,r15
    1b9d:	48 81 c7 01 00 00 00 	add    rdi,0x1
    1ba4:	48 89 3e             	mov    QWORD PTR [rsi],rdi
    1ba7:	e9 26 00 00 00       	jmp    1bd2 <__lambda_1+0x15a>
    1bac:	48 8d 0d cd 24 01 02 	lea    rcx,[rip+0x20124cd]        # 2014080 <gc_sp>
    1bb3:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    1bb6:	4c 8d 05 c3 24 01 00 	lea    r8,[rip+0x124c3]        # 14080 <gc_stack>
    1bbd:	4c 6b ca 08          	imul   r9,rdx,0x8
    1bc1:	4f 89 2c 08          	mov    QWORD PTR [r8+r9*1],r13
    1bc5:	48 81 c2 01 00 00 00 	add    rdx,0x1
    1bcc:	48 89 11             	mov    QWORD PTR [rcx],rdx
    1bcf:	4d 89 ec             	mov    r12,r13
    1bd2:	4c 8d 0d 71 50 00 00 	lea    r9,[rip+0x5071]        # 6c4a <cljn_gc_leave>
    1bd9:	48 89 df             	mov    rdi,rbx
    1bdc:	41 ff d1             	call   r9
    1bdf:	4c 89 e0             	mov    rax,r12
    1be2:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    1be6:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    1beb:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    1bf0:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    1bf5:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    1bfa:	48 83 c4 30          	add    rsp,0x30
    1bfe:	48 89 ec             	mov    rsp,rbp
    1c01:	5d                   	pop    rbp
    1c02:	c3                   	ret
    1c03:	00 00                	add    BYTE PTR [rax],al
    1c05:	00 00                	add    BYTE PTR [rax],al
    1c07:	00 0a                	add    BYTE PTR [rdx],cl
    1c09:	00 00                	add    BYTE PTR [rax],al
    1c0b:	00 00                	add    BYTE PTR [rax],al
    1c0d:	00 00                	add    BYTE PTR [rax],al
	...

0000000000001c10 <min>:
    1c10:	55                   	push   rbp
    1c11:	48 89 e5             	mov    rbp,rsp
    1c14:	48 83 ec 30          	sub    rsp,0x30
    1c18:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    1c1c:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    1c21:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    1c26:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    1c2b:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
    1c30:	49 89 f4             	mov    r12,rsi
    1c33:	49 89 d6             	mov    r14,rdx
    1c36:	bf 02 00 00 00       	mov    edi,0x2
    1c3b:	4c 8d 0d 52 4f 00 00 	lea    r9,[rip+0x4f52]        # 6b94 <cljn_gc_enter>
    1c42:	41 ff d1             	call   r9
    1c45:	4c 89 e6             	mov    rsi,r12
    1c48:	48 83 fe 01          	cmp    rsi,0x1
    1c4c:	0f 8d 3a 00 00 00    	jge    1c8c <min+0x7c>
    1c52:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    1c59:	4c 8d 1d 1c 5c 00 00 	lea    r11,[rip+0x5c1c]        # 787c <cljn_check_arity>
    1c60:	4c 89 e7             	mov    rdi,r12
    1c63:	41 ff d3             	call   r11
    1c66:	b8 02 00 00 00       	mov    eax,0x2
    1c6b:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    1c6f:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    1c74:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    1c79:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    1c7e:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    1c83:	48 83 c4 30          	add    rsp,0x30
    1c87:	48 89 ec             	mov    rsp,rbp
    1c8a:	5d                   	pop    rbp
    1c8b:	c3                   	ret
    1c8c:	4c 89 f2             	mov    rdx,r14
    1c8f:	4c 89 e7             	mov    rdi,r12
    1c92:	4c 8b 22             	mov    r12,QWORD PTR [rdx]
    1c95:	48 8d 08             	lea    rcx,[rax]
    1c98:	49 89 c5             	mov    r13,rax
    1c9b:	48 8d 15 de 23 01 00 	lea    rdx,[rip+0x123de]        # 14080 <gc_stack>
    1ca2:	48 6b c9 08          	imul   rcx,rcx,0x8
    1ca6:	4c 89 24 0a          	mov    QWORD PTR [rdx+rcx*1],r12
    1caa:	ba 01 00 00 00       	mov    edx,0x1
    1caf:	48 8d 0d 62 5c 00 00 	lea    rcx,[rip+0x5c62]        # 7918 <cljn_collect_rest>
    1cb6:	4c 89 f6             	mov    rsi,r14
    1cb9:	ff d1                	call   rcx
    1cbb:	48 89 c6             	mov    rsi,rax
    1cbe:	4c 89 e8             	mov    rax,r13
    1cc1:	48 8d 48 01          	lea    rcx,[rax+0x1]
    1cc5:	48 8d 05 b4 23 01 00 	lea    rax,[rip+0x123b4]        # 14080 <gc_stack>
    1ccc:	48 6b c9 08          	imul   rcx,rcx,0x8
    1cd0:	48 89 f3             	mov    rbx,rsi
    1cd3:	48 89 1c 08          	mov    QWORD PTR [rax+rcx*1],rbx
    1cd7:	48 8d 3d 9a fd ff ff 	lea    rdi,[rip+0xfffffffffffffd9a]        # 1a78 <__lambda_1>
    1cde:	be 02 00 00 00       	mov    esi,0x2
    1ce3:	48 33 d2             	xor    rdx,rdx
    1ce6:	4c 8d 05 39 5a 00 00 	lea    r8,[rip+0x5a39]        # 7726 <cljn_make_fn>
    1ced:	41 ff d0             	call   r8
    1cf0:	48 8d 15 89 23 01 02 	lea    rdx,[rip+0x2012389]        # 2014080 <gc_sp>
    1cf7:	4c 8b 02             	mov    r8,QWORD PTR [rdx]
    1cfa:	4c 8d 0d 7f 23 01 00 	lea    r9,[rip+0x1237f]        # 14080 <gc_stack>
    1d01:	4d 6b d0 08          	imul   r10,r8,0x8
    1d05:	4b 89 04 11          	mov    QWORD PTR [r9+r10*1],rax
    1d09:	49 81 c0 01 00 00 00 	add    r8,0x1
    1d10:	4c 89 02             	mov    QWORD PTR [rdx],r8
    1d13:	4c 8d 0d 66 23 01 02 	lea    r9,[rip+0x2012366]        # 2014080 <gc_sp>
    1d1a:	4d 8b 11             	mov    r10,QWORD PTR [r9]
    1d1d:	4c 8d 1d 5c 23 01 00 	lea    r11,[rip+0x1235c]        # 14080 <gc_stack>
    1d24:	49 6b f2 08          	imul   rsi,r10,0x8
    1d28:	4d 89 24 33          	mov    QWORD PTR [r11+rsi*1],r12
    1d2c:	49 81 c2 01 00 00 00 	add    r10,0x1
    1d33:	4d 89 11             	mov    QWORD PTR [r9],r10
    1d36:	4c 8d 1d 43 23 01 02 	lea    r11,[rip+0x2012343]        # 2014080 <gc_sp>
    1d3d:	49 8b 33             	mov    rsi,QWORD PTR [r11]
    1d40:	48 8d 3d 39 23 01 00 	lea    rdi,[rip+0x12339]        # 14080 <gc_stack>
    1d47:	48 6b c6 08          	imul   rax,rsi,0x8
    1d4b:	48 89 1c 07          	mov    QWORD PTR [rdi+rax*1],rbx
    1d4f:	48 81 c6 01 00 00 00 	add    rsi,0x1
    1d56:	49 89 33             	mov    QWORD PTR [r11],rsi
    1d59:	41 bf 03 00 00 00    	mov    r15d,0x3
    1d5f:	48 8d 05 e9 5a 00 00 	lea    rax,[rip+0x5ae9]        # 784f <cljn_argv>
    1d66:	4c 89 ff             	mov    rdi,r15
    1d69:	ff d0                	call   rax
    1d6b:	bf 02 00 00 00       	mov    edi,0x2
    1d70:	48 89 c2             	mov    rdx,rax
    1d73:	4c 89 fe             	mov    rsi,r15
    1d76:	e8 61 00 00 00       	call   1ddc <reduce>
    1d7b:	48 8d 3d fe 22 01 02 	lea    rdi,[rip+0x20122fe]        # 2014080 <gc_sp>
    1d82:	48 83 07 fd          	add    QWORD PTR [rdi],0xfffffffffffffffd
    1d86:	48 8d 0d f3 22 01 02 	lea    rcx,[rip+0x20122f3]        # 2014080 <gc_sp>
    1d8d:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    1d90:	4c 8d 05 e9 22 01 00 	lea    r8,[rip+0x122e9]        # 14080 <gc_stack>
    1d97:	4c 6b ca 08          	imul   r9,rdx,0x8
    1d9b:	4b 89 04 08          	mov    QWORD PTR [r8+r9*1],rax
    1d9f:	49 89 c4             	mov    r12,rax
    1da2:	48 81 c2 01 00 00 00 	add    rdx,0x1
    1da9:	48 89 11             	mov    QWORD PTR [rcx],rdx
    1dac:	48 8d 15 97 4e 00 00 	lea    rdx,[rip+0x4e97]        # 6c4a <cljn_gc_leave>
    1db3:	4c 89 ef             	mov    rdi,r13
    1db6:	ff d2                	call   rdx
    1db8:	4c 89 e0             	mov    rax,r12
    1dbb:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    1dbf:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    1dc4:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    1dc9:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    1dce:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    1dd3:	48 83 c4 30          	add    rsp,0x30
    1dd7:	48 89 ec             	mov    rsp,rbp
    1dda:	5d                   	pop    rbp
    1ddb:	c3                   	ret

0000000000001ddc <reduce>:
    1ddc:	55                   	push   rbp
    1ddd:	48 89 e5             	mov    rbp,rsp
    1de0:	48 83 ec 30          	sub    rsp,0x30
    1de4:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    1de8:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    1ded:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    1df2:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    1df7:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
    1dfc:	48 89 d3             	mov    rbx,rdx
    1dff:	49 89 f6             	mov    r14,rsi
    1e02:	bf 03 00 00 00       	mov    edi,0x3
    1e07:	4c 8d 1d 86 4d 00 00 	lea    r11,[rip+0x4d86]        # 6b94 <cljn_gc_enter>
    1e0e:	41 ff d3             	call   r11
    1e11:	4c 89 f1             	mov    rcx,r14
    1e14:	48 83 f9 03          	cmp    rcx,0x3
    1e18:	0f 84 39 00 00 00    	je     1e57 <reduce+0x7b>
    1e1e:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    1e25:	48 8d 05 50 5a 00 00 	lea    rax,[rip+0x5a50]        # 787c <cljn_check_arity>
    1e2c:	48 89 cf             	mov    rdi,rcx
    1e2f:	ff d0                	call   rax
    1e31:	b8 02 00 00 00       	mov    eax,0x2
    1e36:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    1e3a:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    1e3f:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    1e44:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    1e49:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    1e4e:	48 83 c4 30          	add    rsp,0x30
    1e52:	48 89 ec             	mov    rsp,rbp
    1e55:	5d                   	pop    rbp
    1e56:	c3                   	ret
    1e57:	48 89 da             	mov    rdx,rbx
    1e5a:	4c 8b 3a             	mov    r15,QWORD PTR [rdx]
    1e5d:	48 8d 08             	lea    rcx,[rax]
    1e60:	4c 8d 05 19 22 01 00 	lea    r8,[rip+0x12219]        # 14080 <gc_stack>
    1e67:	48 6b c9 08          	imul   rcx,rcx,0x8
    1e6b:	4d 89 3c 08          	mov    QWORD PTR [r8+rcx*1],r15
    1e6f:	48 8b 4a 08          	mov    rcx,QWORD PTR [rdx+0x8]
    1e73:	4c 8d 40 01          	lea    r8,[rax+0x1]
    1e77:	4c 8d 0d 02 22 01 00 	lea    r9,[rip+0x12202]        # 14080 <gc_stack>
    1e7e:	4d 6b c0 08          	imul   r8,r8,0x8
    1e82:	4b 89 0c 01          	mov    QWORD PTR [r9+r8*1],rcx
    1e86:	48 89 cb             	mov    rbx,rcx
    1e89:	48 8b 7a 10          	mov    rdi,QWORD PTR [rdx+0x10]
    1e8d:	4c 8d 40 02          	lea    r8,[rax+0x2]
    1e91:	49 89 c4             	mov    r12,rax
    1e94:	4c 8d 0d e5 21 01 00 	lea    r9,[rip+0x121e5]        # 14080 <gc_stack>
    1e9b:	4d 6b c0 08          	imul   r8,r8,0x8
    1e9f:	4b 89 3c 01          	mov    QWORD PTR [r9+r8*1],rdi
    1ea3:	49 89 fd             	mov    r13,rdi
    1ea6:	4c 8d 15 4b be 00 00 	lea    r10,[rip+0xbe4b]        # dcf8 <cljn_emptyp>
    1ead:	4c 89 ef             	mov    rdi,r13
    1eb0:	41 ff d2             	call   r10
    1eb3:	48 83 f8 06          	cmp    rax,0x6
    1eb7:	41 0f 95 c3          	setne  r11b
    1ebb:	48 83 f8 02          	cmp    rax,0x2
    1ebf:	40 0f 95 c6          	setne  sil
    1ec3:	44 84 de             	test   sil,r11b
    1ec6:	0f 85 65 01 00 00    	jne    2031 <reduce+0x255>
    1ecc:	48 8d 05 ad 21 01 02 	lea    rax,[rip+0x20121ad]        # 2014080 <gc_sp>
    1ed3:	48 8b 08             	mov    rcx,QWORD PTR [rax]
    1ed6:	48 8d 15 a3 21 01 00 	lea    rdx,[rip+0x121a3]        # 14080 <gc_stack>
    1edd:	4c 6b c1 08          	imul   r8,rcx,0x8
    1ee1:	4e 89 3c 02          	mov    QWORD PTR [rdx+r8*1],r15
    1ee5:	48 81 c1 01 00 00 00 	add    rcx,0x1
    1eec:	48 89 08             	mov    QWORD PTR [rax],rcx
    1eef:	48 8d 15 8a 21 01 02 	lea    rdx,[rip+0x201218a]        # 2014080 <gc_sp>
    1ef6:	4c 8b 02             	mov    r8,QWORD PTR [rdx]
    1ef9:	4c 8d 0d 80 21 01 00 	lea    r9,[rip+0x12180]        # 14080 <gc_stack>
    1f00:	4d 6b d0 08          	imul   r10,r8,0x8
    1f04:	48 89 d8             	mov    rax,rbx
    1f07:	4b 89 04 11          	mov    QWORD PTR [r9+r10*1],rax
    1f0b:	49 81 c0 01 00 00 00 	add    r8,0x1
    1f12:	4c 89 02             	mov    QWORD PTR [rdx],r8
    1f15:	4c 8d 0d ff be 00 00 	lea    r9,[rip+0xbeff]        # de1b <cljn_first>
    1f1c:	4c 89 ef             	mov    rdi,r13
    1f1f:	41 ff d1             	call   r9
    1f22:	4c 8d 0d 57 21 01 02 	lea    r9,[rip+0x2012157]        # 2014080 <gc_sp>
    1f29:	4d 8b 11             	mov    r10,QWORD PTR [r9]
    1f2c:	4c 8d 1d 4d 21 01 00 	lea    r11,[rip+0x1214d]        # 14080 <gc_stack>
    1f33:	49 6b f2 08          	imul   rsi,r10,0x8
    1f37:	49 89 04 33          	mov    QWORD PTR [r11+rsi*1],rax
    1f3b:	49 81 c2 01 00 00 00 	add    r10,0x1
    1f42:	4d 89 11             	mov    QWORD PTR [r9],r10
    1f45:	4c 8d 1d d0 58 00 00 	lea    r11,[rip+0x58d0]        # 781c <cljn_check_fn>
    1f4c:	4c 89 ff             	mov    rdi,r15
    1f4f:	41 ff d3             	call   r11
    1f52:	bb 02 00 00 00       	mov    ebx,0x2
    1f57:	48 8d 35 f1 58 00 00 	lea    rsi,[rip+0x58f1]        # 784f <cljn_argv>
    1f5e:	48 89 df             	mov    rdi,rbx
    1f61:	ff d6                	call   rsi
    1f63:	49 89 c6             	mov    r14,rax
    1f66:	48 8d 35 99 58 00 00 	lea    rsi,[rip+0x5899]        # 7806 <cljn_fn_code>
    1f6d:	4c 89 ff             	mov    rdi,r15
    1f70:	ff d6                	call   rsi
    1f72:	4c 89 f2             	mov    rdx,r14
    1f75:	48 89 de             	mov    rsi,rbx
    1f78:	4c 89 ff             	mov    rdi,r15
    1f7b:	ff d0                	call   rax
    1f7d:	48 89 c3             	mov    rbx,rax
    1f80:	48 8d 35 f9 20 01 02 	lea    rsi,[rip+0x20120f9]        # 2014080 <gc_sp>
    1f87:	48 83 06 fd          	add    QWORD PTR [rsi],0xfffffffffffffffd
    1f8b:	48 8d 3d ee 20 01 02 	lea    rdi,[rip+0x20120ee]        # 2014080 <gc_sp>
    1f92:	48 8b 07             	mov    rax,QWORD PTR [rdi]
    1f95:	48 8d 0d e4 20 01 00 	lea    rcx,[rip+0x120e4]        # 14080 <gc_stack>
    1f9c:	48 6b d0 08          	imul   rdx,rax,0x8
    1fa0:	49 89 d8             	mov    r8,rbx
    1fa3:	4c 89 04 11          	mov    QWORD PTR [rcx+rdx*1],r8
    1fa7:	48 81 c0 01 00 00 00 	add    rax,0x1
    1fae:	48 89 07             	mov    QWORD PTR [rdi],rax
    1fb1:	48 8d 0d b2 bf 00 00 	lea    rcx,[rip+0xbfb2]        # df6a <cljn_rest>
    1fb8:	4c 89 ef             	mov    rdi,r13
    1fbb:	ff d1                	call   rcx
    1fbd:	48 8d 0d bc 20 01 02 	lea    rcx,[rip+0x20120bc]        # 2014080 <gc_sp>
    1fc4:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    1fc7:	4c 8d 05 b2 20 01 00 	lea    r8,[rip+0x120b2]        # 14080 <gc_stack>
    1fce:	4c 6b ca 08          	imul   r9,rdx,0x8
    1fd2:	4b 89 04 08          	mov    QWORD PTR [r8+r9*1],rax
    1fd6:	48 81 c2 01 00 00 00 	add    rdx,0x1
    1fdd:	48 89 11             	mov    QWORD PTR [rcx],rdx
    1fe0:	4c 89 e2             	mov    rdx,r12
    1fe3:	4c 8d 02             	lea    r8,[rdx]
    1fe6:	4c 8d 0d 93 20 01 00 	lea    r9,[rip+0x12093]        # 14080 <gc_stack>
    1fed:	4d 6b c0 08          	imul   r8,r8,0x8
    1ff1:	4f 89 3c 01          	mov    QWORD PTR [r9+r8*1],r15
    1ff5:	4c 8d 4a 01          	lea    r9,[rdx+0x1]
    1ff9:	4c 8d 15 80 20 01 00 	lea    r10,[rip+0x12080]        # 14080 <gc_stack>
    2000:	4d 6b c9 08          	imul   r9,r9,0x8
    2004:	48 89 d9             	mov    rcx,rbx
    2007:	4b 89 0c 0a          	mov    QWORD PTR [r10+r9*1],rcx
    200b:	4c 8d 52 02          	lea    r10,[rdx+0x2]
    200f:	4c 8d 1d 6a 20 01 00 	lea    r11,[rip+0x1206a]        # 14080 <gc_stack>
    2016:	4d 6b d2 08          	imul   r10,r10,0x8
    201a:	4b 89 04 13          	mov    QWORD PTR [r11+r10*1],rax
    201e:	4c 8d 1d 5b 20 01 02 	lea    r11,[rip+0x201205b]        # 2014080 <gc_sp>
    2025:	49 83 03 fe          	add    QWORD PTR [r11],0xfffffffffffffffe
    2029:	49 89 c5             	mov    r13,rax
    202c:	e9 75 fe ff ff       	jmp    1ea6 <reduce+0xca>
    2031:	48 8d 3d 48 20 01 02 	lea    rdi,[rip+0x2012048]        # 2014080 <gc_sp>
    2038:	48 8b 07             	mov    rax,QWORD PTR [rdi]
    203b:	48 8d 0d 3e 20 01 00 	lea    rcx,[rip+0x1203e]        # 14080 <gc_stack>
    2042:	48 6b d0 08          	imul   rdx,rax,0x8
    2046:	49 89 da             	mov    r10,rbx
    2049:	4c 89 14 11          	mov    QWORD PTR [rcx+rdx*1],r10
    204d:	48 81 c0 01 00 00 00 	add    rax,0x1
    2054:	48 89 07             	mov    QWORD PTR [rdi],rax
    2057:	48 8d 15 ec 4b 00 00 	lea    rdx,[rip+0x4bec]        # 6c4a <cljn_gc_leave>
    205e:	4c 89 e7             	mov    rdi,r12
    2061:	ff d2                	call   rdx
    2063:	48 89 d8             	mov    rax,rbx
    2066:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    206a:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    206f:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    2074:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    2079:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    207e:	48 83 c4 30          	add    rsp,0x30
    2082:	48 89 ec             	mov    rsp,rbp
    2085:	5d                   	pop    rbp
    2086:	c3                   	ret

0000000000002087 <map>:
    2087:	55                   	push   rbp
    2088:	48 89 e5             	mov    rbp,rsp
    208b:	48 83 ec 30          	sub    rsp,0x30
    208f:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    2093:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    2098:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    209d:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    20a2:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
    20a7:	49 89 d4             	mov    r12,rdx
    20aa:	49 89 f7             	mov    r15,rsi
    20ad:	bf 02 00 00 00       	mov    edi,0x2
    20b2:	48 8d 05 db 4a 00 00 	lea    rax,[rip+0x4adb]        # 6b94 <cljn_gc_enter>
    20b9:	ff d0                	call   rax
    20bb:	4d 89 fb             	mov    r11,r15
    20be:	49 83 fb 02          	cmp    r11,0x2
    20c2:	0f 84 39 00 00 00    	je     2101 <map+0x7a>
    20c8:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    20cf:	48 8d 0d a6 57 00 00 	lea    rcx,[rip+0x57a6]        # 787c <cljn_check_arity>
    20d6:	4c 89 df             	mov    rdi,r11
    20d9:	ff d1                	call   rcx
    20db:	b8 02 00 00 00       	mov    eax,0x2
    20e0:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    20e4:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    20e9:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    20ee:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    20f3:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    20f8:	48 83 c4 30          	add    rsp,0x30
    20fc:	48 89 ec             	mov    rsp,rbp
    20ff:	5d                   	pop    rbp
    2100:	c3                   	ret
    2101:	4c 89 e2             	mov    rdx,r12
    2104:	48 8b 1a             	mov    rbx,QWORD PTR [rdx]
    2107:	4c 8d 00             	lea    r8,[rax]
    210a:	4c 8d 0d 6f 1f 01 00 	lea    r9,[rip+0x11f6f]        # 14080 <gc_stack>
    2111:	4d 6b c0 08          	imul   r8,r8,0x8
    2115:	4b 89 1c 01          	mov    QWORD PTR [r9+r8*1],rbx
    2119:	4c 8b 62 08          	mov    r12,QWORD PTR [rdx+0x8]
    211d:	4c 8d 48 01          	lea    r9,[rax+0x1]
    2121:	49 89 c6             	mov    r14,rax
    2124:	4c 8d 15 55 1f 01 00 	lea    r10,[rip+0x11f55]        # 14080 <gc_stack>
    212b:	4d 6b c9 08          	imul   r9,r9,0x8
    212f:	4f 89 24 0a          	mov    QWORD PTR [r10+r9*1],r12
    2133:	4c 8d 1d be bb 00 00 	lea    r11,[rip+0xbbbe]        # dcf8 <cljn_emptyp>
    213a:	4c 89 e7             	mov    rdi,r12
    213d:	41 ff d3             	call   r11
    2140:	48 83 f8 06          	cmp    rax,0x6
    2144:	40 0f 95 c6          	setne  sil
    2148:	48 83 f8 02          	cmp    rax,0x2
    214c:	40 0f 95 c7          	setne  dil
    2150:	40 84 f7             	test   dil,sil
    2153:	0f 85 a7 01 00 00    	jne    2300 <map+0x279>
    2159:	48 8d 0d 20 1f 01 02 	lea    rcx,[rip+0x2011f20]        # 2014080 <gc_sp>
    2160:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    2163:	4c 8d 05 16 1f 01 00 	lea    r8,[rip+0x11f16]        # 14080 <gc_stack>
    216a:	4c 6b ca 08          	imul   r9,rdx,0x8
    216e:	4b 89 1c 08          	mov    QWORD PTR [r8+r9*1],rbx
    2172:	48 81 c2 01 00 00 00 	add    rdx,0x1
    2179:	48 89 11             	mov    QWORD PTR [rcx],rdx
    217c:	4c 8d 05 98 bc 00 00 	lea    r8,[rip+0xbc98]        # de1b <cljn_first>
    2183:	4c 89 e7             	mov    rdi,r12
    2186:	41 ff d0             	call   r8
    2189:	4c 8d 05 f0 1e 01 02 	lea    r8,[rip+0x2011ef0]        # 2014080 <gc_sp>
    2190:	4d 8b 08             	mov    r9,QWORD PTR [r8]
    2193:	4c 8d 15 e6 1e 01 00 	lea    r10,[rip+0x11ee6]        # 14080 <gc_stack>
    219a:	4d 6b d9 08          	imul   r11,r9,0x8
    219e:	4b 89 04 1a          	mov    QWORD PTR [r10+r11*1],rax
    21a2:	49 81 c1 01 00 00 00 	add    r9,0x1
    21a9:	4d 89 08             	mov    QWORD PTR [r8],r9
    21ac:	4c 8d 15 69 56 00 00 	lea    r10,[rip+0x5669]        # 781c <cljn_check_fn>
    21b3:	48 89 df             	mov    rdi,rbx
    21b6:	41 ff d2             	call   r10
    21b9:	41 bd 01 00 00 00    	mov    r13d,0x1
    21bf:	4c 8d 1d 89 56 00 00 	lea    r11,[rip+0x5689]        # 784f <cljn_argv>
    21c6:	4c 89 ef             	mov    rdi,r13
    21c9:	41 ff d3             	call   r11
    21cc:	49 89 c7             	mov    r15,rax
    21cf:	4c 8d 1d 30 56 00 00 	lea    r11,[rip+0x5630]        # 7806 <cljn_fn_code>
    21d6:	48 89 df             	mov    rdi,rbx
    21d9:	41 ff d3             	call   r11
    21dc:	4c 89 fa             	mov    rdx,r15
    21df:	4c 89 ee             	mov    rsi,r13
    21e2:	48 89 df             	mov    rdi,rbx
    21e5:	ff d0                	call   rax
    21e7:	4c 8d 1d 92 1e 01 02 	lea    r11,[rip+0x2011e92]        # 2014080 <gc_sp>
    21ee:	49 83 03 fe          	add    QWORD PTR [r11],0xfffffffffffffffe
    21f2:	48 8d 35 87 1e 01 02 	lea    rsi,[rip+0x2011e87]        # 2014080 <gc_sp>
    21f9:	48 8b 3e             	mov    rdi,QWORD PTR [rsi]
    21fc:	48 8d 0d 7d 1e 01 00 	lea    rcx,[rip+0x11e7d]        # 14080 <gc_stack>
    2203:	48 6b d7 08          	imul   rdx,rdi,0x8
    2207:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    220b:	49 89 c5             	mov    r13,rax
    220e:	48 81 c7 01 00 00 00 	add    rdi,0x1
    2215:	48 89 3e             	mov    QWORD PTR [rsi],rdi
    2218:	48 8d 05 61 1e 01 02 	lea    rax,[rip+0x2011e61]        # 2014080 <gc_sp>
    221f:	48 8b 08             	mov    rcx,QWORD PTR [rax]
    2222:	48 8d 15 57 1e 01 00 	lea    rdx,[rip+0x11e57]        # 14080 <gc_stack>
    2229:	4c 6b c1 08          	imul   r8,rcx,0x8
    222d:	4a 89 1c 02          	mov    QWORD PTR [rdx+r8*1],rbx
    2231:	48 81 c1 01 00 00 00 	add    rcx,0x1
    2238:	48 89 08             	mov    QWORD PTR [rax],rcx
    223b:	48 8d 15 28 bd 00 00 	lea    rdx,[rip+0xbd28]        # df6a <cljn_rest>
    2242:	4c 89 e7             	mov    rdi,r12
    2245:	ff d2                	call   rdx
    2247:	48 8d 15 32 1e 01 02 	lea    rdx,[rip+0x2011e32]        # 2014080 <gc_sp>
    224e:	4c 8b 02             	mov    r8,QWORD PTR [rdx]
    2251:	4c 8d 0d 28 1e 01 00 	lea    r9,[rip+0x11e28]        # 14080 <gc_stack>
    2258:	4d 6b d0 08          	imul   r10,r8,0x8
    225c:	4b 89 04 11          	mov    QWORD PTR [r9+r10*1],rax
    2260:	49 81 c0 01 00 00 00 	add    r8,0x1
    2267:	4c 89 02             	mov    QWORD PTR [rdx],r8
    226a:	41 bf 02 00 00 00    	mov    r15d,0x2
    2270:	4c 8d 0d d8 55 00 00 	lea    r9,[rip+0x55d8]        # 784f <cljn_argv>
    2277:	4c 89 ff             	mov    rdi,r15
    227a:	41 ff d1             	call   r9
    227d:	bf 02 00 00 00       	mov    edi,0x2
    2282:	48 89 c2             	mov    rdx,rax
    2285:	4c 89 fe             	mov    rsi,r15
    2288:	e8 fa fd ff ff       	call   2087 <map>
    228d:	4c 8d 0d ec 1d 01 02 	lea    r9,[rip+0x2011dec]        # 2014080 <gc_sp>
    2294:	49 83 01 fe          	add    QWORD PTR [r9],0xfffffffffffffffe
    2298:	4c 8d 15 e1 1d 01 02 	lea    r10,[rip+0x2011de1]        # 2014080 <gc_sp>
    229f:	4d 8b 1a             	mov    r11,QWORD PTR [r10]
    22a2:	48 8d 35 d7 1d 01 00 	lea    rsi,[rip+0x11dd7]        # 14080 <gc_stack>
    22a9:	49 6b fb 08          	imul   rdi,r11,0x8
    22ad:	48 89 04 3e          	mov    QWORD PTR [rsi+rdi*1],rax
    22b1:	49 81 c3 01 00 00 00 	add    r11,0x1
    22b8:	4d 89 1a             	mov    QWORD PTR [r10],r11
    22bb:	48 8d 0d 1f 54 00 00 	lea    rcx,[rip+0x541f]        # 76e1 <cljn_cons>
    22c2:	48 89 c6             	mov    rsi,rax
    22c5:	4c 89 ef             	mov    rdi,r13
    22c8:	ff d1                	call   rcx
    22ca:	48 8d 35 af 1d 01 02 	lea    rsi,[rip+0x2011daf]        # 2014080 <gc_sp>
    22d1:	48 83 06 fe          	add    QWORD PTR [rsi],0xfffffffffffffffe
    22d5:	48 8d 3d a4 1d 01 02 	lea    rdi,[rip+0x2011da4]        # 2014080 <gc_sp>
    22dc:	48 8b 0f             	mov    rcx,QWORD PTR [rdi]
    22df:	48 8d 15 9a 1d 01 00 	lea    rdx,[rip+0x11d9a]        # 14080 <gc_stack>
    22e6:	4c 6b c1 08          	imul   r8,rcx,0x8
    22ea:	4a 89 04 02          	mov    QWORD PTR [rdx+r8*1],rax
    22ee:	48 81 c1 01 00 00 00 	add    rcx,0x1
    22f5:	48 89 0f             	mov    QWORD PTR [rdi],rcx
    22f8:	49 89 c4             	mov    r12,rax
    22fb:	e9 5d 00 00 00       	jmp    235d <map+0x2d6>
    2300:	48 8d 15 cb 53 00 00 	lea    rdx,[rip+0x53cb]        # 76d2 <cljn_empty>
    2307:	ff d2                	call   rdx
    2309:	48 8d 15 70 1d 01 02 	lea    rdx,[rip+0x2011d70]        # 2014080 <gc_sp>
    2310:	4c 8b 02             	mov    r8,QWORD PTR [rdx]
    2313:	4c 8d 0d 66 1d 01 00 	lea    r9,[rip+0x11d66]        # 14080 <gc_stack>
    231a:	4d 6b d0 08          	imul   r10,r8,0x8
    231e:	4b 89 04 11          	mov    QWORD PTR [r9+r10*1],rax
    2322:	49 81 c0 01 00 00 00 	add    r8,0x1
    2329:	4c 89 02             	mov    QWORD PTR [rdx],r8
    232c:	4c 8d 0d 4d 1d 01 02 	lea    r9,[rip+0x2011d4d]        # 2014080 <gc_sp>
    2333:	49 83 01 ff          	add    QWORD PTR [r9],0xffffffffffffffff
    2337:	4c 8d 15 42 1d 01 02 	lea    r10,[rip+0x2011d42]        # 2014080 <gc_sp>
    233e:	4d 8b 1a             	mov    r11,QWORD PTR [r10]
    2341:	48 8d 35 38 1d 01 00 	lea    rsi,[rip+0x11d38]        # 14080 <gc_stack>
    2348:	49 6b fb 08          	imul   rdi,r11,0x8
    234c:	48 89 04 3e          	mov    QWORD PTR [rsi+rdi*1],rax
    2350:	49 89 c4             	mov    r12,rax
    2353:	49 81 c3 01 00 00 00 	add    r11,0x1
    235a:	4d 89 1a             	mov    QWORD PTR [r10],r11
    235d:	48 8d 05 e6 48 00 00 	lea    rax,[rip+0x48e6]        # 6c4a <cljn_gc_leave>
    2364:	4c 89 f7             	mov    rdi,r14
    2367:	ff d0                	call   rax
    2369:	4c 89 e0             	mov    rax,r12
    236c:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    2370:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    2375:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    237a:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    237f:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    2384:	48 83 c4 30          	add    rsp,0x30
    2388:	48 89 ec             	mov    rsp,rbp
    238b:	5d                   	pop    rbp
    238c:	c3                   	ret

000000000000238d <filter>:
    238d:	55                   	push   rbp
    238e:	48 89 e5             	mov    rbp,rsp
    2391:	48 83 ec 30          	sub    rsp,0x30
    2395:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    2399:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    239e:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    23a3:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    23a8:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
    23ad:	48 89 d3             	mov    rbx,rdx
    23b0:	49 89 f6             	mov    r14,rsi
    23b3:	bf 02 00 00 00       	mov    edi,0x2
    23b8:	4c 8d 1d d5 47 00 00 	lea    r11,[rip+0x47d5]        # 6b94 <cljn_gc_enter>
    23bf:	41 ff d3             	call   r11
    23c2:	4c 89 f1             	mov    rcx,r14
    23c5:	48 83 f9 02          	cmp    rcx,0x2
    23c9:	0f 84 39 00 00 00    	je     2408 <filter+0x7b>
    23cf:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    23d6:	48 8d 05 9f 54 00 00 	lea    rax,[rip+0x549f]        # 787c <cljn_check_arity>
    23dd:	48 89 cf             	mov    rdi,rcx
    23e0:	ff d0                	call   rax
    23e2:	b8 02 00 00 00       	mov    eax,0x2
    23e7:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    23eb:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    23f0:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    23f5:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    23fa:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    23ff:	48 83 c4 30          	add    rsp,0x30
    2403:	48 89 ec             	mov    rsp,rbp
    2406:	5d                   	pop    rbp
    2407:	c3                   	ret
    2408:	48 89 da             	mov    rdx,rbx
    240b:	4c 8b 3a             	mov    r15,QWORD PTR [rdx]
    240e:	48 8d 08             	lea    rcx,[rax]
    2411:	4c 8d 05 68 1c 01 00 	lea    r8,[rip+0x11c68]        # 14080 <gc_stack>
    2418:	48 6b c9 08          	imul   rcx,rcx,0x8
    241c:	4d 89 3c 08          	mov    QWORD PTR [r8+rcx*1],r15
    2420:	48 8b 7a 08          	mov    rdi,QWORD PTR [rdx+0x8]
    2424:	48 8d 50 01          	lea    rdx,[rax+0x1]
    2428:	49 89 c5             	mov    r13,rax
    242b:	4c 8d 05 4e 1c 01 00 	lea    r8,[rip+0x11c4e]        # 14080 <gc_stack>
    2432:	48 6b d2 08          	imul   rdx,rdx,0x8
    2436:	49 89 3c 10          	mov    QWORD PTR [r8+rdx*1],rdi
    243a:	49 89 fe             	mov    r14,rdi
    243d:	4c 8d 0d b4 b8 00 00 	lea    r9,[rip+0xb8b4]        # dcf8 <cljn_emptyp>
    2444:	4c 89 f7             	mov    rdi,r14
    2447:	41 ff d1             	call   r9
    244a:	48 83 f8 06          	cmp    rax,0x6
    244e:	41 0f 95 c2          	setne  r10b
    2452:	48 83 f8 02          	cmp    rax,0x2
    2456:	41 0f 95 c3          	setne  r11b
    245a:	45 84 d3             	test   r11b,r10b
    245d:	0f 85 9e 02 00 00    	jne    2701 <filter+0x374>
    2463:	48 8d 3d 16 1c 01 02 	lea    rdi,[rip+0x2011c16]        # 2014080 <gc_sp>
    246a:	48 8b 07             	mov    rax,QWORD PTR [rdi]
    246d:	48 8d 0d 0c 1c 01 00 	lea    rcx,[rip+0x11c0c]        # 14080 <gc_stack>
    2474:	48 6b d0 08          	imul   rdx,rax,0x8
    2478:	4c 89 3c 11          	mov    QWORD PTR [rcx+rdx*1],r15
    247c:	48 81 c0 01 00 00 00 	add    rax,0x1
    2483:	48 89 07             	mov    QWORD PTR [rdi],rax
    2486:	48 8d 0d 8e b9 00 00 	lea    rcx,[rip+0xb98e]        # de1b <cljn_first>
    248d:	4c 89 f7             	mov    rdi,r14
    2490:	ff d1                	call   rcx
    2492:	48 8d 0d e7 1b 01 02 	lea    rcx,[rip+0x2011be7]        # 2014080 <gc_sp>
    2499:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    249c:	4c 8d 05 dd 1b 01 00 	lea    r8,[rip+0x11bdd]        # 14080 <gc_stack>
    24a3:	4c 6b ca 08          	imul   r9,rdx,0x8
    24a7:	4b 89 04 08          	mov    QWORD PTR [r8+r9*1],rax
    24ab:	48 81 c2 01 00 00 00 	add    rdx,0x1
    24b2:	48 89 11             	mov    QWORD PTR [rcx],rdx
    24b5:	4c 8d 05 60 53 00 00 	lea    r8,[rip+0x5360]        # 781c <cljn_check_fn>
    24bc:	4c 89 ff             	mov    rdi,r15
    24bf:	41 ff d0             	call   r8
    24c2:	41 bc 01 00 00 00    	mov    r12d,0x1
    24c8:	4c 8d 0d 80 53 00 00 	lea    r9,[rip+0x5380]        # 784f <cljn_argv>
    24cf:	4c 89 e7             	mov    rdi,r12
    24d2:	41 ff d1             	call   r9
    24d5:	48 89 c3             	mov    rbx,rax
    24d8:	4c 8d 0d 27 53 00 00 	lea    r9,[rip+0x5327]        # 7806 <cljn_fn_code>
    24df:	4c 89 ff             	mov    rdi,r15
    24e2:	41 ff d1             	call   r9
    24e5:	48 89 da             	mov    rdx,rbx
    24e8:	4c 89 e6             	mov    rsi,r12
    24eb:	4c 89 ff             	mov    rdi,r15
    24ee:	ff d0                	call   rax
    24f0:	4c 8d 0d 89 1b 01 02 	lea    r9,[rip+0x2011b89]        # 2014080 <gc_sp>
    24f7:	49 83 01 fe          	add    QWORD PTR [r9],0xfffffffffffffffe
    24fb:	4c 8d 15 7e 1b 01 02 	lea    r10,[rip+0x2011b7e]        # 2014080 <gc_sp>
    2502:	4d 8b 1a             	mov    r11,QWORD PTR [r10]
    2505:	48 8d 35 74 1b 01 00 	lea    rsi,[rip+0x11b74]        # 14080 <gc_stack>
    250c:	49 6b fb 08          	imul   rdi,r11,0x8
    2510:	48 89 04 3e          	mov    QWORD PTR [rsi+rdi*1],rax
    2514:	49 81 c3 01 00 00 00 	add    r11,0x1
    251b:	4d 89 1a             	mov    QWORD PTR [r10],r11
    251e:	48 8d 35 5a b7 00 00 	lea    rsi,[rip+0xb75a]        # dc7f <cljn_truthy>
    2525:	48 89 c7             	mov    rdi,rax
    2528:	ff d6                	call   rsi
    252a:	48 8d 35 4f 1b 01 02 	lea    rsi,[rip+0x2011b4f]        # 2014080 <gc_sp>
    2531:	48 83 06 ff          	add    QWORD PTR [rsi],0xffffffffffffffff
    2535:	85 c0                	test   eax,eax
    2537:	0f 85 a9 00 00 00    	jne    25e6 <filter+0x259>
    253d:	48 8d 0d 3c 1b 01 02 	lea    rcx,[rip+0x2011b3c]        # 2014080 <gc_sp>
    2544:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    2547:	4c 8d 05 32 1b 01 00 	lea    r8,[rip+0x11b32]        # 14080 <gc_stack>
    254e:	4c 6b ca 08          	imul   r9,rdx,0x8
    2552:	4f 89 3c 08          	mov    QWORD PTR [r8+r9*1],r15
    2556:	48 81 c2 01 00 00 00 	add    rdx,0x1
    255d:	48 89 11             	mov    QWORD PTR [rcx],rdx
    2560:	4c 8d 05 03 ba 00 00 	lea    r8,[rip+0xba03]        # df6a <cljn_rest>
    2567:	4c 89 f7             	mov    rdi,r14
    256a:	41 ff d0             	call   r8
    256d:	4c 8d 05 0c 1b 01 02 	lea    r8,[rip+0x2011b0c]        # 2014080 <gc_sp>
    2574:	4d 8b 08             	mov    r9,QWORD PTR [r8]
    2577:	4c 8d 15 02 1b 01 00 	lea    r10,[rip+0x11b02]        # 14080 <gc_stack>
    257e:	4d 6b d9 08          	imul   r11,r9,0x8
    2582:	4b 89 04 1a          	mov    QWORD PTR [r10+r11*1],rax
    2586:	49 81 c1 01 00 00 00 	add    r9,0x1
    258d:	4d 89 08             	mov    QWORD PTR [r8],r9
    2590:	41 bc 02 00 00 00    	mov    r12d,0x2
    2596:	4c 8d 15 b2 52 00 00 	lea    r10,[rip+0x52b2]        # 784f <cljn_argv>
    259d:	4c 89 e7             	mov    rdi,r12
    25a0:	41 ff d2             	call   r10
    25a3:	bf 02 00 00 00       	mov    edi,0x2
    25a8:	48 89 c2             	mov    rdx,rax
    25ab:	4c 89 e6             	mov    rsi,r12
    25ae:	e8 da fd ff ff       	call   238d <filter>
    25b3:	4c 8d 15 c6 1a 01 02 	lea    r10,[rip+0x2011ac6]        # 2014080 <gc_sp>
    25ba:	49 83 02 fe          	add    QWORD PTR [r10],0xfffffffffffffffe
    25be:	4c 8d 1d bb 1a 01 02 	lea    r11,[rip+0x2011abb]        # 2014080 <gc_sp>
    25c5:	49 8b 33             	mov    rsi,QWORD PTR [r11]
    25c8:	48 8d 3d b1 1a 01 00 	lea    rdi,[rip+0x11ab1]        # 14080 <gc_stack>
    25cf:	48 6b ce 08          	imul   rcx,rsi,0x8
    25d3:	48 89 04 0f          	mov    QWORD PTR [rdi+rcx*1],rax
    25d7:	48 81 c6 01 00 00 00 	add    rsi,0x1
    25de:	49 89 33             	mov    QWORD PTR [r11],rsi
    25e1:	e9 13 01 00 00       	jmp    26f9 <filter+0x36c>
    25e6:	48 8d 05 2e b8 00 00 	lea    rax,[rip+0xb82e]        # de1b <cljn_first>
    25ed:	4c 89 f7             	mov    rdi,r14
    25f0:	ff d0                	call   rax
    25f2:	48 8d 0d 87 1a 01 02 	lea    rcx,[rip+0x2011a87]        # 2014080 <gc_sp>
    25f9:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    25fc:	4c 8d 05 7d 1a 01 00 	lea    r8,[rip+0x11a7d]        # 14080 <gc_stack>
    2603:	4c 6b ca 08          	imul   r9,rdx,0x8
    2607:	4b 89 04 08          	mov    QWORD PTR [r8+r9*1],rax
    260b:	48 89 c3             	mov    rbx,rax
    260e:	48 81 c2 01 00 00 00 	add    rdx,0x1
    2615:	48 89 11             	mov    QWORD PTR [rcx],rdx
    2618:	48 8d 15 61 1a 01 02 	lea    rdx,[rip+0x2011a61]        # 2014080 <gc_sp>
    261f:	4c 8b 02             	mov    r8,QWORD PTR [rdx]
    2622:	4c 8d 0d 57 1a 01 00 	lea    r9,[rip+0x11a57]        # 14080 <gc_stack>
    2629:	4d 6b d0 08          	imul   r10,r8,0x8
    262d:	4f 89 3c 11          	mov    QWORD PTR [r9+r10*1],r15
    2631:	49 81 c0 01 00 00 00 	add    r8,0x1
    2638:	4c 89 02             	mov    QWORD PTR [rdx],r8
    263b:	4c 8d 0d 28 b9 00 00 	lea    r9,[rip+0xb928]        # df6a <cljn_rest>
    2642:	4c 89 f7             	mov    rdi,r14
    2645:	41 ff d1             	call   r9
    2648:	4c 8d 0d 31 1a 01 02 	lea    r9,[rip+0x2011a31]        # 2014080 <gc_sp>
    264f:	4d 8b 11             	mov    r10,QWORD PTR [r9]
    2652:	4c 8d 1d 27 1a 01 00 	lea    r11,[rip+0x11a27]        # 14080 <gc_stack>
    2659:	49 6b f2 08          	imul   rsi,r10,0x8
    265d:	49 89 04 33          	mov    QWORD PTR [r11+rsi*1],rax
    2661:	49 81 c2 01 00 00 00 	add    r10,0x1
    2668:	4d 89 11             	mov    QWORD PTR [r9],r10
    266b:	41 bc 02 00 00 00    	mov    r12d,0x2
    2671:	4c 8d 1d d7 51 00 00 	lea    r11,[rip+0x51d7]        # 784f <cljn_argv>
    2678:	4c 89 e7             	mov    rdi,r12
    267b:	41 ff d3             	call   r11
    267e:	bf 02 00 00 00       	mov    edi,0x2
    2683:	48 89 c2             	mov    rdx,rax
    2686:	4c 89 e6             	mov    rsi,r12
    2689:	e8 ff fc ff ff       	call   238d <filter>
    268e:	4c 8d 1d eb 19 01 02 	lea    r11,[rip+0x20119eb]        # 2014080 <gc_sp>
    2695:	49 83 03 fe          	add    QWORD PTR [r11],0xfffffffffffffffe
    2699:	48 8d 35 e0 19 01 02 	lea    rsi,[rip+0x20119e0]        # 2014080 <gc_sp>
    26a0:	48 8b 3e             	mov    rdi,QWORD PTR [rsi]
    26a3:	48 8d 0d d6 19 01 00 	lea    rcx,[rip+0x119d6]        # 14080 <gc_stack>
    26aa:	48 6b d7 08          	imul   rdx,rdi,0x8
    26ae:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    26b2:	48 81 c7 01 00 00 00 	add    rdi,0x1
    26b9:	48 89 3e             	mov    QWORD PTR [rsi],rdi
    26bc:	48 8d 0d 1e 50 00 00 	lea    rcx,[rip+0x501e]        # 76e1 <cljn_cons>
    26c3:	48 89 c6             	mov    rsi,rax
    26c6:	48 89 df             	mov    rdi,rbx
    26c9:	ff d1                	call   rcx
    26cb:	48 8d 0d ae 19 01 02 	lea    rcx,[rip+0x20119ae]        # 2014080 <gc_sp>
    26d2:	48 83 01 fe          	add    QWORD PTR [rcx],0xfffffffffffffffe
    26d6:	48 8d 0d a3 19 01 02 	lea    rcx,[rip+0x20119a3]        # 2014080 <gc_sp>
    26dd:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    26e0:	4c 8d 05 99 19 01 00 	lea    r8,[rip+0x11999]        # 14080 <gc_stack>
    26e7:	4c 6b ca 08          	imul   r9,rdx,0x8
    26eb:	4b 89 04 08          	mov    QWORD PTR [r8+r9*1],rax
    26ef:	48 81 c2 01 00 00 00 	add    rdx,0x1
    26f6:	48 89 11             	mov    QWORD PTR [rcx],rdx
    26f9:	49 89 c6             	mov    r14,rax
    26fc:	e9 5b 00 00 00       	jmp    275c <filter+0x3cf>
    2701:	4c 8d 15 ca 4f 00 00 	lea    r10,[rip+0x4fca]        # 76d2 <cljn_empty>
    2708:	41 ff d2             	call   r10
    270b:	4c 8d 15 6e 19 01 02 	lea    r10,[rip+0x201196e]        # 2014080 <gc_sp>
    2712:	4d 8b 1a             	mov    r11,QWORD PTR [r10]
    2715:	48 8d 35 64 19 01 00 	lea    rsi,[rip+0x11964]        # 14080 <gc_stack>
    271c:	49 6b fb 08          	imul   rdi,r11,0x8
    2720:	48 89 04 3e          	mov    QWORD PTR [rsi+rdi*1],rax
    2724:	49 81 c3 01 00 00 00 	add    r11,0x1
    272b:	4d 89 1a             	mov    QWORD PTR [r10],r11
    272e:	48 8d 35 4b 19 01 02 	lea    rsi,[rip+0x201194b]        # 2014080 <gc_sp>
    2735:	48 83 06 ff          	add    QWORD PTR [rsi],0xffffffffffffffff
    2739:	48 8d 3d 40 19 01 02 	lea    rdi,[rip+0x2011940]        # 2014080 <gc_sp>
    2740:	48 8b 0f             	mov    rcx,QWORD PTR [rdi]
    2743:	48 8d 15 36 19 01 00 	lea    rdx,[rip+0x11936]        # 14080 <gc_stack>
    274a:	4c 6b c1 08          	imul   r8,rcx,0x8
    274e:	4a 89 04 02          	mov    QWORD PTR [rdx+r8*1],rax
    2752:	49 89 c6             	mov    r14,rax
    2755:	48 8d 41 01          	lea    rax,[rcx+0x1]
    2759:	48 89 07             	mov    QWORD PTR [rdi],rax
    275c:	48 8d 15 e7 44 00 00 	lea    rdx,[rip+0x44e7]        # 6c4a <cljn_gc_leave>
    2763:	4c 89 ef             	mov    rdi,r13
    2766:	ff d2                	call   rdx
    2768:	4c 89 f0             	mov    rax,r14
    276b:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    276f:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    2774:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    2779:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    277e:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    2783:	48 83 c4 30          	add    rsp,0x30
    2787:	48 89 ec             	mov    rsp,rbp
    278a:	5d                   	pop    rbp
    278b:	c3                   	ret

000000000000278c <__lambda_2>:
    278c:	55                   	push   rbp
    278d:	48 89 e5             	mov    rbp,rsp
    2790:	48 83 ec 30          	sub    rsp,0x30
    2794:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    2798:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    279d:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    27a2:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    27a7:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
    27ac:	49 89 f4             	mov    r12,rsi
    27af:	49 89 fd             	mov    r13,rdi
    27b2:	49 89 d6             	mov    r14,rdx
    27b5:	bf 01 00 00 00       	mov    edi,0x1
    27ba:	4c 8d 1d d3 43 00 00 	lea    r11,[rip+0x43d3]        # 6b94 <cljn_gc_enter>
    27c1:	41 ff d3             	call   r11
    27c4:	4d 89 e0             	mov    r8,r12
    27c7:	49 83 f8 01          	cmp    r8,0x1
    27cb:	0f 84 39 00 00 00    	je     280a <__lambda_2+0x7e>
    27d1:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    27d8:	48 8d 05 9d 50 00 00 	lea    rax,[rip+0x509d]        # 787c <cljn_check_arity>
    27df:	4c 89 c7             	mov    rdi,r8
    27e2:	ff d0                	call   rax
    27e4:	b8 02 00 00 00       	mov    eax,0x2
    27e9:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    27ed:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    27f2:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    27f7:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    27fc:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    2801:	48 83 c4 30          	add    rsp,0x30
    2805:	48 89 ec             	mov    rsp,rbp
    2808:	5d                   	pop    rbp
    2809:	c3                   	ret
    280a:	4c 89 f2             	mov    rdx,r14
    280d:	4c 8b 32             	mov    r14,QWORD PTR [rdx]
    2810:	48 8d 08             	lea    rcx,[rax]
    2813:	49 89 c4             	mov    r12,rax
    2816:	48 8d 15 63 18 01 00 	lea    rdx,[rip+0x11863]        # 14080 <gc_stack>
    281d:	48 6b c9 08          	imul   rcx,rcx,0x8
    2821:	4c 89 34 0a          	mov    QWORD PTR [rdx+rcx*1],r14
    2825:	48 33 f6             	xor    rsi,rsi
    2828:	4c 8d 05 b4 4f 00 00 	lea    r8,[rip+0x4fb4]        # 77e3 <cljn_fn_free>
    282f:	4c 89 ef             	mov    rdi,r13
    2832:	41 ff d0             	call   r8
    2835:	49 89 c7             	mov    r15,rax
    2838:	4c 8d 05 41 18 01 02 	lea    r8,[rip+0x2011841]        # 2014080 <gc_sp>
    283f:	4d 8b 08             	mov    r9,QWORD PTR [r8]
    2842:	4c 8d 15 37 18 01 00 	lea    r10,[rip+0x11837]        # 14080 <gc_stack>
    2849:	4d 6b d9 08          	imul   r11,r9,0x8
    284d:	4b 89 04 1a          	mov    QWORD PTR [r10+r11*1],rax
    2851:	49 81 c1 01 00 00 00 	add    r9,0x1
    2858:	4d 89 08             	mov    QWORD PTR [r8],r9
    285b:	4c 8d 15 1e 18 01 02 	lea    r10,[rip+0x201181e]        # 2014080 <gc_sp>
    2862:	4d 8b 1a             	mov    r11,QWORD PTR [r10]
    2865:	48 8d 35 14 18 01 00 	lea    rsi,[rip+0x11814]        # 14080 <gc_stack>
    286c:	49 6b fb 08          	imul   rdi,r11,0x8
    2870:	4c 89 34 3e          	mov    QWORD PTR [rsi+rdi*1],r14
    2874:	49 81 c3 01 00 00 00 	add    r11,0x1
    287b:	4d 89 1a             	mov    QWORD PTR [r10],r11
    287e:	48 8d 35 97 4f 00 00 	lea    rsi,[rip+0x4f97]        # 781c <cljn_check_fn>
    2885:	4c 89 ff             	mov    rdi,r15
    2888:	ff d6                	call   rsi
    288a:	bb 01 00 00 00       	mov    ebx,0x1
    288f:	48 8d 05 b9 4f 00 00 	lea    rax,[rip+0x4fb9]        # 784f <cljn_argv>
    2896:	48 89 df             	mov    rdi,rbx
    2899:	ff d0                	call   rax
    289b:	49 89 c5             	mov    r13,rax
    289e:	48 8d 0d 61 4f 00 00 	lea    rcx,[rip+0x4f61]        # 7806 <cljn_fn_code>
    28a5:	4c 89 ff             	mov    rdi,r15
    28a8:	ff d1                	call   rcx
    28aa:	4c 89 ea             	mov    rdx,r13
    28ad:	48 89 de             	mov    rsi,rbx
    28b0:	4c 89 ff             	mov    rdi,r15
    28b3:	ff d0                	call   rax
    28b5:	48 8d 3d c4 17 01 02 	lea    rdi,[rip+0x20117c4]        # 2014080 <gc_sp>
    28bc:	48 83 07 fe          	add    QWORD PTR [rdi],0xfffffffffffffffe
    28c0:	48 8d 0d b9 17 01 02 	lea    rcx,[rip+0x20117b9]        # 2014080 <gc_sp>
    28c7:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    28ca:	4c 8d 05 af 17 01 00 	lea    r8,[rip+0x117af]        # 14080 <gc_stack>
    28d1:	4c 6b ca 08          	imul   r9,rdx,0x8
    28d5:	4b 89 04 08          	mov    QWORD PTR [r8+r9*1],rax
    28d9:	48 81 c2 01 00 00 00 	add    rdx,0x1
    28e0:	48 89 11             	mov    QWORD PTR [rcx],rdx
    28e3:	48 8d 15 bd b3 00 00 	lea    rdx,[rip+0xb3bd]        # dca7 <cljn_not>
    28ea:	48 89 c7             	mov    rdi,rax
    28ed:	ff d2                	call   rdx
    28ef:	49 89 c5             	mov    r13,rax
    28f2:	48 8d 15 87 17 01 02 	lea    rdx,[rip+0x2011787]        # 2014080 <gc_sp>
    28f9:	48 83 02 ff          	add    QWORD PTR [rdx],0xffffffffffffffff
    28fd:	4c 8d 05 46 43 00 00 	lea    r8,[rip+0x4346]        # 6c4a <cljn_gc_leave>
    2904:	4c 89 e7             	mov    rdi,r12
    2907:	41 ff d0             	call   r8
    290a:	4c 89 e8             	mov    rax,r13
    290d:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    2911:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    2916:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    291b:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    2920:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    2925:	48 83 c4 30          	add    rsp,0x30
    2929:	48 89 ec             	mov    rsp,rbp
    292c:	5d                   	pop    rbp
    292d:	c3                   	ret

000000000000292e <remove>:
    292e:	55                   	push   rbp
    292f:	48 89 e5             	mov    rbp,rsp
    2932:	48 83 ec 30          	sub    rsp,0x30
    2936:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    293a:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    293f:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    2944:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    2949:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
    294e:	48 89 d3             	mov    rbx,rdx
    2951:	49 89 f6             	mov    r14,rsi
    2954:	bf 02 00 00 00       	mov    edi,0x2
    2959:	4c 8d 15 34 42 00 00 	lea    r10,[rip+0x4234]        # 6b94 <cljn_gc_enter>
    2960:	41 ff d2             	call   r10
    2963:	4c 89 f1             	mov    rcx,r14
    2966:	48 83 f9 02          	cmp    rcx,0x2
    296a:	0f 84 39 00 00 00    	je     29a9 <remove+0x7b>
    2970:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    2977:	48 8d 05 fe 4e 00 00 	lea    rax,[rip+0x4efe]        # 787c <cljn_check_arity>
    297e:	48 89 cf             	mov    rdi,rcx
    2981:	ff d0                	call   rax
    2983:	b8 02 00 00 00       	mov    eax,0x2
    2988:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    298c:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    2991:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    2996:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    299b:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    29a0:	48 83 c4 30          	add    rsp,0x30
    29a4:	48 89 ec             	mov    rsp,rbp
    29a7:	5d                   	pop    rbp
    29a8:	c3                   	ret
    29a9:	48 89 da             	mov    rdx,rbx
    29ac:	4c 8b 22             	mov    r12,QWORD PTR [rdx]
    29af:	48 8d 08             	lea    rcx,[rax]
    29b2:	4c 8d 05 c7 16 01 00 	lea    r8,[rip+0x116c7]        # 14080 <gc_stack>
    29b9:	48 6b c9 08          	imul   rcx,rcx,0x8
    29bd:	4d 89 24 08          	mov    QWORD PTR [r8+rcx*1],r12
    29c1:	4c 8b 6a 08          	mov    r13,QWORD PTR [rdx+0x8]
    29c5:	48 8d 48 01          	lea    rcx,[rax+0x1]
    29c9:	49 89 c6             	mov    r14,rax
    29cc:	48 8d 15 ad 16 01 00 	lea    rdx,[rip+0x116ad]        # 14080 <gc_stack>
    29d3:	48 6b c9 08          	imul   rcx,rcx,0x8
    29d7:	4c 89 2c 0a          	mov    QWORD PTR [rdx+rcx*1],r13
    29db:	4c 8d 05 9e 16 01 02 	lea    r8,[rip+0x201169e]        # 2014080 <gc_sp>
    29e2:	4d 8b 08             	mov    r9,QWORD PTR [r8]
    29e5:	4c 8d 15 94 16 01 00 	lea    r10,[rip+0x11694]        # 14080 <gc_stack>
    29ec:	4d 6b d9 08          	imul   r11,r9,0x8
    29f0:	4f 89 24 1a          	mov    QWORD PTR [r10+r11*1],r12
    29f4:	49 81 c1 01 00 00 00 	add    r9,0x1
    29fb:	4d 89 08             	mov    QWORD PTR [r8],r9
    29fe:	48 8d 3d 87 fd ff ff 	lea    rdi,[rip+0xfffffffffffffd87]        # 278c <__lambda_2>
    2a05:	be 01 00 00 00       	mov    esi,0x1
    2a0a:	ba 01 00 00 00       	mov    edx,0x1
    2a0f:	4c 8d 15 10 4d 00 00 	lea    r10,[rip+0x4d10]        # 7726 <cljn_make_fn>
    2a16:	41 ff d2             	call   r10
    2a19:	4c 8d 15 60 16 01 02 	lea    r10,[rip+0x2011660]        # 2014080 <gc_sp>
    2a20:	49 83 02 ff          	add    QWORD PTR [r10],0xffffffffffffffff
    2a24:	4c 8d 1d 55 16 01 02 	lea    r11,[rip+0x2011655]        # 2014080 <gc_sp>
    2a2b:	49 8b 33             	mov    rsi,QWORD PTR [r11]
    2a2e:	48 8d 3d 4b 16 01 00 	lea    rdi,[rip+0x1164b]        # 14080 <gc_stack>
    2a35:	48 6b ce 08          	imul   rcx,rsi,0x8
    2a39:	48 89 04 0f          	mov    QWORD PTR [rdi+rcx*1],rax
    2a3d:	48 81 c6 01 00 00 00 	add    rsi,0x1
    2a44:	49 89 33             	mov    QWORD PTR [r11],rsi
    2a47:	48 33 f6             	xor    rsi,rsi
    2a4a:	48 8d 0d 66 4d 00 00 	lea    rcx,[rip+0x4d66]        # 77b7 <cljn_fn_set_free>
    2a51:	4c 89 e2             	mov    rdx,r12
    2a54:	48 89 c7             	mov    rdi,rax
    2a57:	ff d1                	call   rcx
    2a59:	48 8d 05 20 16 01 02 	lea    rax,[rip+0x2011620]        # 2014080 <gc_sp>
    2a60:	48 8b 08             	mov    rcx,QWORD PTR [rax]
    2a63:	48 8d 15 16 16 01 00 	lea    rdx,[rip+0x11616]        # 14080 <gc_stack>
    2a6a:	4c 6b c1 08          	imul   r8,rcx,0x8
    2a6e:	4e 89 2c 02          	mov    QWORD PTR [rdx+r8*1],r13
    2a72:	48 81 c1 01 00 00 00 	add    rcx,0x1
    2a79:	48 89 08             	mov    QWORD PTR [rax],rcx
    2a7c:	41 bd 02 00 00 00    	mov    r13d,0x2
    2a82:	48 8d 15 c6 4d 00 00 	lea    rdx,[rip+0x4dc6]        # 784f <cljn_argv>
    2a89:	4c 89 ef             	mov    rdi,r13
    2a8c:	ff d2                	call   rdx
    2a8e:	bf 02 00 00 00       	mov    edi,0x2
    2a93:	48 89 c2             	mov    rdx,rax
    2a96:	4c 89 ee             	mov    rsi,r13
    2a99:	e8 ef f8 ff ff       	call   238d <filter>
    2a9e:	48 8d 15 db 15 01 02 	lea    rdx,[rip+0x20115db]        # 2014080 <gc_sp>
    2aa5:	48 83 02 fe          	add    QWORD PTR [rdx],0xfffffffffffffffe
    2aa9:	4c 8d 05 d0 15 01 02 	lea    r8,[rip+0x20115d0]        # 2014080 <gc_sp>
    2ab0:	4d 8b 08             	mov    r9,QWORD PTR [r8]
    2ab3:	4c 8d 15 c6 15 01 00 	lea    r10,[rip+0x115c6]        # 14080 <gc_stack>
    2aba:	4d 6b d9 08          	imul   r11,r9,0x8
    2abe:	4b 89 04 1a          	mov    QWORD PTR [r10+r11*1],rax
    2ac2:	49 89 c7             	mov    r15,rax
    2ac5:	49 81 c1 01 00 00 00 	add    r9,0x1
    2acc:	4d 89 08             	mov    QWORD PTR [r8],r9
    2acf:	4c 8d 15 74 41 00 00 	lea    r10,[rip+0x4174]        # 6c4a <cljn_gc_leave>
    2ad6:	4c 89 f7             	mov    rdi,r14
    2ad9:	41 ff d2             	call   r10
    2adc:	4c 89 f8             	mov    rax,r15
    2adf:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    2ae3:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    2ae8:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    2aed:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    2af2:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    2af7:	48 83 c4 30          	add    rsp,0x30
    2afb:	48 89 ec             	mov    rsp,rbp
    2afe:	5d                   	pop    rbp
    2aff:	c3                   	ret

0000000000002b00 <__lambda_3>:
    2b00:	55                   	push   rbp
    2b01:	48 89 e5             	mov    rbp,rsp
    2b04:	48 83 ec 20          	sub    rsp,0x20
    2b08:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    2b0c:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    2b11:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    2b16:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    2b1b:	49 89 d5             	mov    r13,rdx
    2b1e:	49 89 f6             	mov    r14,rsi
    2b21:	bf 02 00 00 00       	mov    edi,0x2
    2b26:	4c 8d 0d 67 40 00 00 	lea    r9,[rip+0x4067]        # 6b94 <cljn_gc_enter>
    2b2d:	41 ff d1             	call   r9
    2b30:	4c 89 f7             	mov    rdi,r14
    2b33:	48 83 ff 02          	cmp    rdi,0x2
    2b37:	0f 84 32 00 00 00    	je     2b6f <__lambda_3+0x6f>
    2b3d:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    2b44:	4c 8d 1d 31 4d 00 00 	lea    r11,[rip+0x4d31]        # 787c <cljn_check_arity>
    2b4b:	41 ff d3             	call   r11
    2b4e:	b8 02 00 00 00       	mov    eax,0x2
    2b53:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    2b57:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    2b5c:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    2b61:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    2b66:	48 83 c4 20          	add    rsp,0x20
    2b6a:	48 89 ec             	mov    rsp,rbp
    2b6d:	5d                   	pop    rbp
    2b6e:	c3                   	ret
    2b6f:	4c 89 ea             	mov    rdx,r13
    2b72:	48 8b 32             	mov    rsi,QWORD PTR [rdx]
    2b75:	48 8d 38             	lea    rdi,[rax]
    2b78:	48 8d 0d 01 15 01 00 	lea    rcx,[rip+0x11501]        # 14080 <gc_stack>
    2b7f:	48 6b ff 08          	imul   rdi,rdi,0x8
    2b83:	48 89 34 39          	mov    QWORD PTR [rcx+rdi*1],rsi
    2b87:	48 8b 7a 08          	mov    rdi,QWORD PTR [rdx+0x8]
    2b8b:	48 8d 48 01          	lea    rcx,[rax+0x1]
    2b8f:	48 89 c3             	mov    rbx,rax
    2b92:	48 8d 05 e7 14 01 00 	lea    rax,[rip+0x114e7]        # 14080 <gc_stack>
    2b99:	48 6b c9 08          	imul   rcx,rcx,0x8
    2b9d:	48 89 3c 08          	mov    QWORD PTR [rax+rcx*1],rdi
    2ba1:	48 8d 15 39 4b 00 00 	lea    rdx,[rip+0x4b39]        # 76e1 <cljn_cons>
    2ba8:	ff d2                	call   rdx
    2baa:	48 8d 15 cf 14 01 02 	lea    rdx,[rip+0x20114cf]        # 2014080 <gc_sp>
    2bb1:	4c 8b 02             	mov    r8,QWORD PTR [rdx]
    2bb4:	4c 8d 0d c5 14 01 00 	lea    r9,[rip+0x114c5]        # 14080 <gc_stack>
    2bbb:	4d 6b d0 08          	imul   r10,r8,0x8
    2bbf:	4b 89 04 11          	mov    QWORD PTR [r9+r10*1],rax
    2bc3:	49 89 c4             	mov    r12,rax
    2bc6:	49 81 c0 01 00 00 00 	add    r8,0x1
    2bcd:	4c 89 02             	mov    QWORD PTR [rdx],r8
    2bd0:	4c 8d 0d 73 40 00 00 	lea    r9,[rip+0x4073]        # 6c4a <cljn_gc_leave>
    2bd7:	48 89 df             	mov    rdi,rbx
    2bda:	41 ff d1             	call   r9
    2bdd:	4c 89 e0             	mov    rax,r12
    2be0:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    2be4:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    2be9:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    2bee:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    2bf3:	48 83 c4 20          	add    rsp,0x20
    2bf7:	48 89 ec             	mov    rsp,rbp
    2bfa:	5d                   	pop    rbp
    2bfb:	c3                   	ret

0000000000002bfc <reverse>:
    2bfc:	55                   	push   rbp
    2bfd:	48 89 e5             	mov    rbp,rsp
    2c00:	48 83 ec 20          	sub    rsp,0x20
    2c04:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    2c08:	4c 89 6c 24 08       	mov    QWORD PTR [rsp+0x8],r13
    2c0d:	4c 89 74 24 10       	mov    QWORD PTR [rsp+0x10],r14
    2c12:	4c 89 7c 24 18       	mov    QWORD PTR [rsp+0x18],r15
    2c17:	48 89 f3             	mov    rbx,rsi
    2c1a:	49 89 d5             	mov    r13,rdx
    2c1d:	bf 01 00 00 00       	mov    edi,0x1
    2c22:	4c 8d 1d 6b 3f 00 00 	lea    r11,[rip+0x3f6b]        # 6b94 <cljn_gc_enter>
    2c29:	41 ff d3             	call   r11
    2c2c:	48 89 df             	mov    rdi,rbx
    2c2f:	48 83 ff 01          	cmp    rdi,0x1
    2c33:	0f 84 31 00 00 00    	je     2c6a <reverse+0x6e>
    2c39:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    2c40:	48 8d 05 35 4c 00 00 	lea    rax,[rip+0x4c35]        # 787c <cljn_check_arity>
    2c47:	ff d0                	call   rax
    2c49:	b8 02 00 00 00       	mov    eax,0x2
    2c4e:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    2c52:	4c 8b 6c 24 08       	mov    r13,QWORD PTR [rsp+0x8]
    2c57:	4c 8b 74 24 10       	mov    r14,QWORD PTR [rsp+0x10]
    2c5c:	4c 8b 7c 24 18       	mov    r15,QWORD PTR [rsp+0x18]
    2c61:	48 83 c4 20          	add    rsp,0x20
    2c65:	48 89 ec             	mov    rsp,rbp
    2c68:	5d                   	pop    rbp
    2c69:	c3                   	ret
    2c6a:	4c 89 ea             	mov    rdx,r13
    2c6d:	4c 8b 2a             	mov    r13,QWORD PTR [rdx]
    2c70:	48 8d 08             	lea    rcx,[rax]
    2c73:	49 89 c7             	mov    r15,rax
    2c76:	48 8d 15 03 14 01 00 	lea    rdx,[rip+0x11403]        # 14080 <gc_stack>
    2c7d:	48 6b c9 08          	imul   rcx,rcx,0x8
    2c81:	4c 89 2c 0a          	mov    QWORD PTR [rdx+rcx*1],r13
    2c85:	48 8d 3d 74 fe ff ff 	lea    rdi,[rip+0xfffffffffffffe74]        # 2b00 <__lambda_3>
    2c8c:	be 02 00 00 00       	mov    esi,0x2
    2c91:	48 33 d2             	xor    rdx,rdx
    2c94:	4c 8d 05 8b 4a 00 00 	lea    r8,[rip+0x4a8b]        # 7726 <cljn_make_fn>
    2c9b:	41 ff d0             	call   r8
    2c9e:	4c 8d 05 db 13 01 02 	lea    r8,[rip+0x20113db]        # 2014080 <gc_sp>
    2ca5:	4d 8b 08             	mov    r9,QWORD PTR [r8]
    2ca8:	4c 8d 15 d1 13 01 00 	lea    r10,[rip+0x113d1]        # 14080 <gc_stack>
    2caf:	4d 6b d9 08          	imul   r11,r9,0x8
    2cb3:	4b 89 04 1a          	mov    QWORD PTR [r10+r11*1],rax
    2cb7:	49 81 c1 01 00 00 00 	add    r9,0x1
    2cbe:	4d 89 08             	mov    QWORD PTR [r8],r9
    2cc1:	4c 8d 15 0a 4a 00 00 	lea    r10,[rip+0x4a0a]        # 76d2 <cljn_empty>
    2cc8:	41 ff d2             	call   r10
    2ccb:	4c 8d 15 ae 13 01 02 	lea    r10,[rip+0x20113ae]        # 2014080 <gc_sp>
    2cd2:	4d 8b 1a             	mov    r11,QWORD PTR [r10]
    2cd5:	48 8d 35 a4 13 01 00 	lea    rsi,[rip+0x113a4]        # 14080 <gc_stack>
    2cdc:	49 6b fb 08          	imul   rdi,r11,0x8
    2ce0:	48 89 04 3e          	mov    QWORD PTR [rsi+rdi*1],rax
    2ce4:	49 81 c3 01 00 00 00 	add    r11,0x1
    2ceb:	4d 89 1a             	mov    QWORD PTR [r10],r11
    2cee:	48 8d 35 8b 13 01 02 	lea    rsi,[rip+0x201138b]        # 2014080 <gc_sp>
    2cf5:	48 83 06 ff          	add    QWORD PTR [rsi],0xffffffffffffffff
    2cf9:	48 8d 3d 80 13 01 02 	lea    rdi,[rip+0x2011380]        # 2014080 <gc_sp>
    2d00:	48 8b 0f             	mov    rcx,QWORD PTR [rdi]
    2d03:	48 8d 15 76 13 01 00 	lea    rdx,[rip+0x11376]        # 14080 <gc_stack>
    2d0a:	4c 6b c1 08          	imul   r8,rcx,0x8
    2d0e:	4a 89 04 02          	mov    QWORD PTR [rdx+r8*1],rax
    2d12:	48 8d 41 01          	lea    rax,[rcx+0x1]
    2d16:	48 89 07             	mov    QWORD PTR [rdi],rax
    2d19:	48 8d 0d 60 13 01 02 	lea    rcx,[rip+0x2011360]        # 2014080 <gc_sp>
    2d20:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    2d23:	4c 8d 05 56 13 01 00 	lea    r8,[rip+0x11356]        # 14080 <gc_stack>
    2d2a:	4c 6b ca 08          	imul   r9,rdx,0x8
    2d2e:	4f 89 2c 08          	mov    QWORD PTR [r8+r9*1],r13
    2d32:	48 81 c2 01 00 00 00 	add    rdx,0x1
    2d39:	48 89 11             	mov    QWORD PTR [rcx],rdx
    2d3c:	41 be 03 00 00 00    	mov    r14d,0x3
    2d42:	4c 8d 05 06 4b 00 00 	lea    r8,[rip+0x4b06]        # 784f <cljn_argv>
    2d49:	4c 89 f7             	mov    rdi,r14
    2d4c:	41 ff d0             	call   r8
    2d4f:	bf 02 00 00 00       	mov    edi,0x2
    2d54:	48 89 c2             	mov    rdx,rax
    2d57:	4c 89 f6             	mov    rsi,r14
    2d5a:	e8 7d f0 ff ff       	call   1ddc <reduce>
    2d5f:	4c 8d 05 1a 13 01 02 	lea    r8,[rip+0x201131a]        # 2014080 <gc_sp>
    2d66:	49 83 00 fd          	add    QWORD PTR [r8],0xfffffffffffffffd
    2d6a:	4c 8d 0d 0f 13 01 02 	lea    r9,[rip+0x201130f]        # 2014080 <gc_sp>
    2d71:	4d 8b 11             	mov    r10,QWORD PTR [r9]
    2d74:	4c 8d 1d 05 13 01 00 	lea    r11,[rip+0x11305]        # 14080 <gc_stack>
    2d7b:	49 6b f2 08          	imul   rsi,r10,0x8
    2d7f:	49 89 04 33          	mov    QWORD PTR [r11+rsi*1],rax
    2d83:	49 89 c5             	mov    r13,rax
    2d86:	49 81 c2 01 00 00 00 	add    r10,0x1
    2d8d:	4d 89 11             	mov    QWORD PTR [r9],r10
    2d90:	4c 8d 1d b3 3e 00 00 	lea    r11,[rip+0x3eb3]        # 6c4a <cljn_gc_leave>
    2d97:	4c 89 ff             	mov    rdi,r15
    2d9a:	41 ff d3             	call   r11
    2d9d:	4c 89 e8             	mov    rax,r13
    2da0:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    2da4:	4c 8b 6c 24 08       	mov    r13,QWORD PTR [rsp+0x8]
    2da9:	4c 8b 74 24 10       	mov    r14,QWORD PTR [rsp+0x10]
    2dae:	4c 8b 7c 24 18       	mov    r15,QWORD PTR [rsp+0x18]
    2db3:	48 83 c4 20          	add    rsp,0x20
    2db7:	48 89 ec             	mov    rsp,rbp
    2dba:	5d                   	pop    rbp
    2dbb:	c3                   	ret
    2dbc:	00 00                	add    BYTE PTR [rax],al
	...

0000000000002dc0 <take>:
    2dc0:	55                   	push   rbp
    2dc1:	48 89 e5             	mov    rbp,rsp
    2dc4:	48 83 ec 20          	sub    rsp,0x20
    2dc8:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    2dcc:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    2dd1:	4c 89 74 24 10       	mov    QWORD PTR [rsp+0x10],r14
    2dd6:	4c 89 7c 24 18       	mov    QWORD PTR [rsp+0x18],r15
    2ddb:	49 89 d4             	mov    r12,rdx
    2dde:	49 89 f7             	mov    r15,rsi
    2de1:	bf 02 00 00 00       	mov    edi,0x2
    2de6:	4c 8d 05 a7 3d 00 00 	lea    r8,[rip+0x3da7]        # 6b94 <cljn_gc_enter>
    2ded:	41 ff d0             	call   r8
    2df0:	4c 89 fa             	mov    rdx,r15
    2df3:	48 83 fa 02          	cmp    rdx,0x2
    2df7:	0f 84 35 00 00 00    	je     2e32 <take+0x72>
    2dfd:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    2e04:	4c 8d 15 71 4a 00 00 	lea    r10,[rip+0x4a71]        # 787c <cljn_check_arity>
    2e0b:	48 89 d7             	mov    rdi,rdx
    2e0e:	41 ff d2             	call   r10
    2e11:	b8 02 00 00 00       	mov    eax,0x2
    2e16:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    2e1a:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    2e1f:	4c 8b 74 24 10       	mov    r14,QWORD PTR [rsp+0x10]
    2e24:	4c 8b 7c 24 18       	mov    r15,QWORD PTR [rsp+0x18]
    2e29:	48 83 c4 20          	add    rsp,0x20
    2e2d:	48 89 ec             	mov    rsp,rbp
    2e30:	5d                   	pop    rbp
    2e31:	c3                   	ret
    2e32:	4c 89 e2             	mov    rdx,r12
    2e35:	4c 8b 3a             	mov    r15,QWORD PTR [rdx]
    2e38:	48 8d 30             	lea    rsi,[rax]
    2e3b:	48 8d 3d 3e 12 01 00 	lea    rdi,[rip+0x1123e]        # 14080 <gc_stack>
    2e42:	48 6b f6 08          	imul   rsi,rsi,0x8
    2e46:	4c 89 3c 37          	mov    QWORD PTR [rdi+rsi*1],r15
    2e4a:	48 8b 5a 08          	mov    rbx,QWORD PTR [rdx+0x8]
    2e4e:	48 8d 78 01          	lea    rdi,[rax+0x1]
    2e52:	49 89 c4             	mov    r12,rax
    2e55:	48 8d 05 24 12 01 00 	lea    rax,[rip+0x11224]        # 14080 <gc_stack>
    2e5c:	48 6b ff 08          	imul   rdi,rdi,0x8
    2e60:	48 89 1c 38          	mov    QWORD PTR [rax+rdi*1],rbx
    2e64:	48 8d 0d 8d ae 00 00 	lea    rcx,[rip+0xae8d]        # dcf8 <cljn_emptyp>
    2e6b:	48 89 df             	mov    rdi,rbx
    2e6e:	ff d1                	call   rcx
    2e70:	48 83 f8 06          	cmp    rax,0x6
    2e74:	0f 95 c2             	setne  dl
    2e77:	48 83 f8 02          	cmp    rax,0x2
    2e7b:	41 0f 95 c0          	setne  r8b
    2e7f:	41 84 d0             	test   r8b,dl
    2e82:	0f 85 48 00 00 00    	jne    2ed0 <take+0x110>
    2e88:	be 01 00 00 00       	mov    esi,0x1
    2e8d:	4d 89 fa             	mov    r10,r15
    2e90:	49 83 e2 01          	and    r10,0x1
    2e94:	49 f7 c2 01 00 00 00 	test   r10,0x1
    2e9b:	0f 85 11 00 00 00    	jne    2eb2 <take+0xf2>
    2ea1:	48 8d 05 69 a5 00 00 	lea    rax,[rip+0xa569]        # d411 <cljn_le>
    2ea8:	4c 89 ff             	mov    rdi,r15
    2eab:	ff d0                	call   rax
    2ead:	e9 23 00 00 00       	jmp    2ed5 <take+0x115>
    2eb2:	4c 89 ff             	mov    rdi,r15
    2eb5:	48 d1 ff             	sar    rdi,1
    2eb8:	48 d1 fe             	sar    rsi,1
    2ebb:	b8 06 00 00 00       	mov    eax,0x6
    2ec0:	48 3b fe             	cmp    rdi,rsi
    2ec3:	48 0f 4e 05 1d 02 00 	cmovle rax,QWORD PTR [rip+0x21d]        # 30e8 <take+0x328>
    2eca:	00 
    2ecb:	e9 05 00 00 00       	jmp    2ed5 <take+0x115>
    2ed0:	b8 0a 00 00 00       	mov    eax,0xa
    2ed5:	48 83 f8 06          	cmp    rax,0x6
    2ed9:	41 0f 95 c1          	setne  r9b
    2edd:	48 83 f8 02          	cmp    rax,0x2
    2ee1:	41 0f 95 c2          	setne  r10b
    2ee5:	45 84 ca             	test   r10b,r9b
    2ee8:	0f 85 6c 01 00 00    	jne    305a <take+0x29a>
    2eee:	48 8d 35 26 af 00 00 	lea    rsi,[rip+0xaf26]        # de1b <cljn_first>
    2ef5:	48 89 df             	mov    rdi,rbx
    2ef8:	ff d6                	call   rsi
    2efa:	48 89 c2             	mov    rdx,rax
    2efd:	48 8d 35 7c 11 01 02 	lea    rsi,[rip+0x201117c]        # 2014080 <gc_sp>
    2f04:	48 8b 3e             	mov    rdi,QWORD PTR [rsi]
    2f07:	48 8d 05 72 11 01 00 	lea    rax,[rip+0x11172]        # 14080 <gc_stack>
    2f0e:	48 6b cf 08          	imul   rcx,rdi,0x8
    2f12:	49 89 d6             	mov    r14,rdx
    2f15:	4c 89 34 08          	mov    QWORD PTR [rax+rcx*1],r14
    2f19:	48 81 c7 01 00 00 00 	add    rdi,0x1
    2f20:	48 89 3e             	mov    QWORD PTR [rsi],rdi
    2f23:	49 f7 c7 01 00 00 00 	test   r15,0x1
    2f2a:	0f 84 29 00 00 00    	je     2f59 <take+0x199>
    2f30:	4d 89 f8             	mov    r8,r15
    2f33:	49 d1 f8             	sar    r8,1
    2f36:	49 8d 40 ff          	lea    rax,[r8-0x1]
    2f3a:	48 3b 05 af 01 00 00 	cmp    rax,QWORD PTR [rip+0x1af]        # 30f0 <take+0x330>
    2f41:	41 0f 9d c1          	setge  r9b
    2f45:	48 3b 05 ac 01 00 00 	cmp    rax,QWORD PTR [rip+0x1ac]        # 30f8 <take+0x338>
    2f4c:	41 0f 9e c2          	setle  r10b
    2f50:	45 84 ca             	test   r10b,r9b
    2f53:	0f 85 11 00 00 00    	jne    2f6a <take+0x1aa>
    2f59:	48 8d 05 aa a3 00 00 	lea    rax,[rip+0xa3aa]        # d30a <cljn_dec>
    2f60:	4c 89 ff             	mov    rdi,r15
    2f63:	ff d0                	call   rax
    2f65:	e9 07 00 00 00       	jmp    2f71 <take+0x1b1>
    2f6a:	48 d1 e0             	shl    rax,1
    2f6d:	48 83 c8 01          	or     rax,0x1
    2f71:	48 8d 0d 08 11 01 02 	lea    rcx,[rip+0x2011108]        # 2014080 <gc_sp>
    2f78:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    2f7b:	4c 8d 05 fe 10 01 00 	lea    r8,[rip+0x110fe]        # 14080 <gc_stack>
    2f82:	4c 6b ca 08          	imul   r9,rdx,0x8
    2f86:	4b 89 04 08          	mov    QWORD PTR [r8+r9*1],rax
    2f8a:	48 81 c2 01 00 00 00 	add    rdx,0x1
    2f91:	48 89 11             	mov    QWORD PTR [rcx],rdx
    2f94:	4c 8d 05 cf af 00 00 	lea    r8,[rip+0xafcf]        # df6a <cljn_rest>
    2f9b:	48 89 df             	mov    rdi,rbx
    2f9e:	41 ff d0             	call   r8
    2fa1:	4c 8d 05 d8 10 01 02 	lea    r8,[rip+0x20110d8]        # 2014080 <gc_sp>
    2fa8:	4d 8b 08             	mov    r9,QWORD PTR [r8]
    2fab:	4c 8d 15 ce 10 01 00 	lea    r10,[rip+0x110ce]        # 14080 <gc_stack>
    2fb2:	4d 6b d9 08          	imul   r11,r9,0x8
    2fb6:	4b 89 04 1a          	mov    QWORD PTR [r10+r11*1],rax
    2fba:	49 81 c1 01 00 00 00 	add    r9,0x1
    2fc1:	4d 89 08             	mov    QWORD PTR [r8],r9
    2fc4:	41 bf 02 00 00 00    	mov    r15d,0x2
    2fca:	4c 8d 15 7e 48 00 00 	lea    r10,[rip+0x487e]        # 784f <cljn_argv>
    2fd1:	4c 89 ff             	mov    rdi,r15
    2fd4:	41 ff d2             	call   r10
    2fd7:	bf 02 00 00 00       	mov    edi,0x2
    2fdc:	48 89 c2             	mov    rdx,rax
    2fdf:	4c 89 fe             	mov    rsi,r15
    2fe2:	e8 d9 fd ff ff       	call   2dc0 <take>
    2fe7:	4c 8d 15 92 10 01 02 	lea    r10,[rip+0x2011092]        # 2014080 <gc_sp>
    2fee:	49 83 02 fe          	add    QWORD PTR [r10],0xfffffffffffffffe
    2ff2:	4c 8d 1d 87 10 01 02 	lea    r11,[rip+0x2011087]        # 2014080 <gc_sp>
    2ff9:	49 8b 33             	mov    rsi,QWORD PTR [r11]
    2ffc:	48 8d 3d 7d 10 01 00 	lea    rdi,[rip+0x1107d]        # 14080 <gc_stack>
    3003:	48 6b ce 08          	imul   rcx,rsi,0x8
    3007:	48 89 04 0f          	mov    QWORD PTR [rdi+rcx*1],rax
    300b:	48 81 c6 01 00 00 00 	add    rsi,0x1
    3012:	49 89 33             	mov    QWORD PTR [r11],rsi
    3015:	48 8d 0d c5 46 00 00 	lea    rcx,[rip+0x46c5]        # 76e1 <cljn_cons>
    301c:	48 89 c6             	mov    rsi,rax
    301f:	4c 89 f7             	mov    rdi,r14
    3022:	ff d1                	call   rcx
    3024:	48 8d 3d 55 10 01 02 	lea    rdi,[rip+0x2011055]        # 2014080 <gc_sp>
    302b:	48 83 07 fe          	add    QWORD PTR [rdi],0xfffffffffffffffe
    302f:	48 8d 0d 4a 10 01 02 	lea    rcx,[rip+0x201104a]        # 2014080 <gc_sp>
    3036:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    3039:	4c 8d 05 40 10 01 00 	lea    r8,[rip+0x11040]        # 14080 <gc_stack>
    3040:	4c 6b ca 08          	imul   r9,rdx,0x8
    3044:	4b 89 04 08          	mov    QWORD PTR [r8+r9*1],rax
    3048:	48 81 c2 01 00 00 00 	add    rdx,0x1
    304f:	48 89 11             	mov    QWORD PTR [rcx],rdx
    3052:	49 89 c6             	mov    r14,rax
    3055:	e9 5e 00 00 00       	jmp    30b8 <take+0x2f8>
    305a:	4c 8d 05 71 46 00 00 	lea    r8,[rip+0x4671]        # 76d2 <cljn_empty>
    3061:	41 ff d0             	call   r8
    3064:	4c 8d 05 15 10 01 02 	lea    r8,[rip+0x2011015]        # 2014080 <gc_sp>
    306b:	4d 8b 08             	mov    r9,QWORD PTR [r8]
    306e:	4c 8d 15 0b 10 01 00 	lea    r10,[rip+0x1100b]        # 14080 <gc_stack>
    3075:	4d 6b d9 08          	imul   r11,r9,0x8
    3079:	4b 89 04 1a          	mov    QWORD PTR [r10+r11*1],rax
    307d:	49 81 c1 01 00 00 00 	add    r9,0x1
    3084:	4d 89 08             	mov    QWORD PTR [r8],r9
    3087:	4c 8d 15 f2 0f 01 02 	lea    r10,[rip+0x2010ff2]        # 2014080 <gc_sp>
    308e:	49 83 02 ff          	add    QWORD PTR [r10],0xffffffffffffffff
    3092:	4c 8d 1d e7 0f 01 02 	lea    r11,[rip+0x2010fe7]        # 2014080 <gc_sp>
    3099:	49 8b 33             	mov    rsi,QWORD PTR [r11]
    309c:	48 8d 3d dd 0f 01 00 	lea    rdi,[rip+0x10fdd]        # 14080 <gc_stack>
    30a3:	48 6b ce 08          	imul   rcx,rsi,0x8
    30a7:	48 89 04 0f          	mov    QWORD PTR [rdi+rcx*1],rax
    30ab:	49 89 c6             	mov    r14,rax
    30ae:	48 81 c6 01 00 00 00 	add    rsi,0x1
    30b5:	49 89 33             	mov    QWORD PTR [r11],rsi
    30b8:	48 8d 05 8b 3b 00 00 	lea    rax,[rip+0x3b8b]        # 6c4a <cljn_gc_leave>
    30bf:	4c 89 e7             	mov    rdi,r12
    30c2:	ff d0                	call   rax
    30c4:	4c 89 f0             	mov    rax,r14
    30c7:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    30cb:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    30d0:	4c 8b 74 24 10       	mov    r14,QWORD PTR [rsp+0x10]
    30d5:	4c 8b 7c 24 18       	mov    r15,QWORD PTR [rsp+0x18]
    30da:	48 83 c4 20          	add    rsp,0x20
    30de:	48 89 ec             	mov    rsp,rbp
    30e1:	5d                   	pop    rbp
    30e2:	c3                   	ret
    30e3:	00 00                	add    BYTE PTR [rax],al
    30e5:	00 00                	add    BYTE PTR [rax],al
    30e7:	00 0a                	add    BYTE PTR [rdx],cl
	...
    30f5:	00 00                	add    BYTE PTR [rax],al
    30f7:	c0 ff ff             	sar    bh,0xff
    30fa:	ff                   	(bad)
    30fb:	ff                   	(bad)
    30fc:	ff                   	(bad)
    30fd:	ff                   	(bad)
    30fe:	ff                   	(bad)
    30ff:	3f                   	(bad)

0000000000003100 <drop>:
    3100:	55                   	push   rbp
    3101:	48 89 e5             	mov    rbp,rsp
    3104:	48 83 ec 20          	sub    rsp,0x20
    3108:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    310c:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    3111:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    3116:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    311b:	48 89 d3             	mov    rbx,rdx
    311e:	49 89 f6             	mov    r14,rsi
    3121:	bf 02 00 00 00       	mov    edi,0x2
    3126:	48 8d 05 67 3a 00 00 	lea    rax,[rip+0x3a67]        # 6b94 <cljn_gc_enter>
    312d:	ff d0                	call   rax
    312f:	4d 89 f0             	mov    r8,r14
    3132:	49 83 f8 02          	cmp    r8,0x2
    3136:	0f 84 34 00 00 00    	je     3170 <drop+0x70>
    313c:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    3143:	48 8d 15 32 47 00 00 	lea    rdx,[rip+0x4732]        # 787c <cljn_check_arity>
    314a:	4c 89 c7             	mov    rdi,r8
    314d:	ff d2                	call   rdx
    314f:	b8 02 00 00 00       	mov    eax,0x2
    3154:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    3158:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    315d:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    3162:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    3167:	48 83 c4 20          	add    rsp,0x20
    316b:	48 89 ec             	mov    rsp,rbp
    316e:	5d                   	pop    rbp
    316f:	c3                   	ret
    3170:	48 89 da             	mov    rdx,rbx
    3173:	48 8b 3a             	mov    rdi,QWORD PTR [rdx]
    3176:	4c 8d 08             	lea    r9,[rax]
    3179:	4c 8d 15 00 0f 01 00 	lea    r10,[rip+0x10f00]        # 14080 <gc_stack>
    3180:	4d 6b c9 08          	imul   r9,r9,0x8
    3184:	4b 89 3c 0a          	mov    QWORD PTR [r10+r9*1],rdi
    3188:	49 89 fe             	mov    r14,rdi
    318b:	48 8b 7a 08          	mov    rdi,QWORD PTR [rdx+0x8]
    318f:	4c 8d 50 01          	lea    r10,[rax+0x1]
    3193:	49 89 c4             	mov    r12,rax
    3196:	4c 8d 1d e3 0e 01 00 	lea    r11,[rip+0x10ee3]        # 14080 <gc_stack>
    319d:	4d 6b d2 08          	imul   r10,r10,0x8
    31a1:	4b 89 3c 13          	mov    QWORD PTR [r11+r10*1],rdi
    31a5:	48 89 fb             	mov    rbx,rdi
    31a8:	48 8d 35 49 ab 00 00 	lea    rsi,[rip+0xab49]        # dcf8 <cljn_emptyp>
    31af:	48 89 df             	mov    rdi,rbx
    31b2:	ff d6                	call   rsi
    31b4:	48 83 f8 06          	cmp    rax,0x6
    31b8:	40 0f 95 c7          	setne  dil
    31bc:	48 83 f8 02          	cmp    rax,0x2
    31c0:	0f 95 c0             	setne  al
    31c3:	40 84 f8             	test   al,dil
    31c6:	0f 85 4f 00 00 00    	jne    321b <drop+0x11b>
    31cc:	be 01 00 00 00       	mov    esi,0x1
    31d1:	4c 89 f7             	mov    rdi,r14
    31d4:	48 89 fa             	mov    rdx,rdi
    31d7:	48 83 e2 01          	and    rdx,0x1
    31db:	48 f7 c2 01 00 00 00 	test   rdx,0x1
    31e2:	0f 85 12 00 00 00    	jne    31fa <drop+0xfa>
    31e8:	4c 8d 0d 22 a2 00 00 	lea    r9,[rip+0xa222]        # d411 <cljn_le>
    31ef:	4c 89 f7             	mov    rdi,r14
    31f2:	41 ff d1             	call   r9
    31f5:	e9 26 00 00 00       	jmp    3220 <drop+0x120>
    31fa:	4c 89 f7             	mov    rdi,r14
    31fd:	49 89 fa             	mov    r10,rdi
    3200:	49 d1 fa             	sar    r10,1
    3203:	48 d1 fe             	sar    rsi,1
    3206:	b8 06 00 00 00       	mov    eax,0x6
    320b:	4c 3b d6             	cmp    r10,rsi
    320e:	48 0f 4e 05 42 01 00 	cmovle rax,QWORD PTR [rip+0x142]        # 3358 <drop+0x258>
    3215:	00 
    3216:	e9 05 00 00 00       	jmp    3220 <drop+0x120>
    321b:	b8 0a 00 00 00       	mov    eax,0xa
    3220:	48 83 f8 06          	cmp    rax,0x6
    3224:	0f 95 c1             	setne  cl
    3227:	48 83 f8 02          	cmp    rax,0x2
    322b:	0f 95 c2             	setne  dl
    322e:	84 ca                	test   dl,cl
    3230:	0f 85 c8 00 00 00    	jne    32fe <drop+0x1fe>
    3236:	4c 89 f7             	mov    rdi,r14
    3239:	48 f7 c7 01 00 00 00 	test   rdi,0x1
    3240:	0f 84 2b 00 00 00    	je     3271 <drop+0x171>
    3246:	4c 89 f7             	mov    rdi,r14
    3249:	48 89 fe             	mov    rsi,rdi
    324c:	48 d1 fe             	sar    rsi,1
    324f:	48 8d 4e ff          	lea    rcx,[rsi-0x1]
    3253:	48 3b 0d 06 01 00 00 	cmp    rcx,QWORD PTR [rip+0x106]        # 3360 <drop+0x260>
    325a:	40 0f 9d c7          	setge  dil
    325e:	48 3b 0d 03 01 00 00 	cmp    rcx,QWORD PTR [rip+0x103]        # 3368 <drop+0x268>
    3265:	0f 9e c0             	setle  al
    3268:	40 84 f8             	test   al,dil
    326b:	0f 85 15 00 00 00    	jne    3286 <drop+0x186>
    3271:	4c 8d 05 92 a0 00 00 	lea    r8,[rip+0xa092]        # d30a <cljn_dec>
    3278:	4c 89 f7             	mov    rdi,r14
    327b:	41 ff d0             	call   r8
    327e:	49 89 c6             	mov    r14,rax
    3281:	e9 0a 00 00 00       	jmp    3290 <drop+0x190>
    3286:	48 d1 e1             	shl    rcx,1
    3289:	48 83 c9 01          	or     rcx,0x1
    328d:	49 89 ce             	mov    r14,rcx
    3290:	4c 8d 15 d3 ac 00 00 	lea    r10,[rip+0xacd3]        # df6a <cljn_rest>
    3297:	48 89 df             	mov    rdi,rbx
    329a:	41 ff d2             	call   r10
    329d:	4c 8d 15 dc 0d 01 02 	lea    r10,[rip+0x2010ddc]        # 2014080 <gc_sp>
    32a4:	4d 8b 1a             	mov    r11,QWORD PTR [r10]
    32a7:	48 8d 35 d2 0d 01 00 	lea    rsi,[rip+0x10dd2]        # 14080 <gc_stack>
    32ae:	49 6b fb 08          	imul   rdi,r11,0x8
    32b2:	48 89 04 3e          	mov    QWORD PTR [rsi+rdi*1],rax
    32b6:	49 81 c3 01 00 00 00 	add    r11,0x1
    32bd:	4d 89 1a             	mov    QWORD PTR [r10],r11
    32c0:	4c 89 e2             	mov    rdx,r12
    32c3:	48 8d 32             	lea    rsi,[rdx]
    32c6:	48 8d 3d b3 0d 01 00 	lea    rdi,[rip+0x10db3]        # 14080 <gc_stack>
    32cd:	48 6b f6 08          	imul   rsi,rsi,0x8
    32d1:	4d 89 f1             	mov    r9,r14
    32d4:	4c 89 0c 37          	mov    QWORD PTR [rdi+rsi*1],r9
    32d8:	48 8d 7a 01          	lea    rdi,[rdx+0x1]
    32dc:	48 8d 0d 9d 0d 01 00 	lea    rcx,[rip+0x10d9d]        # 14080 <gc_stack>
    32e3:	48 6b ff 08          	imul   rdi,rdi,0x8
    32e7:	48 89 04 39          	mov    QWORD PTR [rcx+rdi*1],rax
    32eb:	48 8d 0d 8e 0d 01 02 	lea    rcx,[rip+0x2010d8e]        # 2014080 <gc_sp>
    32f2:	48 83 01 ff          	add    QWORD PTR [rcx],0xffffffffffffffff
    32f6:	48 89 c3             	mov    rbx,rax
    32f9:	e9 aa fe ff ff       	jmp    31a8 <drop+0xa8>
    32fe:	48 89 df             	mov    rdi,rbx
    3301:	48 8d 15 78 0d 01 02 	lea    rdx,[rip+0x2010d78]        # 2014080 <gc_sp>
    3308:	4c 8b 02             	mov    r8,QWORD PTR [rdx]
    330b:	4c 8d 0d 6e 0d 01 00 	lea    r9,[rip+0x10d6e]        # 14080 <gc_stack>
    3312:	4d 6b d0 08          	imul   r10,r8,0x8
    3316:	4b 89 3c 11          	mov    QWORD PTR [r9+r10*1],rdi
    331a:	49 81 c0 01 00 00 00 	add    r8,0x1
    3321:	4c 89 02             	mov    QWORD PTR [rdx],r8
    3324:	49 89 fd             	mov    r13,rdi
    3327:	4c 8d 15 1c 39 00 00 	lea    r10,[rip+0x391c]        # 6c4a <cljn_gc_leave>
    332e:	4c 89 e7             	mov    rdi,r12
    3331:	41 ff d2             	call   r10
    3334:	4c 89 e8             	mov    rax,r13
    3337:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    333b:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    3340:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    3345:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    334a:	48 83 c4 20          	add    rsp,0x20
    334e:	48 89 ec             	mov    rsp,rbp
    3351:	5d                   	pop    rbp
    3352:	c3                   	ret
    3353:	00 00                	add    BYTE PTR [rax],al
    3355:	00 00                	add    BYTE PTR [rax],al
    3357:	00 0a                	add    BYTE PTR [rdx],cl
	...
    3365:	00 00                	add    BYTE PTR [rax],al
    3367:	c0 ff ff             	sar    bh,0xff
    336a:	ff                   	(bad)
    336b:	ff                   	(bad)
    336c:	ff                   	(bad)
    336d:	ff                   	(bad)
    336e:	ff                   	(bad)
    336f:	3f                   	(bad)

0000000000003370 <range>:
    3370:	55                   	push   rbp
    3371:	48 89 e5             	mov    rbp,rsp
    3374:	48 83 ec 30          	sub    rsp,0x30
    3378:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    337c:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    3381:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    3386:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    338b:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
    3390:	49 89 f5             	mov    r13,rsi
    3393:	49 89 d7             	mov    r15,rdx
    3396:	bf 03 00 00 00       	mov    edi,0x3
    339b:	4c 8d 15 f2 37 00 00 	lea    r10,[rip+0x37f2]        # 6b94 <cljn_gc_enter>
    33a2:	41 ff d2             	call   r10
    33a5:	48 89 c3             	mov    rbx,rax
    33a8:	4d 89 e9             	mov    r9,r13
    33ab:	49 83 f9 01          	cmp    r9,0x1
    33af:	0f 84 39 00 00 00    	je     33ee <range+0x7e>
    33b5:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    33bc:	48 8d 05 b9 44 00 00 	lea    rax,[rip+0x44b9]        # 787c <cljn_check_arity>
    33c3:	4c 89 cf             	mov    rdi,r9
    33c6:	ff d0                	call   rax
    33c8:	b8 02 00 00 00       	mov    eax,0x2
    33cd:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    33d1:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    33d6:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    33db:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    33e0:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    33e5:	48 83 c4 30          	add    rsp,0x30
    33e9:	48 89 ec             	mov    rsp,rbp
    33ec:	5d                   	pop    rbp
    33ed:	c3                   	ret
    33ee:	4c 89 fa             	mov    rdx,r15
    33f1:	4c 8b 32             	mov    r14,QWORD PTR [rdx]
    33f4:	48 89 d8             	mov    rax,rbx
    33f7:	48 81 c0 00 00 00 00 	add    rax,0x0
    33fe:	48 8d 0d 7b 0c 01 00 	lea    rcx,[rip+0x10c7b]        # 14080 <gc_stack>
    3405:	48 6b c0 08          	imul   rax,rax,0x8
    3409:	4c 89 34 01          	mov    QWORD PTR [rcx+rax*1],r14
    340d:	bf 01 00 00 00       	mov    edi,0x1
    3412:	49 89 fd             	mov    r13,rdi
    3415:	48 8d 15 b6 42 00 00 	lea    rdx,[rip+0x42b6]        # 76d2 <cljn_empty>
    341c:	ff d2                	call   rdx
    341e:	48 8d 15 5b 0c 01 02 	lea    rdx,[rip+0x2010c5b]        # 2014080 <gc_sp>
    3425:	4c 8b 02             	mov    r8,QWORD PTR [rdx]
    3428:	4c 8d 0d 51 0c 01 00 	lea    r9,[rip+0x10c51]        # 14080 <gc_stack>
    342f:	4d 6b d0 08          	imul   r10,r8,0x8
    3433:	4b 89 04 11          	mov    QWORD PTR [r9+r10*1],rax
    3437:	49 81 c0 01 00 00 00 	add    r8,0x1
    343e:	4c 89 02             	mov    QWORD PTR [rdx],r8
    3441:	4c 8d 0d 38 0c 01 02 	lea    r9,[rip+0x2010c38]        # 2014080 <gc_sp>
    3448:	49 83 01 ff          	add    QWORD PTR [r9],0xffffffffffffffff
    344c:	4c 8d 15 2d 0c 01 02 	lea    r10,[rip+0x2010c2d]        # 2014080 <gc_sp>
    3453:	4d 8b 1a             	mov    r11,QWORD PTR [r10]
    3456:	48 8d 35 23 0c 01 00 	lea    rsi,[rip+0x10c23]        # 14080 <gc_stack>
    345d:	49 6b fb 08          	imul   rdi,r11,0x8
    3461:	48 89 04 3e          	mov    QWORD PTR [rsi+rdi*1],rax
    3465:	49 81 c3 01 00 00 00 	add    r11,0x1
    346c:	4d 89 1a             	mov    QWORD PTR [r10],r11
    346f:	48 89 d9             	mov    rcx,rbx
    3472:	48 8d 71 02          	lea    rsi,[rcx+0x2]
    3476:	48 8d 3d 03 0c 01 00 	lea    rdi,[rip+0x10c03]        # 14080 <gc_stack>
    347d:	48 6b f6 08          	imul   rsi,rsi,0x8
    3481:	48 89 04 37          	mov    QWORD PTR [rdi+rsi*1],rax
    3485:	48 8d 3d f4 0b 01 02 	lea    rdi,[rip+0x2010bf4]        # 2014080 <gc_sp>
    348c:	48 83 07 ff          	add    QWORD PTR [rdi],0xffffffffffffffff
    3490:	4c 89 ef             	mov    rdi,r13
    3493:	49 89 c7             	mov    r15,rax
    3496:	48 89 f9             	mov    rcx,rdi
    3499:	49 23 ce             	and    rcx,r14
    349c:	49 89 fd             	mov    r13,rdi
    349f:	48 f7 c1 01 00 00 00 	test   rcx,0x1
    34a6:	0f 85 15 00 00 00    	jne    34c1 <range+0x151>
    34ac:	4c 8d 05 04 9f 00 00 	lea    r8,[rip+0x9f04]        # d3b7 <cljn_lt>
    34b3:	4c 89 f6             	mov    rsi,r14
    34b6:	4c 89 ef             	mov    rdi,r13
    34b9:	41 ff d0             	call   r8
    34bc:	e9 1f 00 00 00       	jmp    34e0 <range+0x170>
    34c1:	4c 89 ef             	mov    rdi,r13
    34c4:	49 89 f9             	mov    r9,rdi
    34c7:	49 d1 f9             	sar    r9,1
    34ca:	4d 89 f2             	mov    r10,r14
    34cd:	49 d1 fa             	sar    r10,1
    34d0:	b8 06 00 00 00       	mov    eax,0x6
    34d5:	4d 3b ca             	cmp    r9,r10
    34d8:	48 0f 4c 05 80 01 00 	cmovl  rax,QWORD PTR [rip+0x180]        # 3660 <range+0x2f0>
    34df:	00 
    34e0:	48 83 f8 06          	cmp    rax,0x6
    34e4:	40 0f 95 c6          	setne  sil
    34e8:	48 83 f8 02          	cmp    rax,0x2
    34ec:	40 0f 95 c7          	setne  dil
    34f0:	40 84 f7             	test   dil,sil
    34f3:	0f 85 aa 00 00 00    	jne    35a3 <range+0x233>
    34f9:	48 8d 0d 80 0b 01 02 	lea    rcx,[rip+0x2010b80]        # 2014080 <gc_sp>
    3500:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    3503:	4c 8d 05 76 0b 01 00 	lea    r8,[rip+0x10b76]        # 14080 <gc_stack>
    350a:	4c 6b ca 08          	imul   r9,rdx,0x8
    350e:	4c 89 fe             	mov    rsi,r15
    3511:	4b 89 34 08          	mov    QWORD PTR [r8+r9*1],rsi
    3515:	48 81 c2 01 00 00 00 	add    rdx,0x1
    351c:	48 89 11             	mov    QWORD PTR [rcx],rdx
    351f:	41 bf 01 00 00 00    	mov    r15d,0x1
    3525:	4c 8d 05 23 43 00 00 	lea    r8,[rip+0x4323]        # 784f <cljn_argv>
    352c:	4c 89 ff             	mov    rdi,r15
    352f:	41 ff d0             	call   r8
    3532:	bf 02 00 00 00       	mov    edi,0x2
    3537:	48 89 c2             	mov    rdx,rax
    353a:	4c 89 fe             	mov    rsi,r15
    353d:	e8 ba f6 ff ff       	call   2bfc <reverse>
    3542:	4c 8d 05 37 0b 01 02 	lea    r8,[rip+0x2010b37]        # 2014080 <gc_sp>
    3549:	49 83 00 ff          	add    QWORD PTR [r8],0xffffffffffffffff
    354d:	4c 8d 0d 2c 0b 01 02 	lea    r9,[rip+0x2010b2c]        # 2014080 <gc_sp>
    3554:	4d 8b 11             	mov    r10,QWORD PTR [r9]
    3557:	4c 8d 1d 22 0b 01 00 	lea    r11,[rip+0x10b22]        # 14080 <gc_stack>
    355e:	49 6b f2 08          	imul   rsi,r10,0x8
    3562:	49 89 04 33          	mov    QWORD PTR [r11+rsi*1],rax
    3566:	49 89 c7             	mov    r15,rax
    3569:	49 81 c2 01 00 00 00 	add    r10,0x1
    3570:	4d 89 11             	mov    QWORD PTR [r9],r10
    3573:	48 8d 35 d0 36 00 00 	lea    rsi,[rip+0x36d0]        # 6c4a <cljn_gc_leave>
    357a:	48 89 df             	mov    rdi,rbx
    357d:	ff d6                	call   rsi
    357f:	4c 89 f8             	mov    rax,r15
    3582:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    3586:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    358b:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    3590:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    3595:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    359a:	48 83 c4 30          	add    rsp,0x30
    359e:	48 89 ec             	mov    rsp,rbp
    35a1:	5d                   	pop    rbp
    35a2:	c3                   	ret
    35a3:	4c 89 ef             	mov    rdi,r13
    35a6:	48 f7 c7 01 00 00 00 	test   rdi,0x1
    35ad:	0f 84 2c 00 00 00    	je     35df <range+0x26f>
    35b3:	4c 89 ef             	mov    rdi,r13
    35b6:	49 89 f8             	mov    r8,rdi
    35b9:	49 d1 f8             	sar    r8,1
    35bc:	49 8d 40 01          	lea    rax,[r8+0x1]
    35c0:	48 3b 05 a1 00 00 00 	cmp    rax,QWORD PTR [rip+0xa1]        # 3668 <range+0x2f8>
    35c7:	41 0f 9d c1          	setge  r9b
    35cb:	48 3b 05 9e 00 00 00 	cmp    rax,QWORD PTR [rip+0x9e]        # 3670 <range+0x300>
    35d2:	41 0f 9e c2          	setle  r10b
    35d6:	45 84 ca             	test   r10b,r9b
    35d9:	0f 85 14 00 00 00    	jne    35f3 <range+0x283>
    35df:	48 8d 05 96 9c 00 00 	lea    rax,[rip+0x9c96]        # d27c <cljn_inc>
    35e6:	4c 89 ef             	mov    rdi,r13
    35e9:	ff d0                	call   rax
    35eb:	49 89 c4             	mov    r12,rax
    35ee:	e9 0a 00 00 00       	jmp    35fd <range+0x28d>
    35f3:	48 d1 e0             	shl    rax,1
    35f6:	48 83 c8 01          	or     rax,0x1
    35fa:	49 89 c4             	mov    r12,rax
    35fd:	48 8d 0d dd 40 00 00 	lea    rcx,[rip+0x40dd]        # 76e1 <cljn_cons>
    3604:	4c 89 fe             	mov    rsi,r15
    3607:	4c 89 ef             	mov    rdi,r13
    360a:	ff d1                	call   rcx
    360c:	48 8d 0d 6d 0a 01 02 	lea    rcx,[rip+0x2010a6d]        # 2014080 <gc_sp>
    3613:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    3616:	4c 8d 05 63 0a 01 00 	lea    r8,[rip+0x10a63]        # 14080 <gc_stack>
    361d:	4c 6b ca 08          	imul   r9,rdx,0x8
    3621:	49 89 c2             	mov    r10,rax
    3624:	4f 89 14 08          	mov    QWORD PTR [r8+r9*1],r10
    3628:	48 81 c2 01 00 00 00 	add    rdx,0x1
    362f:	48 89 11             	mov    QWORD PTR [rcx],rdx
    3632:	48 89 df             	mov    rdi,rbx
    3635:	4c 8d 47 02          	lea    r8,[rdi+0x2]
    3639:	4c 8d 0d 40 0a 01 00 	lea    r9,[rip+0x10a40]        # 14080 <gc_stack>
    3640:	4d 6b c0 08          	imul   r8,r8,0x8
    3644:	4f 89 14 01          	mov    QWORD PTR [r9+r8*1],r10
    3648:	4c 8d 0d 31 0a 01 02 	lea    r9,[rip+0x2010a31]        # 2014080 <gc_sp>
    364f:	49 83 01 ff          	add    QWORD PTR [r9],0xffffffffffffffff
    3653:	4d 89 d7             	mov    r15,r10
    3656:	4c 89 e7             	mov    rdi,r12
    3659:	e9 38 fe ff ff       	jmp    3496 <range+0x126>
    365e:	00 00                	add    BYTE PTR [rax],al
    3660:	0a 00                	or     al,BYTE PTR [rax]
	...
    366e:	00 c0                	add    al,al
    3670:	ff                   	(bad)
    3671:	ff                   	(bad)
    3672:	ff                   	(bad)
    3673:	ff                   	(bad)
    3674:	ff                   	(bad)
    3675:	ff                   	(bad)
    3676:	ff                   	(bad)
    3677:	3f                   	(bad)

0000000000003678 <__lambda_4>:
    3678:	55                   	push   rbp
    3679:	48 89 e5             	mov    rbp,rsp
    367c:	48 83 ec 20          	sub    rsp,0x20
    3680:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    3684:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    3689:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    368e:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    3693:	49 89 d5             	mov    r13,rdx
    3696:	49 89 f6             	mov    r14,rsi
    3699:	bf 02 00 00 00       	mov    edi,0x2
    369e:	4c 8d 0d ef 34 00 00 	lea    r9,[rip+0x34ef]        # 6b94 <cljn_gc_enter>
    36a5:	41 ff d1             	call   r9
    36a8:	4c 89 f7             	mov    rdi,r14
    36ab:	48 83 ff 02          	cmp    rdi,0x2
    36af:	0f 84 32 00 00 00    	je     36e7 <__lambda_4+0x6f>
    36b5:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    36bc:	4c 8d 1d b9 41 00 00 	lea    r11,[rip+0x41b9]        # 787c <cljn_check_arity>
    36c3:	41 ff d3             	call   r11
    36c6:	b8 02 00 00 00       	mov    eax,0x2
    36cb:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    36cf:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    36d4:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    36d9:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    36de:	48 83 c4 20          	add    rsp,0x20
    36e2:	48 89 ec             	mov    rsp,rbp
    36e5:	5d                   	pop    rbp
    36e6:	c3                   	ret
    36e7:	4c 89 ea             	mov    rdx,r13
    36ea:	48 8b 3a             	mov    rdi,QWORD PTR [rdx]
    36ed:	48 8d 08             	lea    rcx,[rax]
    36f0:	4c 8d 05 89 09 01 00 	lea    r8,[rip+0x10989]        # 14080 <gc_stack>
    36f7:	48 6b c9 08          	imul   rcx,rcx,0x8
    36fb:	49 89 3c 08          	mov    QWORD PTR [r8+rcx*1],rdi
    36ff:	48 8b 72 08          	mov    rsi,QWORD PTR [rdx+0x8]
    3703:	48 8d 48 01          	lea    rcx,[rax+0x1]
    3707:	48 89 c3             	mov    rbx,rax
    370a:	48 8d 05 6f 09 01 00 	lea    rax,[rip+0x1096f]        # 14080 <gc_stack>
    3711:	48 6b c9 08          	imul   rcx,rcx,0x8
    3715:	48 89 34 08          	mov    QWORD PTR [rax+rcx*1],rsi
    3719:	48 8d 15 86 91 00 00 	lea    rdx,[rip+0x9186]        # c8a6 <cljn_conj_bang>
    3720:	ff d2                	call   rdx
    3722:	48 8d 15 57 09 01 02 	lea    rdx,[rip+0x2010957]        # 2014080 <gc_sp>
    3729:	4c 8b 02             	mov    r8,QWORD PTR [rdx]
    372c:	4c 8d 0d 4d 09 01 00 	lea    r9,[rip+0x1094d]        # 14080 <gc_stack>
    3733:	4d 6b d0 08          	imul   r10,r8,0x8
    3737:	4b 89 04 11          	mov    QWORD PTR [r9+r10*1],rax
    373b:	49 89 c4             	mov    r12,rax
    373e:	49 81 c0 01 00 00 00 	add    r8,0x1
    3745:	4c 89 02             	mov    QWORD PTR [rdx],r8
    3748:	4c 8d 0d fb 34 00 00 	lea    r9,[rip+0x34fb]        # 6c4a <cljn_gc_leave>
    374f:	48 89 df             	mov    rdi,rbx
    3752:	41 ff d1             	call   r9
    3755:	4c 89 e0             	mov    rax,r12
    3758:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    375c:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    3761:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    3766:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    376b:	48 83 c4 20          	add    rsp,0x20
    376f:	48 89 ec             	mov    rsp,rbp
    3772:	5d                   	pop    rbp
    3773:	c3                   	ret

0000000000003774 <into>:
    3774:	55                   	push   rbp
    3775:	48 89 e5             	mov    rbp,rsp
    3778:	48 83 ec 20          	sub    rsp,0x20
    377c:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    3780:	4c 89 6c 24 08       	mov    QWORD PTR [rsp+0x8],r13
    3785:	4c 89 74 24 10       	mov    QWORD PTR [rsp+0x10],r14
    378a:	4c 89 7c 24 18       	mov    QWORD PTR [rsp+0x18],r15
    378f:	48 89 f3             	mov    rbx,rsi
    3792:	49 89 d5             	mov    r13,rdx
    3795:	bf 02 00 00 00       	mov    edi,0x2
    379a:	4c 8d 05 f3 33 00 00 	lea    r8,[rip+0x33f3]        # 6b94 <cljn_gc_enter>
    37a1:	41 ff d0             	call   r8
    37a4:	48 89 df             	mov    rdi,rbx
    37a7:	48 83 ff 02          	cmp    rdi,0x2
    37ab:	0f 84 32 00 00 00    	je     37e3 <into+0x6f>
    37b1:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    37b8:	4c 8d 15 bd 40 00 00 	lea    r10,[rip+0x40bd]        # 787c <cljn_check_arity>
    37bf:	41 ff d2             	call   r10
    37c2:	b8 02 00 00 00       	mov    eax,0x2
    37c7:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    37cb:	4c 8b 6c 24 08       	mov    r13,QWORD PTR [rsp+0x8]
    37d0:	4c 8b 74 24 10       	mov    r14,QWORD PTR [rsp+0x10]
    37d5:	4c 8b 7c 24 18       	mov    r15,QWORD PTR [rsp+0x18]
    37da:	48 83 c4 20          	add    rsp,0x20
    37de:	48 89 ec             	mov    rsp,rbp
    37e1:	5d                   	pop    rbp
    37e2:	c3                   	ret
    37e3:	4c 89 ea             	mov    rdx,r13
    37e6:	4c 8b 32             	mov    r14,QWORD PTR [rdx]
    37e9:	48 8d 30             	lea    rsi,[rax]
    37ec:	48 8d 3d 8d 08 01 00 	lea    rdi,[rip+0x1088d]        # 14080 <gc_stack>
    37f3:	48 6b f6 08          	imul   rsi,rsi,0x8
    37f7:	4c 89 34 37          	mov    QWORD PTR [rdi+rsi*1],r14
    37fb:	4c 8b 7a 08          	mov    r15,QWORD PTR [rdx+0x8]
    37ff:	48 8d 78 01          	lea    rdi,[rax+0x1]
    3803:	48 89 c3             	mov    rbx,rax
    3806:	48 8d 05 73 08 01 00 	lea    rax,[rip+0x10873]        # 14080 <gc_stack>
    380d:	48 6b ff 08          	imul   rdi,rdi,0x8
    3811:	4c 89 3c 38          	mov    QWORD PTR [rax+rdi*1],r15
    3815:	48 8d 3d 5c fe ff ff 	lea    rdi,[rip+0xfffffffffffffe5c]        # 3678 <__lambda_4>
    381c:	be 02 00 00 00       	mov    esi,0x2
    3821:	48 33 d2             	xor    rdx,rdx
    3824:	48 8d 0d fb 3e 00 00 	lea    rcx,[rip+0x3efb]        # 7726 <cljn_make_fn>
    382b:	ff d1                	call   rcx
    382d:	48 8d 0d 4c 08 01 02 	lea    rcx,[rip+0x201084c]        # 2014080 <gc_sp>
    3834:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    3837:	4c 8d 05 42 08 01 00 	lea    r8,[rip+0x10842]        # 14080 <gc_stack>
    383e:	4c 6b ca 08          	imul   r9,rdx,0x8
    3842:	4b 89 04 08          	mov    QWORD PTR [r8+r9*1],rax
    3846:	48 81 c2 01 00 00 00 	add    rdx,0x1
    384d:	48 89 11             	mov    QWORD PTR [rcx],rdx
    3850:	4c 8d 05 d8 8e 00 00 	lea    r8,[rip+0x8ed8]        # c72f <cljn_transient>
    3857:	4c 89 f7             	mov    rdi,r14
    385a:	41 ff d0             	call   r8
    385d:	4c 8d 05 1c 08 01 02 	lea    r8,[rip+0x201081c]        # 2014080 <gc_sp>
    3864:	4d 8b 08             	mov    r9,QWORD PTR [r8]
    3867:	4c 8d 15 12 08 01 00 	lea    r10,[rip+0x10812]        # 14080 <gc_stack>
    386e:	4d 6b d9 08          	imul   r11,r9,0x8
    3872:	4b 89 04 1a          	mov    QWORD PTR [r10+r11*1],rax
    3876:	49 81 c1 01 00 00 00 	add    r9,0x1
    387d:	4d 89 08             	mov    QWORD PTR [r8],r9
    3880:	4c 8d 15 f9 07 01 02 	lea    r10,[rip+0x20107f9]        # 2014080 <gc_sp>
    3887:	4d 8b 1a             	mov    r11,QWORD PTR [r10]
    388a:	48 8d 35 ef 07 01 00 	lea    rsi,[rip+0x107ef]        # 14080 <gc_stack>
    3891:	49 6b fb 08          	imul   rdi,r11,0x8
    3895:	4c 89 3c 3e          	mov    QWORD PTR [rsi+rdi*1],r15
    3899:	49 81 c3 01 00 00 00 	add    r11,0x1
    38a0:	4d 89 1a             	mov    QWORD PTR [r10],r11
    38a3:	41 bd 03 00 00 00    	mov    r13d,0x3
    38a9:	48 8d 35 9f 3f 00 00 	lea    rsi,[rip+0x3f9f]        # 784f <cljn_argv>
    38b0:	4c 89 ef             	mov    rdi,r13
    38b3:	ff d6                	call   rsi
    38b5:	bf 02 00 00 00       	mov    edi,0x2
    38ba:	48 89 c2             	mov    rdx,rax
    38bd:	4c 89 ee             	mov    rsi,r13
    38c0:	e8 17 e5 ff ff       	call   1ddc <reduce>
    38c5:	48 8d 35 b4 07 01 02 	lea    rsi,[rip+0x20107b4]        # 2014080 <gc_sp>
    38cc:	48 83 06 fd          	add    QWORD PTR [rsi],0xfffffffffffffffd
    38d0:	48 8d 3d a9 07 01 02 	lea    rdi,[rip+0x20107a9]        # 2014080 <gc_sp>
    38d7:	48 8b 0f             	mov    rcx,QWORD PTR [rdi]
    38da:	48 8d 15 9f 07 01 00 	lea    rdx,[rip+0x1079f]        # 14080 <gc_stack>
    38e1:	4c 6b c1 08          	imul   r8,rcx,0x8
    38e5:	4a 89 04 02          	mov    QWORD PTR [rdx+r8*1],rax
    38e9:	48 81 c1 01 00 00 00 	add    rcx,0x1
    38f0:	48 89 0f             	mov    QWORD PTR [rdi],rcx
    38f3:	48 8d 0d 51 94 00 00 	lea    rcx,[rip+0x9451]        # cd4b <cljn_persistent_bang>
    38fa:	48 89 c7             	mov    rdi,rax
    38fd:	ff d1                	call   rcx
    38ff:	48 8d 0d 7a 07 01 02 	lea    rcx,[rip+0x201077a]        # 2014080 <gc_sp>
    3906:	48 83 01 ff          	add    QWORD PTR [rcx],0xffffffffffffffff
    390a:	48 8d 15 6f 07 01 02 	lea    rdx,[rip+0x201076f]        # 2014080 <gc_sp>
    3911:	4c 8b 02             	mov    r8,QWORD PTR [rdx]
    3914:	4c 8d 0d 65 07 01 00 	lea    r9,[rip+0x10765]        # 14080 <gc_stack>
    391b:	4d 6b d0 08          	imul   r10,r8,0x8
    391f:	4b 89 04 11          	mov    QWORD PTR [r9+r10*1],rax
    3923:	49 89 c7             	mov    r15,rax
    3926:	49 81 c0 01 00 00 00 	add    r8,0x1
    392d:	4c 89 02             	mov    QWORD PTR [rdx],r8
    3930:	4c 8d 0d 13 33 00 00 	lea    r9,[rip+0x3313]        # 6c4a <cljn_gc_leave>
    3937:	48 89 df             	mov    rdi,rbx
    393a:	41 ff d1             	call   r9
    393d:	4c 89 f8             	mov    rax,r15
    3940:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    3944:	4c 8b 6c 24 08       	mov    r13,QWORD PTR [rsp+0x8]
    3949:	4c 8b 74 24 10       	mov    r14,QWORD PTR [rsp+0x10]
    394e:	4c 8b 7c 24 18       	mov    r15,QWORD PTR [rsp+0x18]
    3953:	48 83 c4 20          	add    rsp,0x20
    3957:	48 89 ec             	mov    rsp,rbp
    395a:	5d                   	pop    rbp
    395b:	c3                   	ret

000000000000395c <__lambda_5>:
    395c:	55                   	push   rbp
    395d:	48 89 e5             	mov    rbp,rsp
    3960:	48 83 ec 30          	sub    rsp,0x30
    3964:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    3968:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    396d:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    3972:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    3977:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
    397c:	48 89 fb             	mov    rbx,rdi
    397f:	49 89 f4             	mov    r12,rsi
    3982:	49 89 d6             	mov    r14,rdx
    3985:	bf 02 00 00 00       	mov    edi,0x2
    398a:	4c 8d 1d 03 32 00 00 	lea    r11,[rip+0x3203]        # 6b94 <cljn_gc_enter>
    3991:	41 ff d3             	call   r11
    3994:	4c 89 e1             	mov    rcx,r12
    3997:	48 83 f9 02          	cmp    rcx,0x2
    399b:	0f 84 39 00 00 00    	je     39da <__lambda_5+0x7e>
    39a1:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    39a8:	48 8d 05 cd 3e 00 00 	lea    rax,[rip+0x3ecd]        # 787c <cljn_check_arity>
    39af:	48 89 cf             	mov    rdi,rcx
    39b2:	ff d0                	call   rax
    39b4:	b8 02 00 00 00       	mov    eax,0x2
    39b9:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    39bd:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    39c2:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    39c7:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    39cc:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    39d1:	48 83 c4 30          	add    rsp,0x30
    39d5:	48 89 ec             	mov    rsp,rbp
    39d8:	5d                   	pop    rbp
    39d9:	c3                   	ret
    39da:	4c 89 f2             	mov    rdx,r14
    39dd:	4c 8b 2a             	mov    r13,QWORD PTR [rdx]
    39e0:	48 8d 08             	lea    rcx,[rax]
    39e3:	4c 8d 05 96 06 01 00 	lea    r8,[rip+0x10696]        # 14080 <gc_stack>
    39ea:	48 6b c9 08          	imul   rcx,rcx,0x8
    39ee:	4d 89 2c 08          	mov    QWORD PTR [r8+rcx*1],r13
    39f2:	4c 8b 72 08          	mov    r14,QWORD PTR [rdx+0x8]
    39f6:	48 8d 50 01          	lea    rdx,[rax+0x1]
    39fa:	49 89 c7             	mov    r15,rax
    39fd:	4c 8d 05 7c 06 01 00 	lea    r8,[rip+0x1067c]        # 14080 <gc_stack>
    3a04:	48 6b d2 08          	imul   rdx,rdx,0x8
    3a08:	4d 89 34 10          	mov    QWORD PTR [r8+rdx*1],r14
    3a0c:	48 33 f6             	xor    rsi,rsi
    3a0f:	4c 8d 0d cd 3d 00 00 	lea    r9,[rip+0x3dcd]        # 77e3 <cljn_fn_free>
    3a16:	48 89 df             	mov    rdi,rbx
    3a19:	41 ff d1             	call   r9
    3a1c:	49 89 c4             	mov    r12,rax
    3a1f:	4c 8d 0d 5a 06 01 02 	lea    r9,[rip+0x201065a]        # 2014080 <gc_sp>
    3a26:	4d 8b 11             	mov    r10,QWORD PTR [r9]
    3a29:	4c 8d 1d 50 06 01 00 	lea    r11,[rip+0x10650]        # 14080 <gc_stack>
    3a30:	49 6b f2 08          	imul   rsi,r10,0x8
    3a34:	49 89 04 33          	mov    QWORD PTR [r11+rsi*1],rax
    3a38:	49 81 c2 01 00 00 00 	add    r10,0x1
    3a3f:	4d 89 11             	mov    QWORD PTR [r9],r10
    3a42:	4c 8d 1d 37 06 01 02 	lea    r11,[rip+0x2010637]        # 2014080 <gc_sp>
    3a49:	49 8b 33             	mov    rsi,QWORD PTR [r11]
    3a4c:	48 8d 3d 2d 06 01 00 	lea    rdi,[rip+0x1062d]        # 14080 <gc_stack>
    3a53:	48 6b c6 08          	imul   rax,rsi,0x8
    3a57:	4c 89 34 07          	mov    QWORD PTR [rdi+rax*1],r14
    3a5b:	48 81 c6 01 00 00 00 	add    rsi,0x1
    3a62:	49 89 33             	mov    QWORD PTR [r11],rsi
    3a65:	48 8d 05 b0 3d 00 00 	lea    rax,[rip+0x3db0]        # 781c <cljn_check_fn>
    3a6c:	4c 89 e7             	mov    rdi,r12
    3a6f:	ff d0                	call   rax
    3a71:	bb 01 00 00 00       	mov    ebx,0x1
    3a76:	48 8d 05 d2 3d 00 00 	lea    rax,[rip+0x3dd2]        # 784f <cljn_argv>
    3a7d:	48 89 df             	mov    rdi,rbx
    3a80:	ff d0                	call   rax
    3a82:	49 89 c6             	mov    r14,rax
    3a85:	48 8d 0d 7a 3d 00 00 	lea    rcx,[rip+0x3d7a]        # 7806 <cljn_fn_code>
    3a8c:	4c 89 e7             	mov    rdi,r12
    3a8f:	ff d1                	call   rcx
    3a91:	4c 89 f2             	mov    rdx,r14
    3a94:	48 89 de             	mov    rsi,rbx
    3a97:	4c 89 e7             	mov    rdi,r12
    3a9a:	ff d0                	call   rax
    3a9c:	48 8d 0d dd 05 01 02 	lea    rcx,[rip+0x20105dd]        # 2014080 <gc_sp>
    3aa3:	48 83 01 fe          	add    QWORD PTR [rcx],0xfffffffffffffffe
    3aa7:	48 8d 0d d2 05 01 02 	lea    rcx,[rip+0x20105d2]        # 2014080 <gc_sp>
    3aae:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    3ab1:	4c 8d 05 c8 05 01 00 	lea    r8,[rip+0x105c8]        # 14080 <gc_stack>
    3ab8:	4c 6b ca 08          	imul   r9,rdx,0x8
    3abc:	4b 89 04 08          	mov    QWORD PTR [r8+r9*1],rax
    3ac0:	48 81 c2 01 00 00 00 	add    rdx,0x1
    3ac7:	48 89 11             	mov    QWORD PTR [rcx],rdx
    3aca:	4c 8d 05 d5 8d 00 00 	lea    r8,[rip+0x8dd5]        # c8a6 <cljn_conj_bang>
    3ad1:	48 89 c6             	mov    rsi,rax
    3ad4:	4c 89 ef             	mov    rdi,r13
    3ad7:	41 ff d0             	call   r8
    3ada:	4c 8d 05 9f 05 01 02 	lea    r8,[rip+0x201059f]        # 2014080 <gc_sp>
    3ae1:	49 83 00 ff          	add    QWORD PTR [r8],0xffffffffffffffff
    3ae5:	4c 8d 0d 94 05 01 02 	lea    r9,[rip+0x2010594]        # 2014080 <gc_sp>
    3aec:	4d 8b 11             	mov    r10,QWORD PTR [r9]
    3aef:	4c 8d 1d 8a 05 01 00 	lea    r11,[rip+0x1058a]        # 14080 <gc_stack>
    3af6:	49 6b f2 08          	imul   rsi,r10,0x8
    3afa:	49 89 04 33          	mov    QWORD PTR [r11+rsi*1],rax
    3afe:	48 89 c3             	mov    rbx,rax
    3b01:	49 81 c2 01 00 00 00 	add    r10,0x1
    3b08:	4d 89 11             	mov    QWORD PTR [r9],r10
    3b0b:	4c 8d 1d 38 31 00 00 	lea    r11,[rip+0x3138]        # 6c4a <cljn_gc_leave>
    3b12:	4c 89 ff             	mov    rdi,r15
    3b15:	41 ff d3             	call   r11
    3b18:	48 89 d8             	mov    rax,rbx
    3b1b:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    3b1f:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    3b24:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    3b29:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    3b2e:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    3b33:	48 83 c4 30          	add    rsp,0x30
    3b37:	48 89 ec             	mov    rsp,rbp
    3b3a:	5d                   	pop    rbp
    3b3b:	c3                   	ret

0000000000003b3c <mapv>:
    3b3c:	55                   	push   rbp
    3b3d:	48 89 e5             	mov    rbp,rsp
    3b40:	48 83 ec 30          	sub    rsp,0x30
    3b44:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    3b48:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    3b4d:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    3b52:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    3b57:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
    3b5c:	49 89 d4             	mov    r12,rdx
    3b5f:	49 89 f7             	mov    r15,rsi
    3b62:	bf 02 00 00 00       	mov    edi,0x2
    3b67:	48 8d 35 26 30 00 00 	lea    rsi,[rip+0x3026]        # 6b94 <cljn_gc_enter>
    3b6e:	ff d6                	call   rsi
    3b70:	4d 89 fa             	mov    r10,r15
    3b73:	49 83 fa 02          	cmp    r10,0x2
    3b77:	0f 84 39 00 00 00    	je     3bb6 <mapv+0x7a>
    3b7d:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    3b84:	48 8d 05 f1 3c 00 00 	lea    rax,[rip+0x3cf1]        # 787c <cljn_check_arity>
    3b8b:	4c 89 d7             	mov    rdi,r10
    3b8e:	ff d0                	call   rax
    3b90:	b8 02 00 00 00       	mov    eax,0x2
    3b95:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    3b99:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    3b9e:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    3ba3:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    3ba8:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    3bad:	48 83 c4 30          	add    rsp,0x30
    3bb1:	48 89 ec             	mov    rsp,rbp
    3bb4:	5d                   	pop    rbp
    3bb5:	c3                   	ret
    3bb6:	4c 89 e2             	mov    rdx,r12
    3bb9:	48 8b 1a             	mov    rbx,QWORD PTR [rdx]
    3bbc:	4c 8d 00             	lea    r8,[rax]
    3bbf:	4c 8d 0d ba 04 01 00 	lea    r9,[rip+0x104ba]        # 14080 <gc_stack>
    3bc6:	4d 6b c0 08          	imul   r8,r8,0x8
    3bca:	4b 89 1c 01          	mov    QWORD PTR [r9+r8*1],rbx
    3bce:	4c 8b 62 08          	mov    r12,QWORD PTR [rdx+0x8]
    3bd2:	4c 8d 40 01          	lea    r8,[rax+0x1]
    3bd6:	49 89 c5             	mov    r13,rax
    3bd9:	4c 8d 0d a0 04 01 00 	lea    r9,[rip+0x104a0]        # 14080 <gc_stack>
    3be0:	4d 6b c0 08          	imul   r8,r8,0x8
    3be4:	4f 89 24 01          	mov    QWORD PTR [r9+r8*1],r12
    3be8:	4c 8d 15 91 04 01 02 	lea    r10,[rip+0x2010491]        # 2014080 <gc_sp>
    3bef:	4d 8b 1a             	mov    r11,QWORD PTR [r10]
    3bf2:	48 8d 35 87 04 01 00 	lea    rsi,[rip+0x10487]        # 14080 <gc_stack>
    3bf9:	49 6b fb 08          	imul   rdi,r11,0x8
    3bfd:	48 89 1c 3e          	mov    QWORD PTR [rsi+rdi*1],rbx
    3c01:	49 81 c3 01 00 00 00 	add    r11,0x1
    3c08:	4d 89 1a             	mov    QWORD PTR [r10],r11
    3c0b:	48 8d 3d 4a fd ff ff 	lea    rdi,[rip+0xfffffffffffffd4a]        # 395c <__lambda_5>
    3c12:	be 02 00 00 00       	mov    esi,0x2
    3c17:	ba 01 00 00 00       	mov    edx,0x1
    3c1c:	48 8d 05 03 3b 00 00 	lea    rax,[rip+0x3b03]        # 7726 <cljn_make_fn>
    3c23:	ff d0                	call   rax
    3c25:	48 8d 35 54 04 01 02 	lea    rsi,[rip+0x2010454]        # 2014080 <gc_sp>
    3c2c:	48 83 06 ff          	add    QWORD PTR [rsi],0xffffffffffffffff
    3c30:	48 8d 3d 49 04 01 02 	lea    rdi,[rip+0x2010449]        # 2014080 <gc_sp>
    3c37:	48 8b 0f             	mov    rcx,QWORD PTR [rdi]
    3c3a:	48 8d 15 3f 04 01 00 	lea    rdx,[rip+0x1043f]        # 14080 <gc_stack>
    3c41:	4c 6b c1 08          	imul   r8,rcx,0x8
    3c45:	4a 89 04 02          	mov    QWORD PTR [rdx+r8*1],rax
    3c49:	48 81 c1 01 00 00 00 	add    rcx,0x1
    3c50:	48 89 0f             	mov    QWORD PTR [rdi],rcx
    3c53:	48 33 f6             	xor    rsi,rsi
    3c56:	48 8d 0d 5a 3b 00 00 	lea    rcx,[rip+0x3b5a]        # 77b7 <cljn_fn_set_free>
    3c5d:	48 89 da             	mov    rdx,rbx
    3c60:	48 89 c7             	mov    rdi,rax
    3c63:	ff d1                	call   rcx
    3c65:	48 8d 15 c3 42 00 00 	lea    rdx,[rip+0x42c3]        # 7f2f <cljn_vec_empty>
    3c6c:	ff d2                	call   rdx
    3c6e:	48 8d 15 0b 04 01 02 	lea    rdx,[rip+0x201040b]        # 2014080 <gc_sp>
    3c75:	4c 8b 02             	mov    r8,QWORD PTR [rdx]
    3c78:	4c 8d 0d 01 04 01 00 	lea    r9,[rip+0x10401]        # 14080 <gc_stack>
    3c7f:	4d 6b d0 08          	imul   r10,r8,0x8
    3c83:	4b 89 04 11          	mov    QWORD PTR [r9+r10*1],rax
    3c87:	49 81 c0 01 00 00 00 	add    r8,0x1
    3c8e:	4c 89 02             	mov    QWORD PTR [rdx],r8
    3c91:	4c 8d 0d e8 03 01 02 	lea    r9,[rip+0x20103e8]        # 2014080 <gc_sp>
    3c98:	49 83 01 ff          	add    QWORD PTR [r9],0xffffffffffffffff
    3c9c:	4c 8d 15 dd 03 01 02 	lea    r10,[rip+0x20103dd]        # 2014080 <gc_sp>
    3ca3:	4d 8b 1a             	mov    r11,QWORD PTR [r10]
    3ca6:	48 8d 35 d3 03 01 00 	lea    rsi,[rip+0x103d3]        # 14080 <gc_stack>
    3cad:	49 6b fb 08          	imul   rdi,r11,0x8
    3cb1:	48 89 04 3e          	mov    QWORD PTR [rsi+rdi*1],rax
    3cb5:	49 81 c3 01 00 00 00 	add    r11,0x1
    3cbc:	4d 89 1a             	mov    QWORD PTR [r10],r11
    3cbf:	48 8d 35 69 8a 00 00 	lea    rsi,[rip+0x8a69]        # c72f <cljn_transient>
    3cc6:	48 89 c7             	mov    rdi,rax
    3cc9:	ff d6                	call   rsi
    3ccb:	48 8d 35 ae 03 01 02 	lea    rsi,[rip+0x20103ae]        # 2014080 <gc_sp>
    3cd2:	48 83 06 ff          	add    QWORD PTR [rsi],0xffffffffffffffff
    3cd6:	48 8d 3d a3 03 01 02 	lea    rdi,[rip+0x20103a3]        # 2014080 <gc_sp>
    3cdd:	48 8b 0f             	mov    rcx,QWORD PTR [rdi]
    3ce0:	48 8d 15 99 03 01 00 	lea    rdx,[rip+0x10399]        # 14080 <gc_stack>
    3ce7:	4c 6b c1 08          	imul   r8,rcx,0x8
    3ceb:	4a 89 04 02          	mov    QWORD PTR [rdx+r8*1],rax
    3cef:	48 8d 41 01          	lea    rax,[rcx+0x1]
    3cf3:	48 89 07             	mov    QWORD PTR [rdi],rax
    3cf6:	48 8d 0d 83 03 01 02 	lea    rcx,[rip+0x2010383]        # 2014080 <gc_sp>
    3cfd:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    3d00:	4c 8d 05 79 03 01 00 	lea    r8,[rip+0x10379]        # 14080 <gc_stack>
    3d07:	4c 6b ca 08          	imul   r9,rdx,0x8
    3d0b:	4f 89 24 08          	mov    QWORD PTR [r8+r9*1],r12
    3d0f:	48 81 c2 01 00 00 00 	add    rdx,0x1
    3d16:	48 89 11             	mov    QWORD PTR [rcx],rdx
    3d19:	41 bf 03 00 00 00    	mov    r15d,0x3
    3d1f:	4c 8d 05 29 3b 00 00 	lea    r8,[rip+0x3b29]        # 784f <cljn_argv>
    3d26:	4c 89 ff             	mov    rdi,r15
    3d29:	41 ff d0             	call   r8
    3d2c:	bf 02 00 00 00       	mov    edi,0x2
    3d31:	48 89 c2             	mov    rdx,rax
    3d34:	4c 89 fe             	mov    rsi,r15
    3d37:	e8 a0 e0 ff ff       	call   1ddc <reduce>
    3d3c:	4c 8d 05 3d 03 01 02 	lea    r8,[rip+0x201033d]        # 2014080 <gc_sp>
    3d43:	49 83 00 fd          	add    QWORD PTR [r8],0xfffffffffffffffd
    3d47:	4c 8d 0d 32 03 01 02 	lea    r9,[rip+0x2010332]        # 2014080 <gc_sp>
    3d4e:	4d 8b 11             	mov    r10,QWORD PTR [r9]
    3d51:	4c 8d 1d 28 03 01 00 	lea    r11,[rip+0x10328]        # 14080 <gc_stack>
    3d58:	49 6b f2 08          	imul   rsi,r10,0x8
    3d5c:	49 89 04 33          	mov    QWORD PTR [r11+rsi*1],rax
    3d60:	49 81 c2 01 00 00 00 	add    r10,0x1
    3d67:	4d 89 11             	mov    QWORD PTR [r9],r10
    3d6a:	4c 8d 1d da 8f 00 00 	lea    r11,[rip+0x8fda]        # cd4b <cljn_persistent_bang>
    3d71:	48 89 c7             	mov    rdi,rax
    3d74:	41 ff d3             	call   r11
    3d77:	4c 8d 1d 02 03 01 02 	lea    r11,[rip+0x2010302]        # 2014080 <gc_sp>
    3d7e:	49 83 03 ff          	add    QWORD PTR [r11],0xffffffffffffffff
    3d82:	48 8d 35 f7 02 01 02 	lea    rsi,[rip+0x20102f7]        # 2014080 <gc_sp>
    3d89:	48 8b 3e             	mov    rdi,QWORD PTR [rsi]
    3d8c:	48 8d 0d ed 02 01 00 	lea    rcx,[rip+0x102ed]        # 14080 <gc_stack>
    3d93:	48 6b d7 08          	imul   rdx,rdi,0x8
    3d97:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    3d9b:	49 89 c6             	mov    r14,rax
    3d9e:	48 81 c7 01 00 00 00 	add    rdi,0x1
    3da5:	48 89 3e             	mov    QWORD PTR [rsi],rdi
    3da8:	48 8d 05 9b 2e 00 00 	lea    rax,[rip+0x2e9b]        # 6c4a <cljn_gc_leave>
    3daf:	4c 89 ef             	mov    rdi,r13
    3db2:	ff d0                	call   rax
    3db4:	4c 89 f0             	mov    rax,r14
    3db7:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    3dbb:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    3dc0:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    3dc5:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    3dca:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    3dcf:	48 83 c4 30          	add    rsp,0x30
    3dd3:	48 89 ec             	mov    rsp,rbp
    3dd6:	5d                   	pop    rbp
    3dd7:	c3                   	ret

0000000000003dd8 <every?>:
    3dd8:	55                   	push   rbp
    3dd9:	48 89 e5             	mov    rbp,rsp
    3ddc:	48 83 ec 30          	sub    rsp,0x30
    3de0:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    3de4:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    3de9:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    3dee:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    3df3:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
    3df8:	49 89 d4             	mov    r12,rdx
    3dfb:	49 89 f7             	mov    r15,rsi
    3dfe:	bf 02 00 00 00       	mov    edi,0x2
    3e03:	48 8d 15 8a 2d 00 00 	lea    rdx,[rip+0x2d8a]        # 6b94 <cljn_gc_enter>
    3e0a:	ff d2                	call   rdx
    3e0c:	4c 89 ff             	mov    rdi,r15
    3e0f:	48 83 ff 02          	cmp    rdi,0x2
    3e13:	0f 84 37 00 00 00    	je     3e50 <every?+0x78>
    3e19:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    3e20:	4c 8d 0d 55 3a 00 00 	lea    r9,[rip+0x3a55]        # 787c <cljn_check_arity>
    3e27:	41 ff d1             	call   r9
    3e2a:	b8 02 00 00 00       	mov    eax,0x2
    3e2f:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    3e33:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    3e38:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    3e3d:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    3e42:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    3e47:	48 83 c4 30          	add    rsp,0x30
    3e4b:	48 89 ec             	mov    rsp,rbp
    3e4e:	5d                   	pop    rbp
    3e4f:	c3                   	ret
    3e50:	4c 89 e2             	mov    rdx,r12
    3e53:	4c 8b 22             	mov    r12,QWORD PTR [rdx]
    3e56:	4c 8d 18             	lea    r11,[rax]
    3e59:	48 8d 35 20 02 01 00 	lea    rsi,[rip+0x10220]        # 14080 <gc_stack>
    3e60:	4d 6b db 08          	imul   r11,r11,0x8
    3e64:	4e 89 24 1e          	mov    QWORD PTR [rsi+r11*1],r12
    3e68:	48 8b 7a 08          	mov    rdi,QWORD PTR [rdx+0x8]
    3e6c:	48 8d 70 01          	lea    rsi,[rax+0x1]
    3e70:	49 89 c7             	mov    r15,rax
    3e73:	48 8d 05 06 02 01 00 	lea    rax,[rip+0x10206]        # 14080 <gc_stack>
    3e7a:	48 6b f6 08          	imul   rsi,rsi,0x8
    3e7e:	48 89 3c 30          	mov    QWORD PTR [rax+rsi*1],rdi
    3e82:	49 89 fd             	mov    r13,rdi
    3e85:	48 8d 05 6c 9e 00 00 	lea    rax,[rip+0x9e6c]        # dcf8 <cljn_emptyp>
    3e8c:	4c 89 ef             	mov    rdi,r13
    3e8f:	ff d0                	call   rax
    3e91:	48 83 f8 06          	cmp    rax,0x6
    3e95:	0f 95 c1             	setne  cl
    3e98:	48 83 f8 02          	cmp    rax,0x2
    3e9c:	0f 95 c2             	setne  dl
    3e9f:	84 ca                	test   dl,cl
    3ea1:	0f 85 50 01 00 00    	jne    3ff7 <every?+0x21f>
    3ea7:	4c 8d 0d d2 01 01 02 	lea    r9,[rip+0x20101d2]        # 2014080 <gc_sp>
    3eae:	4d 8b 11             	mov    r10,QWORD PTR [r9]
    3eb1:	4c 8d 1d c8 01 01 00 	lea    r11,[rip+0x101c8]        # 14080 <gc_stack>
    3eb8:	49 6b f2 08          	imul   rsi,r10,0x8
    3ebc:	4d 89 24 33          	mov    QWORD PTR [r11+rsi*1],r12
    3ec0:	49 81 c2 01 00 00 00 	add    r10,0x1
    3ec7:	4d 89 11             	mov    QWORD PTR [r9],r10
    3eca:	4c 8d 1d 4a 9f 00 00 	lea    r11,[rip+0x9f4a]        # de1b <cljn_first>
    3ed1:	4c 89 ef             	mov    rdi,r13
    3ed4:	41 ff d3             	call   r11
    3ed7:	4c 8d 1d a2 01 01 02 	lea    r11,[rip+0x20101a2]        # 2014080 <gc_sp>
    3ede:	49 8b 33             	mov    rsi,QWORD PTR [r11]
    3ee1:	48 8d 3d 98 01 01 00 	lea    rdi,[rip+0x10198]        # 14080 <gc_stack>
    3ee8:	48 6b ce 08          	imul   rcx,rsi,0x8
    3eec:	48 89 04 0f          	mov    QWORD PTR [rdi+rcx*1],rax
    3ef0:	48 81 c6 01 00 00 00 	add    rsi,0x1
    3ef7:	49 89 33             	mov    QWORD PTR [r11],rsi
    3efa:	48 8d 05 1b 39 00 00 	lea    rax,[rip+0x391b]        # 781c <cljn_check_fn>
    3f01:	4c 89 e7             	mov    rdi,r12
    3f04:	ff d0                	call   rax
    3f06:	41 be 01 00 00 00    	mov    r14d,0x1
    3f0c:	48 8d 05 3c 39 00 00 	lea    rax,[rip+0x393c]        # 784f <cljn_argv>
    3f13:	4c 89 f7             	mov    rdi,r14
    3f16:	ff d0                	call   rax
    3f18:	48 89 c3             	mov    rbx,rax
    3f1b:	48 8d 0d e4 38 00 00 	lea    rcx,[rip+0x38e4]        # 7806 <cljn_fn_code>
    3f22:	4c 89 e7             	mov    rdi,r12
    3f25:	ff d1                	call   rcx
    3f27:	48 89 da             	mov    rdx,rbx
    3f2a:	4c 89 f6             	mov    rsi,r14
    3f2d:	4c 89 e7             	mov    rdi,r12
    3f30:	ff d0                	call   rax
    3f32:	48 8d 0d 47 01 01 02 	lea    rcx,[rip+0x2010147]        # 2014080 <gc_sp>
    3f39:	48 83 01 fe          	add    QWORD PTR [rcx],0xfffffffffffffffe
    3f3d:	48 8d 0d 3c 01 01 02 	lea    rcx,[rip+0x201013c]        # 2014080 <gc_sp>
    3f44:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    3f47:	4c 8d 05 32 01 01 00 	lea    r8,[rip+0x10132]        # 14080 <gc_stack>
    3f4e:	4c 6b ca 08          	imul   r9,rdx,0x8
    3f52:	4b 89 04 08          	mov    QWORD PTR [r8+r9*1],rax
    3f56:	48 81 c2 01 00 00 00 	add    rdx,0x1
    3f5d:	48 89 11             	mov    QWORD PTR [rcx],rdx
    3f60:	4c 8d 05 18 9d 00 00 	lea    r8,[rip+0x9d18]        # dc7f <cljn_truthy>
    3f67:	48 89 c7             	mov    rdi,rax
    3f6a:	41 ff d0             	call   r8
    3f6d:	4c 8d 05 0c 01 01 02 	lea    r8,[rip+0x201010c]        # 2014080 <gc_sp>
    3f74:	49 83 00 ff          	add    QWORD PTR [r8],0xffffffffffffffff
    3f78:	85 c0                	test   eax,eax
    3f7a:	0f 85 0d 00 00 00    	jne    3f8d <every?+0x1b5>
    3f80:	b8 06 00 00 00       	mov    eax,0x6
    3f85:	49 89 c4             	mov    r12,rax
    3f88:	e9 72 00 00 00       	jmp    3fff <every?+0x227>
    3f8d:	48 8d 05 d6 9f 00 00 	lea    rax,[rip+0x9fd6]        # df6a <cljn_rest>
    3f94:	4c 89 ef             	mov    rdi,r13
    3f97:	ff d0                	call   rax
    3f99:	48 8d 3d e0 00 01 02 	lea    rdi,[rip+0x20100e0]        # 2014080 <gc_sp>
    3fa0:	48 8b 0f             	mov    rcx,QWORD PTR [rdi]
    3fa3:	48 8d 15 d6 00 01 00 	lea    rdx,[rip+0x100d6]        # 14080 <gc_stack>
    3faa:	4c 6b c1 08          	imul   r8,rcx,0x8
    3fae:	4a 89 04 02          	mov    QWORD PTR [rdx+r8*1],rax
    3fb2:	48 81 c1 01 00 00 00 	add    rcx,0x1
    3fb9:	48 89 0f             	mov    QWORD PTR [rdi],rcx
    3fbc:	4d 89 f9             	mov    r9,r15
    3fbf:	49 8d 09             	lea    rcx,[r9]
    3fc2:	48 8d 15 b7 00 01 00 	lea    rdx,[rip+0x100b7]        # 14080 <gc_stack>
    3fc9:	48 6b c9 08          	imul   rcx,rcx,0x8
    3fcd:	4c 89 24 0a          	mov    QWORD PTR [rdx+rcx*1],r12
    3fd1:	49 8d 51 01          	lea    rdx,[r9+0x1]
    3fd5:	4c 8d 05 a4 00 01 00 	lea    r8,[rip+0x100a4]        # 14080 <gc_stack>
    3fdc:	48 6b d2 08          	imul   rdx,rdx,0x8
    3fe0:	49 89 04 10          	mov    QWORD PTR [r8+rdx*1],rax
    3fe4:	4c 8d 05 95 00 01 02 	lea    r8,[rip+0x2010095]        # 2014080 <gc_sp>
    3feb:	49 83 00 ff          	add    QWORD PTR [r8],0xffffffffffffffff
    3fef:	49 89 c5             	mov    r13,rax
    3ff2:	e9 8e fe ff ff       	jmp    3e85 <every?+0xad>
    3ff7:	b8 0a 00 00 00       	mov    eax,0xa
    3ffc:	49 89 c4             	mov    r12,rax
    3fff:	4c 8d 1d 44 2c 00 00 	lea    r11,[rip+0x2c44]        # 6c4a <cljn_gc_leave>
    4006:	4c 89 ff             	mov    rdi,r15
    4009:	41 ff d3             	call   r11
    400c:	4c 89 e0             	mov    rax,r12
    400f:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    4013:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    4018:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    401d:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    4022:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    4027:	48 83 c4 30          	add    rsp,0x30
    402b:	48 89 ec             	mov    rsp,rbp
    402e:	5d                   	pop    rbp
    402f:	c3                   	ret

0000000000004030 <some>:
    4030:	55                   	push   rbp
    4031:	48 89 e5             	mov    rbp,rsp
    4034:	48 83 ec 30          	sub    rsp,0x30
    4038:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    403c:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    4041:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    4046:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    404b:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
    4050:	48 89 d3             	mov    rbx,rdx
    4053:	49 89 f6             	mov    r14,rsi
    4056:	bf 03 00 00 00       	mov    edi,0x3
    405b:	4c 8d 15 32 2b 00 00 	lea    r10,[rip+0x2b32]        # 6b94 <cljn_gc_enter>
    4062:	41 ff d2             	call   r10
    4065:	4c 89 f1             	mov    rcx,r14
    4068:	48 83 f9 02          	cmp    rcx,0x2
    406c:	0f 84 39 00 00 00    	je     40ab <some+0x7b>
    4072:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    4079:	48 8d 05 fc 37 00 00 	lea    rax,[rip+0x37fc]        # 787c <cljn_check_arity>
    4080:	48 89 cf             	mov    rdi,rcx
    4083:	ff d0                	call   rax
    4085:	b8 02 00 00 00       	mov    eax,0x2
    408a:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    408e:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    4093:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    4098:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    409d:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    40a2:	48 83 c4 30          	add    rsp,0x30
    40a6:	48 89 ec             	mov    rsp,rbp
    40a9:	5d                   	pop    rbp
    40aa:	c3                   	ret
    40ab:	48 89 da             	mov    rdx,rbx
    40ae:	4c 8b 2a             	mov    r13,QWORD PTR [rdx]
    40b1:	48 8d 08             	lea    rcx,[rax]
    40b4:	4c 8d 05 c5 ff 00 00 	lea    r8,[rip+0xffc5]        # 14080 <gc_stack>
    40bb:	48 6b c9 08          	imul   rcx,rcx,0x8
    40bf:	4d 89 2c 08          	mov    QWORD PTR [r8+rcx*1],r13
    40c3:	4c 8b 72 08          	mov    r14,QWORD PTR [rdx+0x8]
    40c7:	48 8d 48 01          	lea    rcx,[rax+0x1]
    40cb:	48 89 c3             	mov    rbx,rax
    40ce:	48 8d 15 ab ff 00 00 	lea    rdx,[rip+0xffab]        # 14080 <gc_stack>
    40d5:	48 6b c9 08          	imul   rcx,rcx,0x8
    40d9:	4c 89 34 0a          	mov    QWORD PTR [rdx+rcx*1],r14
    40dd:	4c 8d 05 14 9c 00 00 	lea    r8,[rip+0x9c14]        # dcf8 <cljn_emptyp>
    40e4:	4c 89 f7             	mov    rdi,r14
    40e7:	41 ff d0             	call   r8
    40ea:	48 83 f8 06          	cmp    rax,0x6
    40ee:	41 0f 95 c1          	setne  r9b
    40f2:	48 83 f8 02          	cmp    rax,0x2
    40f6:	41 0f 95 c2          	setne  r10b
    40fa:	45 84 ca             	test   r10b,r9b
    40fd:	0f 85 c7 01 00 00    	jne    42ca <some+0x29a>
    4103:	48 8d 35 76 ff 00 02 	lea    rsi,[rip+0x200ff76]        # 2014080 <gc_sp>
    410a:	48 8b 3e             	mov    rdi,QWORD PTR [rsi]
    410d:	48 8d 05 6c ff 00 00 	lea    rax,[rip+0xff6c]        # 14080 <gc_stack>
    4114:	48 6b cf 08          	imul   rcx,rdi,0x8
    4118:	4c 89 2c 08          	mov    QWORD PTR [rax+rcx*1],r13
    411c:	48 81 c7 01 00 00 00 	add    rdi,0x1
    4123:	48 89 3e             	mov    QWORD PTR [rsi],rdi
    4126:	48 8d 05 ee 9c 00 00 	lea    rax,[rip+0x9cee]        # de1b <cljn_first>
    412d:	4c 89 f7             	mov    rdi,r14
    4130:	ff d0                	call   rax
    4132:	49 89 c2             	mov    r10,rax
    4135:	48 8d 05 44 ff 00 02 	lea    rax,[rip+0x200ff44]        # 2014080 <gc_sp>
    413c:	48 8b 08             	mov    rcx,QWORD PTR [rax]
    413f:	48 8d 15 3a ff 00 00 	lea    rdx,[rip+0xff3a]        # 14080 <gc_stack>
    4146:	4c 6b c1 08          	imul   r8,rcx,0x8
    414a:	4e 89 14 02          	mov    QWORD PTR [rdx+r8*1],r10
    414e:	48 81 c1 01 00 00 00 	add    rcx,0x1
    4155:	48 89 08             	mov    QWORD PTR [rax],rcx
    4158:	48 8d 15 bd 36 00 00 	lea    rdx,[rip+0x36bd]        # 781c <cljn_check_fn>
    415f:	4c 89 ef             	mov    rdi,r13
    4162:	ff d2                	call   rdx
    4164:	41 bf 01 00 00 00    	mov    r15d,0x1
    416a:	4c 8d 05 de 36 00 00 	lea    r8,[rip+0x36de]        # 784f <cljn_argv>
    4171:	4c 89 ff             	mov    rdi,r15
    4174:	41 ff d0             	call   r8
    4177:	49 89 c4             	mov    r12,rax
    417a:	4c 8d 05 85 36 00 00 	lea    r8,[rip+0x3685]        # 7806 <cljn_fn_code>
    4181:	4c 89 ef             	mov    rdi,r13
    4184:	41 ff d0             	call   r8
    4187:	4c 89 e2             	mov    rdx,r12
    418a:	4c 89 fe             	mov    rsi,r15
    418d:	4c 89 ef             	mov    rdi,r13
    4190:	ff d0                	call   rax
    4192:	4c 8d 05 e7 fe 00 02 	lea    r8,[rip+0x200fee7]        # 2014080 <gc_sp>
    4199:	49 83 00 fe          	add    QWORD PTR [r8],0xfffffffffffffffe
    419d:	4c 8d 0d dc fe 00 02 	lea    r9,[rip+0x200fedc]        # 2014080 <gc_sp>
    41a4:	4d 8b 11             	mov    r10,QWORD PTR [r9]
    41a7:	4c 8d 1d d2 fe 00 00 	lea    r11,[rip+0xfed2]        # 14080 <gc_stack>
    41ae:	49 6b f2 08          	imul   rsi,r10,0x8
    41b2:	49 89 04 33          	mov    QWORD PTR [r11+rsi*1],rax
    41b6:	49 81 c2 01 00 00 00 	add    r10,0x1
    41bd:	4d 89 11             	mov    QWORD PTR [r9],r10
    41c0:	48 89 df             	mov    rdi,rbx
    41c3:	4c 8d 5f 02          	lea    r11,[rdi+0x2]
    41c7:	48 8d 35 b2 fe 00 00 	lea    rsi,[rip+0xfeb2]        # 14080 <gc_stack>
    41ce:	4d 6b db 08          	imul   r11,r11,0x8
    41d2:	4a 89 04 1e          	mov    QWORD PTR [rsi+r11*1],rax
    41d6:	48 8d 35 a3 fe 00 02 	lea    rsi,[rip+0x200fea3]        # 2014080 <gc_sp>
    41dd:	48 83 06 ff          	add    QWORD PTR [rsi],0xffffffffffffffff
    41e1:	48 83 f8 06          	cmp    rax,0x6
    41e5:	0f 95 c1             	setne  cl
    41e8:	48 83 f8 02          	cmp    rax,0x2
    41ec:	0f 95 c2             	setne  dl
    41ef:	84 ca                	test   dl,cl
    41f1:	0f 85 a8 00 00 00    	jne    429f <some+0x26f>
    41f7:	4c 8d 05 82 fe 00 02 	lea    r8,[rip+0x200fe82]        # 2014080 <gc_sp>
    41fe:	4d 8b 08             	mov    r9,QWORD PTR [r8]
    4201:	4c 8d 15 78 fe 00 00 	lea    r10,[rip+0xfe78]        # 14080 <gc_stack>
    4208:	4d 6b d9 08          	imul   r11,r9,0x8
    420c:	4f 89 2c 1a          	mov    QWORD PTR [r10+r11*1],r13
    4210:	49 81 c1 01 00 00 00 	add    r9,0x1
    4217:	4d 89 08             	mov    QWORD PTR [r8],r9
    421a:	4c 8d 15 49 9d 00 00 	lea    r10,[rip+0x9d49]        # df6a <cljn_rest>
    4221:	4c 89 f7             	mov    rdi,r14
    4224:	41 ff d2             	call   r10
    4227:	4c 8d 15 52 fe 00 02 	lea    r10,[rip+0x200fe52]        # 2014080 <gc_sp>
    422e:	4d 8b 1a             	mov    r11,QWORD PTR [r10]
    4231:	48 8d 35 48 fe 00 00 	lea    rsi,[rip+0xfe48]        # 14080 <gc_stack>
    4238:	49 6b fb 08          	imul   rdi,r11,0x8
    423c:	48 89 04 3e          	mov    QWORD PTR [rsi+rdi*1],rax
    4240:	49 81 c3 01 00 00 00 	add    r11,0x1
    4247:	4d 89 1a             	mov    QWORD PTR [r10],r11
    424a:	41 bd 02 00 00 00    	mov    r13d,0x2
    4250:	48 8d 35 f8 35 00 00 	lea    rsi,[rip+0x35f8]        # 784f <cljn_argv>
    4257:	4c 89 ef             	mov    rdi,r13
    425a:	ff d6                	call   rsi
    425c:	bf 02 00 00 00       	mov    edi,0x2
    4261:	48 89 c2             	mov    rdx,rax
    4264:	4c 89 ee             	mov    rsi,r13
    4267:	e8 c4 fd ff ff       	call   4030 <some>
    426c:	48 8d 35 0d fe 00 02 	lea    rsi,[rip+0x200fe0d]        # 2014080 <gc_sp>
    4273:	48 83 06 fe          	add    QWORD PTR [rsi],0xfffffffffffffffe
    4277:	48 8d 3d 02 fe 00 02 	lea    rdi,[rip+0x200fe02]        # 2014080 <gc_sp>
    427e:	48 8b 0f             	mov    rcx,QWORD PTR [rdi]
    4281:	48 8d 15 f8 fd 00 00 	lea    rdx,[rip+0xfdf8]        # 14080 <gc_stack>
    4288:	4c 6b c1 08          	imul   r8,rcx,0x8
    428c:	4a 89 04 02          	mov    QWORD PTR [rdx+r8*1],rax
    4290:	48 81 c1 01 00 00 00 	add    rcx,0x1
    4297:	48 89 0f             	mov    QWORD PTR [rdi],rcx
    429a:	e9 23 00 00 00       	jmp    42c2 <some+0x292>
    429f:	48 8d 15 da fd 00 02 	lea    rdx,[rip+0x200fdda]        # 2014080 <gc_sp>
    42a6:	4c 8b 02             	mov    r8,QWORD PTR [rdx]
    42a9:	4c 8d 0d d0 fd 00 00 	lea    r9,[rip+0xfdd0]        # 14080 <gc_stack>
    42b0:	4d 6b d0 08          	imul   r10,r8,0x8
    42b4:	4b 89 04 11          	mov    QWORD PTR [r9+r10*1],rax
    42b8:	49 81 c0 01 00 00 00 	add    r8,0x1
    42bf:	4c 89 02             	mov    QWORD PTR [rdx],r8
    42c2:	49 89 c6             	mov    r14,rax
    42c5:	e9 2f 00 00 00       	jmp    42f9 <some+0x2c9>
    42ca:	b8 02 00 00 00       	mov    eax,0x2
    42cf:	49 89 c6             	mov    r14,rax
    42d2:	4c 8d 1d a7 fd 00 02 	lea    r11,[rip+0x200fda7]        # 2014080 <gc_sp>
    42d9:	49 8b 33             	mov    rsi,QWORD PTR [r11]
    42dc:	48 8d 3d 9d fd 00 00 	lea    rdi,[rip+0xfd9d]        # 14080 <gc_stack>
    42e3:	48 6b c6 08          	imul   rax,rsi,0x8
    42e7:	48 c7 04 07 02 00 00 	mov    QWORD PTR [rdi+rax*1],0x2
    42ee:	00 
    42ef:	48 81 c6 01 00 00 00 	add    rsi,0x1
    42f6:	49 89 33             	mov    QWORD PTR [r11],rsi
    42f9:	48 8d 05 4a 29 00 00 	lea    rax,[rip+0x294a]        # 6c4a <cljn_gc_leave>
    4300:	48 89 df             	mov    rdi,rbx
    4303:	ff d0                	call   rax
    4305:	4c 89 f0             	mov    rax,r14
    4308:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    430c:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    4311:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    4316:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    431b:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    4320:	48 83 c4 30          	add    rsp,0x30
    4324:	48 89 ec             	mov    rsp,rbp
    4327:	5d                   	pop    rbp
    4328:	c3                   	ret

0000000000004329 <__lambda_6>:
    4329:	55                   	push   rbp
    432a:	48 89 e5             	mov    rbp,rsp
    432d:	48 83 ec 30          	sub    rsp,0x30
    4331:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    4335:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    433a:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    433f:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    4344:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
    4349:	48 89 f3             	mov    rbx,rsi
    434c:	49 89 d5             	mov    r13,rdx
    434f:	49 89 ff             	mov    r15,rdi
    4352:	bf 01 00 00 00       	mov    edi,0x1
    4357:	4c 8d 1d 36 28 00 00 	lea    r11,[rip+0x2836]        # 6b94 <cljn_gc_enter>
    435e:	41 ff d3             	call   r11
    4361:	48 89 d9             	mov    rcx,rbx
    4364:	48 83 f9 01          	cmp    rcx,0x1
    4368:	0f 84 39 00 00 00    	je     43a7 <__lambda_6+0x7e>
    436e:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    4375:	48 8d 05 00 35 00 00 	lea    rax,[rip+0x3500]        # 787c <cljn_check_arity>
    437c:	48 89 cf             	mov    rdi,rcx
    437f:	ff d0                	call   rax
    4381:	b8 02 00 00 00       	mov    eax,0x2
    4386:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    438a:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    438f:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    4394:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    4399:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    439e:	48 83 c4 30          	add    rsp,0x30
    43a2:	48 89 ec             	mov    rsp,rbp
    43a5:	5d                   	pop    rbp
    43a6:	c3                   	ret
    43a7:	4c 89 ea             	mov    rdx,r13
    43aa:	4c 8b 22             	mov    r12,QWORD PTR [rdx]
    43ad:	48 8d 08             	lea    rcx,[rax]
    43b0:	49 89 c6             	mov    r14,rax
    43b3:	48 8d 15 c6 fc 00 00 	lea    rdx,[rip+0xfcc6]        # 14080 <gc_stack>
    43ba:	48 6b c9 08          	imul   rcx,rcx,0x8
    43be:	4c 89 24 0a          	mov    QWORD PTR [rdx+rcx*1],r12
    43c2:	48 33 f6             	xor    rsi,rsi
    43c5:	4c 8d 05 17 34 00 00 	lea    r8,[rip+0x3417]        # 77e3 <cljn_fn_free>
    43cc:	4c 89 ff             	mov    rdi,r15
    43cf:	41 ff d0             	call   r8
    43d2:	4c 8d 05 a7 fc 00 02 	lea    r8,[rip+0x200fca7]        # 2014080 <gc_sp>
    43d9:	4d 8b 08             	mov    r9,QWORD PTR [r8]
    43dc:	4c 8d 15 9d fc 00 00 	lea    r10,[rip+0xfc9d]        # 14080 <gc_stack>
    43e3:	4d 6b d9 08          	imul   r11,r9,0x8
    43e7:	4b 89 04 1a          	mov    QWORD PTR [r10+r11*1],rax
    43eb:	49 89 c5             	mov    r13,rax
    43ee:	49 81 c1 01 00 00 00 	add    r9,0x1
    43f5:	4d 89 08             	mov    QWORD PTR [r8],r9
    43f8:	be 01 00 00 00       	mov    esi,0x1
    43fd:	4c 8d 15 df 33 00 00 	lea    r10,[rip+0x33df]        # 77e3 <cljn_fn_free>
    4404:	4c 89 ff             	mov    rdi,r15
    4407:	41 ff d2             	call   r10
    440a:	49 89 c7             	mov    r15,rax
    440d:	4c 8d 15 6c fc 00 02 	lea    r10,[rip+0x200fc6c]        # 2014080 <gc_sp>
    4414:	4d 8b 1a             	mov    r11,QWORD PTR [r10]
    4417:	48 8d 35 62 fc 00 00 	lea    rsi,[rip+0xfc62]        # 14080 <gc_stack>
    441e:	49 6b fb 08          	imul   rdi,r11,0x8
    4422:	48 89 04 3e          	mov    QWORD PTR [rsi+rdi*1],rax
    4426:	49 81 c3 01 00 00 00 	add    r11,0x1
    442d:	4d 89 1a             	mov    QWORD PTR [r10],r11
    4430:	48 8d 35 49 fc 00 02 	lea    rsi,[rip+0x200fc49]        # 2014080 <gc_sp>
    4437:	48 8b 3e             	mov    rdi,QWORD PTR [rsi]
    443a:	48 8d 05 3f fc 00 00 	lea    rax,[rip+0xfc3f]        # 14080 <gc_stack>
    4441:	48 6b cf 08          	imul   rcx,rdi,0x8
    4445:	4c 89 24 08          	mov    QWORD PTR [rax+rcx*1],r12
    4449:	48 81 c7 01 00 00 00 	add    rdi,0x1
    4450:	48 89 3e             	mov    QWORD PTR [rsi],rdi
    4453:	48 8d 05 c2 33 00 00 	lea    rax,[rip+0x33c2]        # 781c <cljn_check_fn>
    445a:	4c 89 ff             	mov    rdi,r15
    445d:	ff d0                	call   rax
    445f:	bb 01 00 00 00       	mov    ebx,0x1
    4464:	48 8d 0d e4 33 00 00 	lea    rcx,[rip+0x33e4]        # 784f <cljn_argv>
    446b:	48 89 df             	mov    rdi,rbx
    446e:	ff d1                	call   rcx
    4470:	49 89 c4             	mov    r12,rax
    4473:	48 8d 0d 8c 33 00 00 	lea    rcx,[rip+0x338c]        # 7806 <cljn_fn_code>
    447a:	4c 89 ff             	mov    rdi,r15
    447d:	ff d1                	call   rcx
    447f:	4c 89 e2             	mov    rdx,r12
    4482:	48 89 de             	mov    rsi,rbx
    4485:	4c 89 ff             	mov    rdi,r15
    4488:	ff d0                	call   rax
    448a:	48 8d 0d ef fb 00 02 	lea    rcx,[rip+0x200fbef]        # 2014080 <gc_sp>
    4491:	48 83 01 fe          	add    QWORD PTR [rcx],0xfffffffffffffffe
    4495:	48 8d 15 e4 fb 00 02 	lea    rdx,[rip+0x200fbe4]        # 2014080 <gc_sp>
    449c:	4c 8b 02             	mov    r8,QWORD PTR [rdx]
    449f:	4c 8d 0d da fb 00 00 	lea    r9,[rip+0xfbda]        # 14080 <gc_stack>
    44a6:	4d 6b d0 08          	imul   r10,r8,0x8
    44aa:	4b 89 04 11          	mov    QWORD PTR [r9+r10*1],rax
    44ae:	49 81 c0 01 00 00 00 	add    r8,0x1
    44b5:	4c 89 02             	mov    QWORD PTR [rdx],r8
    44b8:	4c 8d 0d 5d 33 00 00 	lea    r9,[rip+0x335d]        # 781c <cljn_check_fn>
    44bf:	4c 89 ef             	mov    rdi,r13
    44c2:	41 ff d1             	call   r9
    44c5:	41 bf 01 00 00 00    	mov    r15d,0x1
    44cb:	4c 8d 15 7d 33 00 00 	lea    r10,[rip+0x337d]        # 784f <cljn_argv>
    44d2:	4c 89 ff             	mov    rdi,r15
    44d5:	41 ff d2             	call   r10
    44d8:	48 89 c3             	mov    rbx,rax
    44db:	4c 8d 15 24 33 00 00 	lea    r10,[rip+0x3324]        # 7806 <cljn_fn_code>
    44e2:	4c 89 ef             	mov    rdi,r13
    44e5:	41 ff d2             	call   r10
    44e8:	48 89 da             	mov    rdx,rbx
    44eb:	4c 89 fe             	mov    rsi,r15
    44ee:	4c 89 ef             	mov    rdi,r13
    44f1:	ff d0                	call   rax
    44f3:	4c 8d 15 86 fb 00 02 	lea    r10,[rip+0x200fb86]        # 2014080 <gc_sp>
    44fa:	49 83 02 fe          	add    QWORD PTR [r10],0xfffffffffffffffe
    44fe:	4c 8d 1d 7b fb 00 02 	lea    r11,[rip+0x200fb7b]        # 2014080 <gc_sp>
    4505:	49 8b 33             	mov    rsi,QWORD PTR [r11]
    4508:	48 8d 3d 71 fb 00 00 	lea    rdi,[rip+0xfb71]        # 14080 <gc_stack>
    450f:	48 6b ce 08          	imul   rcx,rsi,0x8
    4513:	48 89 04 0f          	mov    QWORD PTR [rdi+rcx*1],rax
    4517:	49 89 c4             	mov    r12,rax
    451a:	48 81 c6 01 00 00 00 	add    rsi,0x1
    4521:	49 89 33             	mov    QWORD PTR [r11],rsi
    4524:	48 8d 05 1f 27 00 00 	lea    rax,[rip+0x271f]        # 6c4a <cljn_gc_leave>
    452b:	4c 89 f7             	mov    rdi,r14
    452e:	ff d0                	call   rax
    4530:	4c 89 e0             	mov    rax,r12
    4533:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    4537:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    453c:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    4541:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    4546:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    454b:	48 83 c4 30          	add    rsp,0x30
    454f:	48 89 ec             	mov    rsp,rbp
    4552:	5d                   	pop    rbp
    4553:	c3                   	ret

0000000000004554 <comp>:
    4554:	55                   	push   rbp
    4555:	48 89 e5             	mov    rbp,rsp
    4558:	48 83 ec 30          	sub    rsp,0x30
    455c:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    4560:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    4565:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    456a:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    456f:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
    4574:	49 89 f4             	mov    r12,rsi
    4577:	49 89 d6             	mov    r14,rdx
    457a:	bf 02 00 00 00       	mov    edi,0x2
    457f:	4c 8d 15 0e 26 00 00 	lea    r10,[rip+0x260e]        # 6b94 <cljn_gc_enter>
    4586:	41 ff d2             	call   r10
    4589:	4d 89 e0             	mov    r8,r12
    458c:	49 83 f8 02          	cmp    r8,0x2
    4590:	0f 84 39 00 00 00    	je     45cf <comp+0x7b>
    4596:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    459d:	48 8d 05 d8 32 00 00 	lea    rax,[rip+0x32d8]        # 787c <cljn_check_arity>
    45a4:	4c 89 c7             	mov    rdi,r8
    45a7:	ff d0                	call   rax
    45a9:	b8 02 00 00 00       	mov    eax,0x2
    45ae:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    45b2:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    45b7:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    45bc:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    45c1:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    45c6:	48 83 c4 30          	add    rsp,0x30
    45ca:	48 89 ec             	mov    rsp,rbp
    45cd:	5d                   	pop    rbp
    45ce:	c3                   	ret
    45cf:	4c 89 f2             	mov    rdx,r14
    45d2:	4c 8b 2a             	mov    r13,QWORD PTR [rdx]
    45d5:	48 8d 08             	lea    rcx,[rax]
    45d8:	4c 8d 05 a1 fa 00 00 	lea    r8,[rip+0xfaa1]        # 14080 <gc_stack>
    45df:	48 6b c9 08          	imul   rcx,rcx,0x8
    45e3:	4d 89 2c 08          	mov    QWORD PTR [r8+rcx*1],r13
    45e7:	4c 8b 72 08          	mov    r14,QWORD PTR [rdx+0x8]
    45eb:	48 8d 48 01          	lea    rcx,[rax+0x1]
    45ef:	49 89 c7             	mov    r15,rax
    45f2:	48 8d 15 87 fa 00 00 	lea    rdx,[rip+0xfa87]        # 14080 <gc_stack>
    45f9:	48 6b c9 08          	imul   rcx,rcx,0x8
    45fd:	4c 89 34 0a          	mov    QWORD PTR [rdx+rcx*1],r14
    4601:	4c 8d 05 78 fa 00 02 	lea    r8,[rip+0x200fa78]        # 2014080 <gc_sp>
    4608:	4d 8b 08             	mov    r9,QWORD PTR [r8]
    460b:	4c 8d 15 6e fa 00 00 	lea    r10,[rip+0xfa6e]        # 14080 <gc_stack>
    4612:	4d 6b d9 08          	imul   r11,r9,0x8
    4616:	4f 89 2c 1a          	mov    QWORD PTR [r10+r11*1],r13
    461a:	49 81 c1 01 00 00 00 	add    r9,0x1
    4621:	4d 89 08             	mov    QWORD PTR [r8],r9
    4624:	4c 8d 15 55 fa 00 02 	lea    r10,[rip+0x200fa55]        # 2014080 <gc_sp>
    462b:	4d 8b 1a             	mov    r11,QWORD PTR [r10]
    462e:	48 8d 35 4b fa 00 00 	lea    rsi,[rip+0xfa4b]        # 14080 <gc_stack>
    4635:	49 6b fb 08          	imul   rdi,r11,0x8
    4639:	4c 89 34 3e          	mov    QWORD PTR [rsi+rdi*1],r14
    463d:	49 81 c3 01 00 00 00 	add    r11,0x1
    4644:	4d 89 1a             	mov    QWORD PTR [r10],r11
    4647:	48 8d 3d db fc ff ff 	lea    rdi,[rip+0xfffffffffffffcdb]        # 4329 <__lambda_6>
    464e:	be 01 00 00 00       	mov    esi,0x1
    4653:	ba 02 00 00 00       	mov    edx,0x2
    4658:	48 8d 05 c7 30 00 00 	lea    rax,[rip+0x30c7]        # 7726 <cljn_make_fn>
    465f:	ff d0                	call   rax
    4661:	48 89 c3             	mov    rbx,rax
    4664:	48 8d 35 15 fa 00 02 	lea    rsi,[rip+0x200fa15]        # 2014080 <gc_sp>
    466b:	48 83 06 fe          	add    QWORD PTR [rsi],0xfffffffffffffffe
    466f:	48 8d 3d 0a fa 00 02 	lea    rdi,[rip+0x200fa0a]        # 2014080 <gc_sp>
    4676:	48 8b 07             	mov    rax,QWORD PTR [rdi]
    4679:	48 8d 0d 00 fa 00 00 	lea    rcx,[rip+0xfa00]        # 14080 <gc_stack>
    4680:	48 6b d0 08          	imul   rdx,rax,0x8
    4684:	49 89 db             	mov    r11,rbx
    4687:	4c 89 1c 11          	mov    QWORD PTR [rcx+rdx*1],r11
    468b:	48 81 c0 01 00 00 00 	add    rax,0x1
    4692:	48 89 07             	mov    QWORD PTR [rdi],rax
    4695:	48 33 f6             	xor    rsi,rsi
    4698:	48 8d 0d 18 31 00 00 	lea    rcx,[rip+0x3118]        # 77b7 <cljn_fn_set_free>
    469f:	4c 89 ea             	mov    rdx,r13
    46a2:	48 89 df             	mov    rdi,rbx
    46a5:	ff d1                	call   rcx
    46a7:	be 01 00 00 00       	mov    esi,0x1
    46ac:	4c 8d 05 04 31 00 00 	lea    r8,[rip+0x3104]        # 77b7 <cljn_fn_set_free>
    46b3:	4c 89 f2             	mov    rdx,r14
    46b6:	48 89 df             	mov    rdi,rbx
    46b9:	41 ff d0             	call   r8
    46bc:	4c 8d 05 87 25 00 00 	lea    r8,[rip+0x2587]        # 6c4a <cljn_gc_leave>
    46c3:	4c 89 ff             	mov    rdi,r15
    46c6:	41 ff d0             	call   r8
    46c9:	48 89 d8             	mov    rax,rbx
    46cc:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    46d0:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    46d5:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    46da:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    46df:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    46e4:	48 83 c4 30          	add    rsp,0x30
    46e8:	48 89 ec             	mov    rsp,rbp
    46eb:	5d                   	pop    rbp
    46ec:	c3                   	ret

00000000000046ed <identity>:
    46ed:	55                   	push   rbp
    46ee:	48 89 e5             	mov    rbp,rsp
    46f1:	48 83 ec 20          	sub    rsp,0x20
    46f5:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    46f9:	4c 89 6c 24 08       	mov    QWORD PTR [rsp+0x8],r13
    46fe:	4c 89 74 24 10       	mov    QWORD PTR [rsp+0x10],r14
    4703:	48 89 d3             	mov    rbx,rdx
    4706:	49 89 f6             	mov    r14,rsi
    4709:	bf 01 00 00 00       	mov    edi,0x1
    470e:	48 8d 15 7f 24 00 00 	lea    rdx,[rip+0x247f]        # 6b94 <cljn_gc_enter>
    4715:	ff d2                	call   rdx
    4717:	4c 89 f1             	mov    rcx,r14
    471a:	48 83 f9 01          	cmp    rcx,0x1
    471e:	0f 84 30 00 00 00    	je     4754 <identity+0x67>
    4724:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    472b:	4c 8d 0d 4a 31 00 00 	lea    r9,[rip+0x314a]        # 787c <cljn_check_arity>
    4732:	48 89 cf             	mov    rdi,rcx
    4735:	41 ff d1             	call   r9
    4738:	b8 02 00 00 00       	mov    eax,0x2
    473d:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    4741:	4c 8b 6c 24 08       	mov    r13,QWORD PTR [rsp+0x8]
    4746:	4c 8b 74 24 10       	mov    r14,QWORD PTR [rsp+0x10]
    474b:	48 83 c4 20          	add    rsp,0x20
    474f:	48 89 ec             	mov    rsp,rbp
    4752:	5d                   	pop    rbp
    4753:	c3                   	ret
    4754:	48 89 da             	mov    rdx,rbx
    4757:	4c 8b 2a             	mov    r13,QWORD PTR [rdx]
    475a:	4c 8d 18             	lea    r11,[rax]
    475d:	48 89 c7             	mov    rdi,rax
    4760:	48 8d 35 19 f9 00 00 	lea    rsi,[rip+0xf919]        # 14080 <gc_stack>
    4767:	4d 6b db 08          	imul   r11,r11,0x8
    476b:	4e 89 2c 1e          	mov    QWORD PTR [rsi+r11*1],r13
    476f:	48 8d 05 d4 24 00 00 	lea    rax,[rip+0x24d4]        # 6c4a <cljn_gc_leave>
    4776:	ff d0                	call   rax
    4778:	4c 89 e8             	mov    rax,r13
    477b:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    477f:	4c 8b 6c 24 08       	mov    r13,QWORD PTR [rsp+0x8]
    4784:	4c 8b 74 24 10       	mov    r14,QWORD PTR [rsp+0x10]
    4789:	48 83 c4 20          	add    rsp,0x20
    478d:	48 89 ec             	mov    rsp,rbp
    4790:	5d                   	pop    rbp
    4791:	c3                   	ret

0000000000004792 <second>:
    4792:	55                   	push   rbp
    4793:	48 89 e5             	mov    rbp,rsp
    4796:	48 83 ec 20          	sub    rsp,0x20
    479a:	4c 89 24 24          	mov    QWORD PTR [rsp],r12
    479e:	4c 89 6c 24 08       	mov    QWORD PTR [rsp+0x8],r13
    47a3:	4c 89 74 24 10       	mov    QWORD PTR [rsp+0x10],r14
    47a8:	4c 89 7c 24 18       	mov    QWORD PTR [rsp+0x18],r15
    47ad:	49 89 f4             	mov    r12,rsi
    47b0:	49 89 d6             	mov    r14,rdx
    47b3:	bf 01 00 00 00       	mov    edi,0x1
    47b8:	48 8d 05 d5 23 00 00 	lea    rax,[rip+0x23d5]        # 6b94 <cljn_gc_enter>
    47bf:	ff d0                	call   rax
    47c1:	4c 89 e1             	mov    rcx,r12
    47c4:	48 83 f9 01          	cmp    rcx,0x1
    47c8:	0f 84 34 00 00 00    	je     4802 <second+0x70>
    47ce:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    47d5:	48 8d 15 a0 30 00 00 	lea    rdx,[rip+0x30a0]        # 787c <cljn_check_arity>
    47dc:	48 89 cf             	mov    rdi,rcx
    47df:	ff d2                	call   rdx
    47e1:	b8 02 00 00 00       	mov    eax,0x2
    47e6:	4c 8b 24 24          	mov    r12,QWORD PTR [rsp]
    47ea:	4c 8b 6c 24 08       	mov    r13,QWORD PTR [rsp+0x8]
    47ef:	4c 8b 74 24 10       	mov    r14,QWORD PTR [rsp+0x10]
    47f4:	4c 8b 7c 24 18       	mov    r15,QWORD PTR [rsp+0x18]
    47f9:	48 83 c4 20          	add    rsp,0x20
    47fd:	48 89 ec             	mov    rsp,rbp
    4800:	5d                   	pop    rbp
    4801:	c3                   	ret
    4802:	4c 89 f2             	mov    rdx,r14
    4805:	48 8b 3a             	mov    rdi,QWORD PTR [rdx]
    4808:	4c 8d 00             	lea    r8,[rax]
    480b:	49 89 c7             	mov    r15,rax
    480e:	4c 8d 0d 6b f8 00 00 	lea    r9,[rip+0xf86b]        # 14080 <gc_stack>
    4815:	4d 6b c0 08          	imul   r8,r8,0x8
    4819:	4b 89 3c 01          	mov    QWORD PTR [r9+r8*1],rdi
    481d:	4c 8d 15 46 97 00 00 	lea    r10,[rip+0x9746]        # df6a <cljn_rest>
    4824:	41 ff d2             	call   r10
    4827:	4c 8d 15 52 f8 00 02 	lea    r10,[rip+0x200f852]        # 2014080 <gc_sp>
    482e:	4d 8b 1a             	mov    r11,QWORD PTR [r10]
    4831:	48 8d 35 48 f8 00 00 	lea    rsi,[rip+0xf848]        # 14080 <gc_stack>
    4838:	49 6b fb 08          	imul   rdi,r11,0x8
    483c:	48 89 04 3e          	mov    QWORD PTR [rsi+rdi*1],rax
    4840:	49 81 c3 01 00 00 00 	add    r11,0x1
    4847:	4d 89 1a             	mov    QWORD PTR [r10],r11
    484a:	48 8d 35 ca 95 00 00 	lea    rsi,[rip+0x95ca]        # de1b <cljn_first>
    4851:	48 89 c7             	mov    rdi,rax
    4854:	ff d6                	call   rsi
    4856:	48 8d 35 23 f8 00 02 	lea    rsi,[rip+0x200f823]        # 2014080 <gc_sp>
    485d:	48 83 06 ff          	add    QWORD PTR [rsi],0xffffffffffffffff
    4861:	48 8d 3d 18 f8 00 02 	lea    rdi,[rip+0x200f818]        # 2014080 <gc_sp>
    4868:	48 8b 0f             	mov    rcx,QWORD PTR [rdi]
    486b:	48 8d 15 0e f8 00 00 	lea    rdx,[rip+0xf80e]        # 14080 <gc_stack>
    4872:	4c 6b c1 08          	imul   r8,rcx,0x8
    4876:	4a 89 04 02          	mov    QWORD PTR [rdx+r8*1],rax
    487a:	49 89 c5             	mov    r13,rax
    487d:	48 8d 41 01          	lea    rax,[rcx+0x1]
    4881:	48 89 07             	mov    QWORD PTR [rdi],rax
    4884:	48 8d 0d bf 23 00 00 	lea    rcx,[rip+0x23bf]        # 6c4a <cljn_gc_leave>
    488b:	4c 89 ff             	mov    rdi,r15
    488e:	ff d1                	call   rcx
    4890:	4c 89 e8             	mov    rax,r13
    4893:	4c 8b 24 24          	mov    r12,QWORD PTR [rsp]
    4897:	4c 8b 6c 24 08       	mov    r13,QWORD PTR [rsp+0x8]
    489c:	4c 8b 74 24 10       	mov    r14,QWORD PTR [rsp+0x10]
    48a1:	4c 8b 7c 24 18       	mov    r15,QWORD PTR [rsp+0x18]
    48a6:	48 83 c4 20          	add    rsp,0x20
    48aa:	48 89 ec             	mov    rsp,rbp
    48ad:	5d                   	pop    rbp
    48ae:	c3                   	ret

00000000000048af <last>:
    48af:	55                   	push   rbp
    48b0:	48 89 e5             	mov    rbp,rsp
    48b3:	48 83 ec 20          	sub    rsp,0x20
    48b7:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    48bb:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    48c0:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    48c5:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    48ca:	49 89 f4             	mov    r12,rsi
    48cd:	49 89 d6             	mov    r14,rdx
    48d0:	bf 01 00 00 00       	mov    edi,0x1
    48d5:	4c 8d 1d b8 22 00 00 	lea    r11,[rip+0x22b8]        # 6b94 <cljn_gc_enter>
    48dc:	41 ff d3             	call   r11
    48df:	4d 89 e0             	mov    r8,r12
    48e2:	49 83 f8 01          	cmp    r8,0x1
    48e6:	0f 84 34 00 00 00    	je     4920 <last+0x71>
    48ec:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    48f3:	48 8d 05 82 2f 00 00 	lea    rax,[rip+0x2f82]        # 787c <cljn_check_arity>
    48fa:	4c 89 c7             	mov    rdi,r8
    48fd:	ff d0                	call   rax
    48ff:	b8 02 00 00 00       	mov    eax,0x2
    4904:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    4908:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    490d:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    4912:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    4917:	48 83 c4 20          	add    rsp,0x20
    491b:	48 89 ec             	mov    rsp,rbp
    491e:	5d                   	pop    rbp
    491f:	c3                   	ret
    4920:	4c 89 f2             	mov    rdx,r14
    4923:	48 8b 3a             	mov    rdi,QWORD PTR [rdx]
    4926:	48 8d 08             	lea    rcx,[rax]
    4929:	48 89 c3             	mov    rbx,rax
    492c:	48 8d 15 4d f7 00 00 	lea    rdx,[rip+0xf74d]        # 14080 <gc_stack>
    4933:	48 6b c9 08          	imul   rcx,rcx,0x8
    4937:	48 89 3c 0a          	mov    QWORD PTR [rdx+rcx*1],rdi
    493b:	49 89 fd             	mov    r13,rdi
    493e:	4c 8d 05 25 96 00 00 	lea    r8,[rip+0x9625]        # df6a <cljn_rest>
    4945:	4c 89 ef             	mov    rdi,r13
    4948:	41 ff d0             	call   r8
    494b:	4c 8d 05 2e f7 00 02 	lea    r8,[rip+0x200f72e]        # 2014080 <gc_sp>
    4952:	4d 8b 08             	mov    r9,QWORD PTR [r8]
    4955:	4c 8d 15 24 f7 00 00 	lea    r10,[rip+0xf724]        # 14080 <gc_stack>
    495c:	4d 6b d9 08          	imul   r11,r9,0x8
    4960:	4b 89 04 1a          	mov    QWORD PTR [r10+r11*1],rax
    4964:	49 81 c1 01 00 00 00 	add    r9,0x1
    496b:	4d 89 08             	mov    QWORD PTR [r8],r9
    496e:	4c 8d 15 83 93 00 00 	lea    r10,[rip+0x9383]        # dcf8 <cljn_emptyp>
    4975:	48 89 c7             	mov    rdi,rax
    4978:	41 ff d2             	call   r10
    497b:	4c 8d 15 fe f6 00 02 	lea    r10,[rip+0x200f6fe]        # 2014080 <gc_sp>
    4982:	49 83 02 ff          	add    QWORD PTR [r10],0xffffffffffffffff
    4986:	48 83 f8 06          	cmp    rax,0x6
    498a:	40 0f 95 c6          	setne  sil
    498e:	48 83 f8 02          	cmp    rax,0x2
    4992:	40 0f 95 c7          	setne  dil
    4996:	40 84 f7             	test   dil,sil
    4999:	0f 85 57 00 00 00    	jne    49f6 <last+0x147>
    499f:	48 8d 0d c4 95 00 00 	lea    rcx,[rip+0x95c4]        # df6a <cljn_rest>
    49a6:	4c 89 ef             	mov    rdi,r13
    49a9:	ff d1                	call   rcx
    49ab:	48 8d 0d ce f6 00 02 	lea    rcx,[rip+0x200f6ce]        # 2014080 <gc_sp>
    49b2:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    49b5:	4c 8d 05 c4 f6 00 00 	lea    r8,[rip+0xf6c4]        # 14080 <gc_stack>
    49bc:	4c 6b ca 08          	imul   r9,rdx,0x8
    49c0:	4b 89 04 08          	mov    QWORD PTR [r8+r9*1],rax
    49c4:	48 81 c2 01 00 00 00 	add    rdx,0x1
    49cb:	48 89 11             	mov    QWORD PTR [rcx],rdx
    49ce:	48 89 da             	mov    rdx,rbx
    49d1:	4c 8d 02             	lea    r8,[rdx]
    49d4:	4c 8d 0d a5 f6 00 00 	lea    r9,[rip+0xf6a5]        # 14080 <gc_stack>
    49db:	4d 6b c0 08          	imul   r8,r8,0x8
    49df:	4b 89 04 01          	mov    QWORD PTR [r9+r8*1],rax
    49e3:	4c 8d 0d 96 f6 00 02 	lea    r9,[rip+0x200f696]        # 2014080 <gc_sp>
    49ea:	49 83 01 ff          	add    QWORD PTR [r9],0xffffffffffffffff
    49ee:	49 89 c5             	mov    r13,rax
    49f1:	e9 48 ff ff ff       	jmp    493e <last+0x8f>
    49f6:	4c 89 ef             	mov    rdi,r13
    49f9:	4c 8d 1d 1b 94 00 00 	lea    r11,[rip+0x941b]        # de1b <cljn_first>
    4a00:	41 ff d3             	call   r11
    4a03:	4c 8d 1d 76 f6 00 02 	lea    r11,[rip+0x200f676]        # 2014080 <gc_sp>
    4a0a:	49 8b 33             	mov    rsi,QWORD PTR [r11]
    4a0d:	48 8d 3d 6c f6 00 00 	lea    rdi,[rip+0xf66c]        # 14080 <gc_stack>
    4a14:	48 6b ce 08          	imul   rcx,rsi,0x8
    4a18:	48 89 04 0f          	mov    QWORD PTR [rdi+rcx*1],rax
    4a1c:	49 89 c5             	mov    r13,rax
    4a1f:	48 81 c6 01 00 00 00 	add    rsi,0x1
    4a26:	49 89 33             	mov    QWORD PTR [r11],rsi
    4a29:	48 8d 05 1a 22 00 00 	lea    rax,[rip+0x221a]        # 6c4a <cljn_gc_leave>
    4a30:	48 89 df             	mov    rdi,rbx
    4a33:	ff d0                	call   rax
    4a35:	4c 89 e8             	mov    rax,r13
    4a38:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    4a3c:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    4a41:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    4a46:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    4a4b:	48 83 c4 20          	add    rsp,0x20
    4a4f:	48 89 ec             	mov    rsp,rbp
    4a52:	5d                   	pop    rbp
    4a53:	c3                   	ret

0000000000004a54 <__lambda_7>:
    4a54:	55                   	push   rbp
    4a55:	48 89 e5             	mov    rbp,rsp
    4a58:	48 83 ec 20          	sub    rsp,0x20
    4a5c:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    4a60:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    4a65:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    4a6a:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    4a6f:	49 89 d5             	mov    r13,rdx
    4a72:	49 89 f6             	mov    r14,rsi
    4a75:	bf 02 00 00 00       	mov    edi,0x2
    4a7a:	4c 8d 0d 13 21 00 00 	lea    r9,[rip+0x2113]        # 6b94 <cljn_gc_enter>
    4a81:	41 ff d1             	call   r9
    4a84:	4c 89 f7             	mov    rdi,r14
    4a87:	48 83 ff 02          	cmp    rdi,0x2
    4a8b:	0f 84 32 00 00 00    	je     4ac3 <__lambda_7+0x6f>
    4a91:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    4a98:	4c 8d 1d dd 2d 00 00 	lea    r11,[rip+0x2ddd]        # 787c <cljn_check_arity>
    4a9f:	41 ff d3             	call   r11
    4aa2:	b8 02 00 00 00       	mov    eax,0x2
    4aa7:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    4aab:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    4ab0:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    4ab5:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    4aba:	48 83 c4 20          	add    rsp,0x20
    4abe:	48 89 ec             	mov    rsp,rbp
    4ac1:	5d                   	pop    rbp
    4ac2:	c3                   	ret
    4ac3:	4c 89 ea             	mov    rdx,r13
    4ac6:	48 8b 32             	mov    rsi,QWORD PTR [rdx]
    4ac9:	48 8d 38             	lea    rdi,[rax]
    4acc:	48 8d 0d ad f5 00 00 	lea    rcx,[rip+0xf5ad]        # 14080 <gc_stack>
    4ad3:	48 6b ff 08          	imul   rdi,rdi,0x8
    4ad7:	48 89 34 39          	mov    QWORD PTR [rcx+rdi*1],rsi
    4adb:	48 8b 7a 08          	mov    rdi,QWORD PTR [rdx+0x8]
    4adf:	48 8d 48 01          	lea    rcx,[rax+0x1]
    4ae3:	48 89 c3             	mov    rbx,rax
    4ae6:	48 8d 05 93 f5 00 00 	lea    rax,[rip+0xf593]        # 14080 <gc_stack>
    4aed:	48 6b c9 08          	imul   rcx,rcx,0x8
    4af1:	48 89 3c 08          	mov    QWORD PTR [rax+rcx*1],rdi
    4af5:	48 8d 15 e5 2b 00 00 	lea    rdx,[rip+0x2be5]        # 76e1 <cljn_cons>
    4afc:	ff d2                	call   rdx
    4afe:	48 8d 15 7b f5 00 02 	lea    rdx,[rip+0x200f57b]        # 2014080 <gc_sp>
    4b05:	4c 8b 02             	mov    r8,QWORD PTR [rdx]
    4b08:	4c 8d 0d 71 f5 00 00 	lea    r9,[rip+0xf571]        # 14080 <gc_stack>
    4b0f:	4d 6b d0 08          	imul   r10,r8,0x8
    4b13:	4b 89 04 11          	mov    QWORD PTR [r9+r10*1],rax
    4b17:	49 89 c4             	mov    r12,rax
    4b1a:	49 81 c0 01 00 00 00 	add    r8,0x1
    4b21:	4c 89 02             	mov    QWORD PTR [rdx],r8
    4b24:	4c 8d 0d 1f 21 00 00 	lea    r9,[rip+0x211f]        # 6c4a <cljn_gc_leave>
    4b2b:	48 89 df             	mov    rdi,rbx
    4b2e:	41 ff d1             	call   r9
    4b31:	4c 89 e0             	mov    rax,r12
    4b34:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    4b38:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    4b3d:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    4b42:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    4b47:	48 83 c4 20          	add    rsp,0x20
    4b4b:	48 89 ec             	mov    rsp,rbp
    4b4e:	5d                   	pop    rbp
    4b4f:	c3                   	ret

0000000000004b50 <concat>:
    4b50:	55                   	push   rbp
    4b51:	48 89 e5             	mov    rbp,rsp
    4b54:	48 83 ec 30          	sub    rsp,0x30
    4b58:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    4b5c:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    4b61:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    4b66:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    4b6b:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
    4b70:	49 89 d4             	mov    r12,rdx
    4b73:	49 89 f7             	mov    r15,rsi
    4b76:	bf 02 00 00 00       	mov    edi,0x2
    4b7b:	4c 8d 0d 12 20 00 00 	lea    r9,[rip+0x2012]        # 6b94 <cljn_gc_enter>
    4b82:	41 ff d1             	call   r9
    4b85:	4c 89 ff             	mov    rdi,r15
    4b88:	48 83 ff 02          	cmp    rdi,0x2
    4b8c:	0f 84 37 00 00 00    	je     4bc9 <concat+0x79>
    4b92:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    4b99:	4c 8d 1d dc 2c 00 00 	lea    r11,[rip+0x2cdc]        # 787c <cljn_check_arity>
    4ba0:	41 ff d3             	call   r11
    4ba3:	b8 02 00 00 00       	mov    eax,0x2
    4ba8:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    4bac:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    4bb1:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    4bb6:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    4bbb:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    4bc0:	48 83 c4 30          	add    rsp,0x30
    4bc4:	48 89 ec             	mov    rsp,rbp
    4bc7:	5d                   	pop    rbp
    4bc8:	c3                   	ret
    4bc9:	4c 89 e2             	mov    rdx,r12
    4bcc:	4c 8b 3a             	mov    r15,QWORD PTR [rdx]
    4bcf:	48 8d 38             	lea    rdi,[rax]
    4bd2:	48 8d 0d a7 f4 00 00 	lea    rcx,[rip+0xf4a7]        # 14080 <gc_stack>
    4bd9:	48 6b ff 08          	imul   rdi,rdi,0x8
    4bdd:	4c 89 3c 39          	mov    QWORD PTR [rcx+rdi*1],r15
    4be1:	48 8b 5a 08          	mov    rbx,QWORD PTR [rdx+0x8]
    4be5:	48 8d 48 01          	lea    rcx,[rax+0x1]
    4be9:	49 89 c5             	mov    r13,rax
    4bec:	48 8d 05 8d f4 00 00 	lea    rax,[rip+0xf48d]        # 14080 <gc_stack>
    4bf3:	48 6b c9 08          	imul   rcx,rcx,0x8
    4bf7:	48 89 1c 08          	mov    QWORD PTR [rax+rcx*1],rbx
    4bfb:	48 8d 3d 52 fe ff ff 	lea    rdi,[rip+0xfffffffffffffe52]        # 4a54 <__lambda_7>
    4c02:	be 02 00 00 00       	mov    esi,0x2
    4c07:	48 33 d2             	xor    rdx,rdx
    4c0a:	4c 8d 05 15 2b 00 00 	lea    r8,[rip+0x2b15]        # 7726 <cljn_make_fn>
    4c11:	41 ff d0             	call   r8
    4c14:	48 8d 15 65 f4 00 02 	lea    rdx,[rip+0x200f465]        # 2014080 <gc_sp>
    4c1b:	4c 8b 02             	mov    r8,QWORD PTR [rdx]
    4c1e:	4c 8d 0d 5b f4 00 00 	lea    r9,[rip+0xf45b]        # 14080 <gc_stack>
    4c25:	4d 6b d0 08          	imul   r10,r8,0x8
    4c29:	4b 89 04 11          	mov    QWORD PTR [r9+r10*1],rax
    4c2d:	49 81 c0 01 00 00 00 	add    r8,0x1
    4c34:	4c 89 02             	mov    QWORD PTR [rdx],r8
    4c37:	4c 8d 0d 42 f4 00 02 	lea    r9,[rip+0x200f442]        # 2014080 <gc_sp>
    4c3e:	4d 8b 11             	mov    r10,QWORD PTR [r9]
    4c41:	4c 8d 1d 38 f4 00 00 	lea    r11,[rip+0xf438]        # 14080 <gc_stack>
    4c48:	49 6b f2 08          	imul   rsi,r10,0x8
    4c4c:	49 89 1c 33          	mov    QWORD PTR [r11+rsi*1],rbx
    4c50:	49 81 c2 01 00 00 00 	add    r10,0x1
    4c57:	4d 89 11             	mov    QWORD PTR [r9],r10
    4c5a:	4c 8d 1d 1f f4 00 02 	lea    r11,[rip+0x200f41f]        # 2014080 <gc_sp>
    4c61:	49 8b 33             	mov    rsi,QWORD PTR [r11]
    4c64:	48 8d 3d 15 f4 00 00 	lea    rdi,[rip+0xf415]        # 14080 <gc_stack>
    4c6b:	48 6b c6 08          	imul   rax,rsi,0x8
    4c6f:	4c 89 3c 07          	mov    QWORD PTR [rdi+rax*1],r15
    4c73:	48 81 c6 01 00 00 00 	add    rsi,0x1
    4c7a:	49 89 33             	mov    QWORD PTR [r11],rsi
    4c7d:	41 be 01 00 00 00    	mov    r14d,0x1
    4c83:	48 8d 05 c5 2b 00 00 	lea    rax,[rip+0x2bc5]        # 784f <cljn_argv>
    4c8a:	4c 89 f7             	mov    rdi,r14
    4c8d:	ff d0                	call   rax
    4c8f:	bf 02 00 00 00       	mov    edi,0x2
    4c94:	48 89 c2             	mov    rdx,rax
    4c97:	4c 89 f6             	mov    rsi,r14
    4c9a:	e8 5d df ff ff       	call   2bfc <reverse>
    4c9f:	48 8d 3d da f3 00 02 	lea    rdi,[rip+0x200f3da]        # 2014080 <gc_sp>
    4ca6:	48 83 07 ff          	add    QWORD PTR [rdi],0xffffffffffffffff
    4caa:	48 8d 0d cf f3 00 02 	lea    rcx,[rip+0x200f3cf]        # 2014080 <gc_sp>
    4cb1:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    4cb4:	4c 8d 05 c5 f3 00 00 	lea    r8,[rip+0xf3c5]        # 14080 <gc_stack>
    4cbb:	4c 6b ca 08          	imul   r9,rdx,0x8
    4cbf:	4b 89 04 08          	mov    QWORD PTR [r8+r9*1],rax
    4cc3:	48 81 c2 01 00 00 00 	add    rdx,0x1
    4cca:	48 89 11             	mov    QWORD PTR [rcx],rdx
    4ccd:	41 bc 03 00 00 00    	mov    r12d,0x3
    4cd3:	48 8d 15 75 2b 00 00 	lea    rdx,[rip+0x2b75]        # 784f <cljn_argv>
    4cda:	4c 89 e7             	mov    rdi,r12
    4cdd:	ff d2                	call   rdx
    4cdf:	bf 02 00 00 00       	mov    edi,0x2
    4ce4:	48 89 c2             	mov    rdx,rax
    4ce7:	4c 89 e6             	mov    rsi,r12
    4cea:	e8 ed d0 ff ff       	call   1ddc <reduce>
    4cef:	48 8d 15 8a f3 00 02 	lea    rdx,[rip+0x200f38a]        # 2014080 <gc_sp>
    4cf6:	48 83 02 fd          	add    QWORD PTR [rdx],0xfffffffffffffffd
    4cfa:	4c 8d 05 7f f3 00 02 	lea    r8,[rip+0x200f37f]        # 2014080 <gc_sp>
    4d01:	4d 8b 08             	mov    r9,QWORD PTR [r8]
    4d04:	4c 8d 15 75 f3 00 00 	lea    r10,[rip+0xf375]        # 14080 <gc_stack>
    4d0b:	4d 6b d9 08          	imul   r11,r9,0x8
    4d0f:	4b 89 04 1a          	mov    QWORD PTR [r10+r11*1],rax
    4d13:	49 89 c7             	mov    r15,rax
    4d16:	49 81 c1 01 00 00 00 	add    r9,0x1
    4d1d:	4d 89 08             	mov    QWORD PTR [r8],r9
    4d20:	4c 8d 15 23 1f 00 00 	lea    r10,[rip+0x1f23]        # 6c4a <cljn_gc_leave>
    4d27:	4c 89 ef             	mov    rdi,r13
    4d2a:	41 ff d2             	call   r10
    4d2d:	4c 89 f8             	mov    rax,r15
    4d30:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    4d34:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    4d39:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    4d3e:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    4d43:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    4d48:	48 83 c4 30          	add    rsp,0x30
    4d4c:	48 89 ec             	mov    rsp,rbp
    4d4f:	5d                   	pop    rbp
    4d50:	c3                   	ret

0000000000004d51 <__lambda_8>:
    4d51:	55                   	push   rbp
    4d52:	48 89 e5             	mov    rbp,rsp
    4d55:	48 83 ec 30          	sub    rsp,0x30
    4d59:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    4d5d:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    4d62:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    4d67:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    4d6c:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
    4d71:	48 89 d3             	mov    rbx,rdx
    4d74:	49 89 f5             	mov    r13,rsi
    4d77:	49 89 ff             	mov    r15,rdi
    4d7a:	bf 02 00 00 00       	mov    edi,0x2
    4d7f:	4c 8d 15 0e 1e 00 00 	lea    r10,[rip+0x1e0e]        # 6b94 <cljn_gc_enter>
    4d86:	41 ff d2             	call   r10
    4d89:	4d 89 eb             	mov    r11,r13
    4d8c:	49 83 fb 02          	cmp    r11,0x2
    4d90:	0f 84 39 00 00 00    	je     4dcf <__lambda_8+0x7e>
    4d96:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    4d9d:	48 8d 05 d8 2a 00 00 	lea    rax,[rip+0x2ad8]        # 787c <cljn_check_arity>
    4da4:	4c 89 df             	mov    rdi,r11
    4da7:	ff d0                	call   rax
    4da9:	b8 02 00 00 00       	mov    eax,0x2
    4dae:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    4db2:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    4db7:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    4dbc:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    4dc1:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    4dc6:	48 83 c4 30          	add    rsp,0x30
    4dca:	48 89 ec             	mov    rsp,rbp
    4dcd:	5d                   	pop    rbp
    4dce:	c3                   	ret
    4dcf:	48 89 da             	mov    rdx,rbx
    4dd2:	48 8b 0a             	mov    rcx,QWORD PTR [rdx]
    4dd5:	4c 8d 00             	lea    r8,[rax]
    4dd8:	4c 8d 0d a1 f2 00 00 	lea    r9,[rip+0xf2a1]        # 14080 <gc_stack>
    4ddf:	4d 6b c0 08          	imul   r8,r8,0x8
    4de3:	4b 89 0c 01          	mov    QWORD PTR [r9+r8*1],rcx
    4de7:	4c 8b 62 08          	mov    r12,QWORD PTR [rdx+0x8]
    4deb:	48 8d 50 01          	lea    rdx,[rax+0x1]
    4def:	49 89 c5             	mov    r13,rax
    4df2:	4c 8d 05 87 f2 00 00 	lea    r8,[rip+0xf287]        # 14080 <gc_stack>
    4df9:	48 6b d2 08          	imul   rdx,rdx,0x8
    4dfd:	4d 89 24 10          	mov    QWORD PTR [r8+rdx*1],r12
    4e01:	4c 8d 05 78 f2 00 02 	lea    r8,[rip+0x200f278]        # 2014080 <gc_sp>
    4e08:	4d 8b 08             	mov    r9,QWORD PTR [r8]
    4e0b:	4c 8d 15 6e f2 00 00 	lea    r10,[rip+0xf26e]        # 14080 <gc_stack>
    4e12:	4d 6b d9 08          	imul   r11,r9,0x8
    4e16:	4b 89 0c 1a          	mov    QWORD PTR [r10+r11*1],rcx
    4e1a:	49 81 c1 01 00 00 00 	add    r9,0x1
    4e21:	4d 89 08             	mov    QWORD PTR [r8],r9
    4e24:	48 33 f6             	xor    rsi,rsi
    4e27:	4c 8d 15 b5 29 00 00 	lea    r10,[rip+0x29b5]        # 77e3 <cljn_fn_free>
    4e2e:	4c 89 ff             	mov    rdi,r15
    4e31:	41 ff d2             	call   r10
    4e34:	49 89 c6             	mov    r14,rax
    4e37:	4c 8d 15 42 f2 00 02 	lea    r10,[rip+0x200f242]        # 2014080 <gc_sp>
    4e3e:	4d 8b 1a             	mov    r11,QWORD PTR [r10]
    4e41:	48 8d 35 38 f2 00 00 	lea    rsi,[rip+0xf238]        # 14080 <gc_stack>
    4e48:	49 6b fb 08          	imul   rdi,r11,0x8
    4e4c:	48 89 04 3e          	mov    QWORD PTR [rsi+rdi*1],rax
    4e50:	49 81 c3 01 00 00 00 	add    r11,0x1
    4e57:	4d 89 1a             	mov    QWORD PTR [r10],r11
    4e5a:	48 8d 35 1f f2 00 02 	lea    rsi,[rip+0x200f21f]        # 2014080 <gc_sp>
    4e61:	48 8b 3e             	mov    rdi,QWORD PTR [rsi]
    4e64:	48 8d 05 15 f2 00 00 	lea    rax,[rip+0xf215]        # 14080 <gc_stack>
    4e6b:	48 6b cf 08          	imul   rcx,rdi,0x8
    4e6f:	4c 89 24 08          	mov    QWORD PTR [rax+rcx*1],r12
    4e73:	48 81 c7 01 00 00 00 	add    rdi,0x1
    4e7a:	48 89 3e             	mov    QWORD PTR [rsi],rdi
    4e7d:	48 8d 05 98 29 00 00 	lea    rax,[rip+0x2998]        # 781c <cljn_check_fn>
    4e84:	4c 89 f7             	mov    rdi,r14
    4e87:	ff d0                	call   rax
    4e89:	bb 01 00 00 00       	mov    ebx,0x1
    4e8e:	48 8d 0d ba 29 00 00 	lea    rcx,[rip+0x29ba]        # 784f <cljn_argv>
    4e95:	48 89 df             	mov    rdi,rbx
    4e98:	ff d1                	call   rcx
    4e9a:	49 89 c7             	mov    r15,rax
    4e9d:	48 8d 0d 62 29 00 00 	lea    rcx,[rip+0x2962]        # 7806 <cljn_fn_code>
    4ea4:	4c 89 f7             	mov    rdi,r14
    4ea7:	ff d1                	call   rcx
    4ea9:	4c 89 fa             	mov    rdx,r15
    4eac:	48 89 de             	mov    rsi,rbx
    4eaf:	4c 89 f7             	mov    rdi,r14
    4eb2:	ff d0                	call   rax
    4eb4:	48 8d 0d c5 f1 00 02 	lea    rcx,[rip+0x200f1c5]        # 2014080 <gc_sp>
    4ebb:	48 83 01 fe          	add    QWORD PTR [rcx],0xfffffffffffffffe
    4ebf:	48 8d 15 ba f1 00 02 	lea    rdx,[rip+0x200f1ba]        # 2014080 <gc_sp>
    4ec6:	4c 8b 02             	mov    r8,QWORD PTR [rdx]
    4ec9:	4c 8d 0d b0 f1 00 00 	lea    r9,[rip+0xf1b0]        # 14080 <gc_stack>
    4ed0:	4d 6b d0 08          	imul   r10,r8,0x8
    4ed4:	4b 89 04 11          	mov    QWORD PTR [r9+r10*1],rax
    4ed8:	49 81 c0 01 00 00 00 	add    r8,0x1
    4edf:	4c 89 02             	mov    QWORD PTR [rdx],r8
    4ee2:	41 be 02 00 00 00    	mov    r14d,0x2
    4ee8:	4c 8d 0d 60 29 00 00 	lea    r9,[rip+0x2960]        # 784f <cljn_argv>
    4eef:	4c 89 f7             	mov    rdi,r14
    4ef2:	41 ff d1             	call   r9
    4ef5:	bf 02 00 00 00       	mov    edi,0x2
    4efa:	48 89 c2             	mov    rdx,rax
    4efd:	4c 89 f6             	mov    rsi,r14
    4f00:	e8 4b fc ff ff       	call   4b50 <concat>
    4f05:	4c 8d 0d 74 f1 00 02 	lea    r9,[rip+0x200f174]        # 2014080 <gc_sp>
    4f0c:	49 83 01 fe          	add    QWORD PTR [r9],0xfffffffffffffffe
    4f10:	4c 8d 15 69 f1 00 02 	lea    r10,[rip+0x200f169]        # 2014080 <gc_sp>
    4f17:	4d 8b 1a             	mov    r11,QWORD PTR [r10]
    4f1a:	48 8d 35 5f f1 00 00 	lea    rsi,[rip+0xf15f]        # 14080 <gc_stack>
    4f21:	49 6b fb 08          	imul   rdi,r11,0x8
    4f25:	48 89 04 3e          	mov    QWORD PTR [rsi+rdi*1],rax
    4f29:	49 89 c6             	mov    r14,rax
    4f2c:	49 81 c3 01 00 00 00 	add    r11,0x1
    4f33:	4d 89 1a             	mov    QWORD PTR [r10],r11
    4f36:	48 8d 35 0d 1d 00 00 	lea    rsi,[rip+0x1d0d]        # 6c4a <cljn_gc_leave>
    4f3d:	4c 89 ef             	mov    rdi,r13
    4f40:	ff d6                	call   rsi
    4f42:	4c 89 f0             	mov    rax,r14
    4f45:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    4f49:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    4f4e:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    4f53:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    4f58:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    4f5d:	48 83 c4 30          	add    rsp,0x30
    4f61:	48 89 ec             	mov    rsp,rbp
    4f64:	5d                   	pop    rbp
    4f65:	c3                   	ret

0000000000004f66 <mapcat>:
    4f66:	55                   	push   rbp
    4f67:	48 89 e5             	mov    rbp,rsp
    4f6a:	48 83 ec 30          	sub    rsp,0x30
    4f6e:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    4f72:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    4f77:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    4f7c:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    4f81:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
    4f86:	49 89 f4             	mov    r12,rsi
    4f89:	49 89 d6             	mov    r14,rdx
    4f8c:	bf 02 00 00 00       	mov    edi,0x2
    4f91:	48 8d 05 fc 1b 00 00 	lea    rax,[rip+0x1bfc]        # 6b94 <cljn_gc_enter>
    4f98:	ff d0                	call   rax
    4f9a:	4d 89 e2             	mov    r10,r12
    4f9d:	49 83 fa 02          	cmp    r10,0x2
    4fa1:	0f 84 39 00 00 00    	je     4fe0 <mapcat+0x7a>
    4fa7:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    4fae:	48 8d 15 c7 28 00 00 	lea    rdx,[rip+0x28c7]        # 787c <cljn_check_arity>
    4fb5:	4c 89 d7             	mov    rdi,r10
    4fb8:	ff d2                	call   rdx
    4fba:	b8 02 00 00 00       	mov    eax,0x2
    4fbf:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    4fc3:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    4fc8:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    4fcd:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    4fd2:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    4fd7:	48 83 c4 30          	add    rsp,0x30
    4fdb:	48 89 ec             	mov    rsp,rbp
    4fde:	5d                   	pop    rbp
    4fdf:	c3                   	ret
    4fe0:	4c 89 f2             	mov    rdx,r14
    4fe3:	4c 8b 3a             	mov    r15,QWORD PTR [rdx]
    4fe6:	4c 8d 08             	lea    r9,[rax]
    4fe9:	4c 8d 15 90 f0 00 00 	lea    r10,[rip+0xf090]        # 14080 <gc_stack>
    4ff0:	4d 6b c9 08          	imul   r9,r9,0x8
    4ff4:	4f 89 3c 0a          	mov    QWORD PTR [r10+r9*1],r15
    4ff8:	48 8b 5a 08          	mov    rbx,QWORD PTR [rdx+0x8]
    4ffc:	4c 8d 50 01          	lea    r10,[rax+0x1]
    5000:	49 89 c4             	mov    r12,rax
    5003:	4c 8d 1d 76 f0 00 00 	lea    r11,[rip+0xf076]        # 14080 <gc_stack>
    500a:	4d 6b d2 08          	imul   r10,r10,0x8
    500e:	4b 89 1c 13          	mov    QWORD PTR [r11+r10*1],rbx
    5012:	48 8d 35 67 f0 00 02 	lea    rsi,[rip+0x200f067]        # 2014080 <gc_sp>
    5019:	48 8b 3e             	mov    rdi,QWORD PTR [rsi]
    501c:	48 8d 05 5d f0 00 00 	lea    rax,[rip+0xf05d]        # 14080 <gc_stack>
    5023:	48 6b cf 08          	imul   rcx,rdi,0x8
    5027:	4c 89 3c 08          	mov    QWORD PTR [rax+rcx*1],r15
    502b:	48 81 c7 01 00 00 00 	add    rdi,0x1
    5032:	48 89 3e             	mov    QWORD PTR [rsi],rdi
    5035:	48 8d 3d 15 fd ff ff 	lea    rdi,[rip+0xfffffffffffffd15]        # 4d51 <__lambda_8>
    503c:	be 02 00 00 00       	mov    esi,0x2
    5041:	ba 01 00 00 00       	mov    edx,0x1
    5046:	48 8d 05 d9 26 00 00 	lea    rax,[rip+0x26d9]        # 7726 <cljn_make_fn>
    504d:	ff d0                	call   rax
    504f:	48 8d 0d 2a f0 00 02 	lea    rcx,[rip+0x200f02a]        # 2014080 <gc_sp>
    5056:	48 83 01 ff          	add    QWORD PTR [rcx],0xffffffffffffffff
    505a:	48 8d 0d 1f f0 00 02 	lea    rcx,[rip+0x200f01f]        # 2014080 <gc_sp>
    5061:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    5064:	4c 8d 05 15 f0 00 00 	lea    r8,[rip+0xf015]        # 14080 <gc_stack>
    506b:	4c 6b ca 08          	imul   r9,rdx,0x8
    506f:	4b 89 04 08          	mov    QWORD PTR [r8+r9*1],rax
    5073:	48 81 c2 01 00 00 00 	add    rdx,0x1
    507a:	48 89 11             	mov    QWORD PTR [rcx],rdx
    507d:	48 33 f6             	xor    rsi,rsi
    5080:	4c 8d 05 30 27 00 00 	lea    r8,[rip+0x2730]        # 77b7 <cljn_fn_set_free>
    5087:	4c 89 fa             	mov    rdx,r15
    508a:	48 89 c7             	mov    rdi,rax
    508d:	41 ff d0             	call   r8
    5090:	4c 8d 0d 3b 26 00 00 	lea    r9,[rip+0x263b]        # 76d2 <cljn_empty>
    5097:	41 ff d1             	call   r9
    509a:	4c 8d 0d df ef 00 02 	lea    r9,[rip+0x200efdf]        # 2014080 <gc_sp>
    50a1:	4d 8b 11             	mov    r10,QWORD PTR [r9]
    50a4:	4c 8d 1d d5 ef 00 00 	lea    r11,[rip+0xefd5]        # 14080 <gc_stack>
    50ab:	49 6b f2 08          	imul   rsi,r10,0x8
    50af:	49 89 04 33          	mov    QWORD PTR [r11+rsi*1],rax
    50b3:	49 81 c2 01 00 00 00 	add    r10,0x1
    50ba:	4d 89 11             	mov    QWORD PTR [r9],r10
    50bd:	4c 8d 1d bc ef 00 02 	lea    r11,[rip+0x200efbc]        # 2014080 <gc_sp>
    50c4:	49 83 03 ff          	add    QWORD PTR [r11],0xffffffffffffffff
    50c8:	48 8d 35 b1 ef 00 02 	lea    rsi,[rip+0x200efb1]        # 2014080 <gc_sp>
    50cf:	48 8b 3e             	mov    rdi,QWORD PTR [rsi]
    50d2:	48 8d 0d a7 ef 00 00 	lea    rcx,[rip+0xefa7]        # 14080 <gc_stack>
    50d9:	48 6b d7 08          	imul   rdx,rdi,0x8
    50dd:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    50e1:	48 81 c7 01 00 00 00 	add    rdi,0x1
    50e8:	48 89 3e             	mov    QWORD PTR [rsi],rdi
    50eb:	48 8d 05 8e ef 00 02 	lea    rax,[rip+0x200ef8e]        # 2014080 <gc_sp>
    50f2:	48 8b 08             	mov    rcx,QWORD PTR [rax]
    50f5:	48 8d 15 84 ef 00 00 	lea    rdx,[rip+0xef84]        # 14080 <gc_stack>
    50fc:	4c 6b c1 08          	imul   r8,rcx,0x8
    5100:	4a 89 1c 02          	mov    QWORD PTR [rdx+r8*1],rbx
    5104:	48 81 c1 01 00 00 00 	add    rcx,0x1
    510b:	48 89 08             	mov    QWORD PTR [rax],rcx
    510e:	bb 03 00 00 00       	mov    ebx,0x3
    5113:	48 8d 15 35 27 00 00 	lea    rdx,[rip+0x2735]        # 784f <cljn_argv>
    511a:	48 89 df             	mov    rdi,rbx
    511d:	ff d2                	call   rdx
    511f:	bf 02 00 00 00       	mov    edi,0x2
    5124:	48 89 c2             	mov    rdx,rax
    5127:	48 89 de             	mov    rsi,rbx
    512a:	e8 ad cc ff ff       	call   1ddc <reduce>
    512f:	48 8d 15 4a ef 00 02 	lea    rdx,[rip+0x200ef4a]        # 2014080 <gc_sp>
    5136:	48 83 02 fd          	add    QWORD PTR [rdx],0xfffffffffffffffd
    513a:	4c 8d 05 3f ef 00 02 	lea    r8,[rip+0x200ef3f]        # 2014080 <gc_sp>
    5141:	4d 8b 08             	mov    r9,QWORD PTR [r8]
    5144:	4c 8d 15 35 ef 00 00 	lea    r10,[rip+0xef35]        # 14080 <gc_stack>
    514b:	4d 6b d9 08          	imul   r11,r9,0x8
    514f:	4b 89 04 1a          	mov    QWORD PTR [r10+r11*1],rax
    5153:	49 89 c5             	mov    r13,rax
    5156:	49 81 c1 01 00 00 00 	add    r9,0x1
    515d:	4d 89 08             	mov    QWORD PTR [r8],r9
    5160:	4c 8d 15 e3 1a 00 00 	lea    r10,[rip+0x1ae3]        # 6c4a <cljn_gc_leave>
    5167:	4c 89 e7             	mov    rdi,r12
    516a:	41 ff d2             	call   r10
    516d:	4c 89 e8             	mov    rax,r13
    5170:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    5174:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    5179:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    517e:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    5183:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    5188:	48 83 c4 30          	add    rsp,0x30
    518c:	48 89 ec             	mov    rsp,rbp
    518f:	5d                   	pop    rbp
    5190:	c3                   	ret
    5191:	00 00                	add    BYTE PTR [rax],al
    5193:	00 00                	add    BYTE PTR [rax],al
    5195:	00 00                	add    BYTE PTR [rax],al
	...

0000000000005198 <__lambda_9>:
    5198:	55                   	push   rbp
    5199:	48 89 e5             	mov    rbp,rsp
    519c:	48 83 ec 30          	sub    rsp,0x30
    51a0:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    51a4:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    51a9:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    51ae:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    51b3:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
    51b8:	48 89 fb             	mov    rbx,rdi
    51bb:	49 89 f4             	mov    r12,rsi
    51be:	49 89 d5             	mov    r13,rdx
    51c1:	bf 02 00 00 00       	mov    edi,0x2
    51c6:	4c 8d 05 c7 19 00 00 	lea    r8,[rip+0x19c7]        # 6b94 <cljn_gc_enter>
    51cd:	41 ff d0             	call   r8
    51d0:	4d 89 e2             	mov    r10,r12
    51d3:	49 83 fa 02          	cmp    r10,0x2
    51d7:	0f 84 3a 00 00 00    	je     5217 <__lambda_9+0x7f>
    51dd:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    51e4:	4c 8d 1d 91 26 00 00 	lea    r11,[rip+0x2691]        # 787c <cljn_check_arity>
    51eb:	4c 89 d7             	mov    rdi,r10
    51ee:	41 ff d3             	call   r11
    51f1:	b8 02 00 00 00       	mov    eax,0x2
    51f6:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    51fa:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    51ff:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    5204:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    5209:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    520e:	48 83 c4 30          	add    rsp,0x30
    5212:	48 89 ec             	mov    rsp,rbp
    5215:	5d                   	pop    rbp
    5216:	c3                   	ret
    5217:	4c 89 ea             	mov    rdx,r13
    521a:	4c 8b 2a             	mov    r13,QWORD PTR [rdx]
    521d:	48 8d 30             	lea    rsi,[rax]
    5220:	48 8d 3d 59 ee 00 00 	lea    rdi,[rip+0xee59]        # 14080 <gc_stack>
    5227:	48 6b f6 08          	imul   rsi,rsi,0x8
    522b:	4c 89 2c 37          	mov    QWORD PTR [rdi+rsi*1],r13
    522f:	4c 8b 72 08          	mov    r14,QWORD PTR [rdx+0x8]
    5233:	48 8d 78 01          	lea    rdi,[rax+0x1]
    5237:	49 89 c7             	mov    r15,rax
    523a:	48 8d 05 3f ee 00 00 	lea    rax,[rip+0xee3f]        # 14080 <gc_stack>
    5241:	48 6b ff 08          	imul   rdi,rdi,0x8
    5245:	4c 89 34 38          	mov    QWORD PTR [rax+rdi*1],r14
    5249:	48 33 f6             	xor    rsi,rsi
    524c:	48 8d 0d 90 25 00 00 	lea    rcx,[rip+0x2590]        # 77e3 <cljn_fn_free>
    5253:	48 89 df             	mov    rdi,rbx
    5256:	ff d1                	call   rcx
    5258:	49 89 c4             	mov    r12,rax
    525b:	48 8d 0d 1e ee 00 02 	lea    rcx,[rip+0x200ee1e]        # 2014080 <gc_sp>
    5262:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    5265:	4c 8d 05 14 ee 00 00 	lea    r8,[rip+0xee14]        # 14080 <gc_stack>
    526c:	4c 6b ca 08          	imul   r9,rdx,0x8
    5270:	4b 89 04 08          	mov    QWORD PTR [r8+r9*1],rax
    5274:	48 81 c2 01 00 00 00 	add    rdx,0x1
    527b:	48 89 11             	mov    QWORD PTR [rcx],rdx
    527e:	4c 8d 05 fb ed 00 02 	lea    r8,[rip+0x200edfb]        # 2014080 <gc_sp>
    5285:	4d 8b 08             	mov    r9,QWORD PTR [r8]
    5288:	4c 8d 15 f1 ed 00 00 	lea    r10,[rip+0xedf1]        # 14080 <gc_stack>
    528f:	4d 6b d9 08          	imul   r11,r9,0x8
    5293:	4f 89 34 1a          	mov    QWORD PTR [r10+r11*1],r14
    5297:	49 81 c1 01 00 00 00 	add    r9,0x1
    529e:	4d 89 08             	mov    QWORD PTR [r8],r9
    52a1:	4c 8d 15 74 25 00 00 	lea    r10,[rip+0x2574]        # 781c <cljn_check_fn>
    52a8:	4c 89 e7             	mov    rdi,r12
    52ab:	41 ff d2             	call   r10
    52ae:	bb 01 00 00 00       	mov    ebx,0x1
    52b3:	4c 8d 1d 95 25 00 00 	lea    r11,[rip+0x2595]        # 784f <cljn_argv>
    52ba:	48 89 df             	mov    rdi,rbx
    52bd:	41 ff d3             	call   r11
    52c0:	49 89 c6             	mov    r14,rax
    52c3:	4c 8d 1d 3c 25 00 00 	lea    r11,[rip+0x253c]        # 7806 <cljn_fn_code>
    52ca:	4c 89 e7             	mov    rdi,r12
    52cd:	41 ff d3             	call   r11
    52d0:	4c 89 f2             	mov    rdx,r14
    52d3:	48 89 de             	mov    rsi,rbx
    52d6:	4c 89 e7             	mov    rdi,r12
    52d9:	ff d0                	call   rax
    52db:	4c 8d 1d 9e ed 00 02 	lea    r11,[rip+0x200ed9e]        # 2014080 <gc_sp>
    52e2:	49 83 03 fe          	add    QWORD PTR [r11],0xfffffffffffffffe
    52e6:	48 8d 35 93 ed 00 02 	lea    rsi,[rip+0x200ed93]        # 2014080 <gc_sp>
    52ed:	48 8b 3e             	mov    rdi,QWORD PTR [rsi]
    52f0:	48 8d 0d 89 ed 00 00 	lea    rcx,[rip+0xed89]        # 14080 <gc_stack>
    52f7:	48 6b d7 08          	imul   rdx,rdi,0x8
    52fb:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    52ff:	48 81 c7 01 00 00 00 	add    rdi,0x1
    5306:	48 89 3e             	mov    QWORD PTR [rsi],rdi
    5309:	48 8d 0d 6f 89 00 00 	lea    rcx,[rip+0x896f]        # dc7f <cljn_truthy>
    5310:	48 89 c7             	mov    rdi,rax
    5313:	ff d1                	call   rcx
    5315:	48 8d 0d 64 ed 00 02 	lea    rcx,[rip+0x200ed64]        # 2014080 <gc_sp>
    531c:	48 83 01 ff          	add    QWORD PTR [rcx],0xffffffffffffffff
    5320:	85 c0                	test   eax,eax
    5322:	0f 85 2b 00 00 00    	jne    5353 <__lambda_9+0x1bb>
    5328:	4c 8d 05 51 ed 00 02 	lea    r8,[rip+0x200ed51]        # 2014080 <gc_sp>
    532f:	4d 8b 08             	mov    r9,QWORD PTR [r8]
    5332:	4c 8d 15 47 ed 00 00 	lea    r10,[rip+0xed47]        # 14080 <gc_stack>
    5339:	4d 6b d9 08          	imul   r11,r9,0x8
    533d:	4f 89 2c 1a          	mov    QWORD PTR [r10+r11*1],r13
    5341:	49 81 c1 01 00 00 00 	add    r9,0x1
    5348:	4d 89 08             	mov    QWORD PTR [r8],r9
    534b:	4c 89 eb             	mov    rbx,r13
    534e:	e9 75 00 00 00       	jmp    53c8 <__lambda_9+0x230>
    5353:	49 f7 c5 01 00 00 00 	test   r13,0x1
    535a:	0f 84 29 00 00 00    	je     5389 <__lambda_9+0x1f1>
    5360:	4c 89 e8             	mov    rax,r13
    5363:	48 d1 f8             	sar    rax,1
    5366:	48 81 c0 01 00 00 00 	add    rax,0x1
    536d:	48 3b 05 84 00 00 00 	cmp    rax,QWORD PTR [rip+0x84]        # 53f8 <__lambda_9+0x260>
    5374:	0f 9d c1             	setge  cl
    5377:	48 3b 05 82 00 00 00 	cmp    rax,QWORD PTR [rip+0x82]        # 5400 <__lambda_9+0x268>
    537e:	0f 9e c2             	setle  dl
    5381:	84 ca                	test   dl,cl
    5383:	0f 85 12 00 00 00    	jne    539b <__lambda_9+0x203>
    5389:	4c 8d 15 ec 7e 00 00 	lea    r10,[rip+0x7eec]        # d27c <cljn_inc>
    5390:	4c 89 ef             	mov    rdi,r13
    5393:	41 ff d2             	call   r10
    5396:	e9 07 00 00 00       	jmp    53a2 <__lambda_9+0x20a>
    539b:	48 d1 e0             	shl    rax,1
    539e:	48 83 c8 01          	or     rax,0x1
    53a2:	48 8d 35 d7 ec 00 02 	lea    rsi,[rip+0x200ecd7]        # 2014080 <gc_sp>
    53a9:	48 8b 3e             	mov    rdi,QWORD PTR [rsi]
    53ac:	48 8d 0d cd ec 00 00 	lea    rcx,[rip+0xeccd]        # 14080 <gc_stack>
    53b3:	48 6b d7 08          	imul   rdx,rdi,0x8
    53b7:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    53bb:	48 89 c3             	mov    rbx,rax
    53be:	48 81 c7 01 00 00 00 	add    rdi,0x1
    53c5:	48 89 3e             	mov    QWORD PTR [rsi],rdi
    53c8:	48 8d 0d 7b 18 00 00 	lea    rcx,[rip+0x187b]        # 6c4a <cljn_gc_leave>
    53cf:	4c 89 ff             	mov    rdi,r15
    53d2:	ff d1                	call   rcx
    53d4:	48 89 d8             	mov    rax,rbx
    53d7:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    53db:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    53e0:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    53e5:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    53ea:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    53ef:	48 83 c4 30          	add    rsp,0x30
    53f3:	48 89 ec             	mov    rsp,rbp
    53f6:	5d                   	pop    rbp
    53f7:	c3                   	ret
    53f8:	00 00                	add    BYTE PTR [rax],al
    53fa:	00 00                	add    BYTE PTR [rax],al
    53fc:	00 00                	add    BYTE PTR [rax],al
    53fe:	00 c0                	add    al,al
    5400:	ff                   	(bad)
    5401:	ff                   	(bad)
    5402:	ff                   	(bad)
    5403:	ff                   	(bad)
    5404:	ff                   	(bad)
    5405:	ff                   	(bad)
    5406:	ff                   	(bad)
    5407:	3f                   	(bad)

0000000000005408 <count-if>:
    5408:	55                   	push   rbp
    5409:	48 89 e5             	mov    rbp,rsp
    540c:	48 83 ec 30          	sub    rsp,0x30
    5410:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    5414:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    5419:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    541e:	4c 89 74 24 18       	mov    QWORD PTR [rsp+0x18],r14
    5423:	4c 89 7c 24 20       	mov    QWORD PTR [rsp+0x20],r15
    5428:	48 89 d3             	mov    rbx,rdx
    542b:	49 89 f6             	mov    r14,rsi
    542e:	bf 02 00 00 00       	mov    edi,0x2
    5433:	48 8d 0d 5a 17 00 00 	lea    rcx,[rip+0x175a]        # 6b94 <cljn_gc_enter>
    543a:	ff d1                	call   rcx
    543c:	4d 89 f3             	mov    r11,r14
    543f:	49 83 fb 02          	cmp    r11,0x2
    5443:	0f 84 3a 00 00 00    	je     5483 <count-if+0x7b>
    5449:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    5450:	4c 8d 05 25 24 00 00 	lea    r8,[rip+0x2425]        # 787c <cljn_check_arity>
    5457:	4c 89 df             	mov    rdi,r11
    545a:	41 ff d0             	call   r8
    545d:	b8 02 00 00 00       	mov    eax,0x2
    5462:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    5466:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    546b:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    5470:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    5475:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    547a:	48 83 c4 30          	add    rsp,0x30
    547e:	48 89 ec             	mov    rsp,rbp
    5481:	5d                   	pop    rbp
    5482:	c3                   	ret
    5483:	48 89 da             	mov    rdx,rbx
    5486:	4c 8b 22             	mov    r12,QWORD PTR [rdx]
    5489:	4c 8d 10             	lea    r10,[rax]
    548c:	4c 8d 1d ed eb 00 00 	lea    r11,[rip+0xebed]        # 14080 <gc_stack>
    5493:	4d 6b d2 08          	imul   r10,r10,0x8
    5497:	4f 89 24 13          	mov    QWORD PTR [r11+r10*1],r12
    549b:	4c 8b 6a 08          	mov    r13,QWORD PTR [rdx+0x8]
    549f:	4c 8d 58 01          	lea    r11,[rax+0x1]
    54a3:	49 89 c6             	mov    r14,rax
    54a6:	48 8d 35 d3 eb 00 00 	lea    rsi,[rip+0xebd3]        # 14080 <gc_stack>
    54ad:	4d 6b db 08          	imul   r11,r11,0x8
    54b1:	4e 89 2c 1e          	mov    QWORD PTR [rsi+r11*1],r13
    54b5:	48 8d 3d c4 eb 00 02 	lea    rdi,[rip+0x200ebc4]        # 2014080 <gc_sp>
    54bc:	48 8b 07             	mov    rax,QWORD PTR [rdi]
    54bf:	48 8d 0d ba eb 00 00 	lea    rcx,[rip+0xebba]        # 14080 <gc_stack>
    54c6:	48 6b d0 08          	imul   rdx,rax,0x8
    54ca:	4c 89 24 11          	mov    QWORD PTR [rcx+rdx*1],r12
    54ce:	48 81 c0 01 00 00 00 	add    rax,0x1
    54d5:	48 89 07             	mov    QWORD PTR [rdi],rax
    54d8:	48 8d 3d b9 fc ff ff 	lea    rdi,[rip+0xfffffffffffffcb9]        # 5198 <__lambda_9>
    54df:	be 02 00 00 00       	mov    esi,0x2
    54e4:	ba 01 00 00 00       	mov    edx,0x1
    54e9:	48 8d 0d 36 22 00 00 	lea    rcx,[rip+0x2236]        # 7726 <cljn_make_fn>
    54f0:	ff d1                	call   rcx
    54f2:	48 8d 0d 87 eb 00 02 	lea    rcx,[rip+0x200eb87]        # 2014080 <gc_sp>
    54f9:	48 83 01 ff          	add    QWORD PTR [rcx],0xffffffffffffffff
    54fd:	48 8d 15 7c eb 00 02 	lea    rdx,[rip+0x200eb7c]        # 2014080 <gc_sp>
    5504:	4c 8b 02             	mov    r8,QWORD PTR [rdx]
    5507:	4c 8d 0d 72 eb 00 00 	lea    r9,[rip+0xeb72]        # 14080 <gc_stack>
    550e:	4d 6b d0 08          	imul   r10,r8,0x8
    5512:	4b 89 04 11          	mov    QWORD PTR [r9+r10*1],rax
    5516:	49 81 c0 01 00 00 00 	add    r8,0x1
    551d:	4c 89 02             	mov    QWORD PTR [rdx],r8
    5520:	48 33 f6             	xor    rsi,rsi
    5523:	4c 8d 0d 8d 22 00 00 	lea    r9,[rip+0x228d]        # 77b7 <cljn_fn_set_free>
    552a:	4c 89 e2             	mov    rdx,r12
    552d:	48 89 c7             	mov    rdi,rax
    5530:	41 ff d1             	call   r9
    5533:	4c 8d 15 46 eb 00 02 	lea    r10,[rip+0x200eb46]        # 2014080 <gc_sp>
    553a:	4d 8b 1a             	mov    r11,QWORD PTR [r10]
    553d:	48 8d 35 3c eb 00 00 	lea    rsi,[rip+0xeb3c]        # 14080 <gc_stack>
    5544:	49 6b fb 08          	imul   rdi,r11,0x8
    5548:	48 c7 04 3e 01 00 00 	mov    QWORD PTR [rsi+rdi*1],0x1
    554f:	00 
    5550:	49 81 c3 01 00 00 00 	add    r11,0x1
    5557:	4d 89 1a             	mov    QWORD PTR [r10],r11
    555a:	48 8d 35 1f eb 00 02 	lea    rsi,[rip+0x200eb1f]        # 2014080 <gc_sp>
    5561:	48 8b 3e             	mov    rdi,QWORD PTR [rsi]
    5564:	48 8d 05 15 eb 00 00 	lea    rax,[rip+0xeb15]        # 14080 <gc_stack>
    556b:	48 6b cf 08          	imul   rcx,rdi,0x8
    556f:	4c 89 2c 08          	mov    QWORD PTR [rax+rcx*1],r13
    5573:	48 81 c7 01 00 00 00 	add    rdi,0x1
    557a:	48 89 3e             	mov    QWORD PTR [rsi],rdi
    557d:	41 bf 03 00 00 00    	mov    r15d,0x3
    5583:	48 8d 05 c5 22 00 00 	lea    rax,[rip+0x22c5]        # 784f <cljn_argv>
    558a:	4c 89 ff             	mov    rdi,r15
    558d:	ff d0                	call   rax
    558f:	bf 02 00 00 00       	mov    edi,0x2
    5594:	48 89 c2             	mov    rdx,rax
    5597:	4c 89 fe             	mov    rsi,r15
    559a:	e8 3d c8 ff ff       	call   1ddc <reduce>
    559f:	48 8d 0d da ea 00 02 	lea    rcx,[rip+0x200eada]        # 2014080 <gc_sp>
    55a6:	48 83 01 fd          	add    QWORD PTR [rcx],0xfffffffffffffffd
    55aa:	48 8d 0d cf ea 00 02 	lea    rcx,[rip+0x200eacf]        # 2014080 <gc_sp>
    55b1:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    55b4:	4c 8d 05 c5 ea 00 00 	lea    r8,[rip+0xeac5]        # 14080 <gc_stack>
    55bb:	4c 6b ca 08          	imul   r9,rdx,0x8
    55bf:	4b 89 04 08          	mov    QWORD PTR [r8+r9*1],rax
    55c3:	49 89 c7             	mov    r15,rax
    55c6:	48 81 c2 01 00 00 00 	add    rdx,0x1
    55cd:	48 89 11             	mov    QWORD PTR [rcx],rdx
    55d0:	4c 8d 05 73 16 00 00 	lea    r8,[rip+0x1673]        # 6c4a <cljn_gc_leave>
    55d7:	4c 89 f7             	mov    rdi,r14
    55da:	41 ff d0             	call   r8
    55dd:	4c 89 f8             	mov    rax,r15
    55e0:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    55e4:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    55e9:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    55ee:	4c 8b 74 24 18       	mov    r14,QWORD PTR [rsp+0x18]
    55f3:	4c 8b 7c 24 20       	mov    r15,QWORD PTR [rsp+0x20]
    55f8:	48 83 c4 30          	add    rsp,0x30
    55fc:	48 89 ec             	mov    rsp,rbp
    55ff:	5d                   	pop    rbp
    5600:	c3                   	ret
    5601:	00 00                	add    BYTE PTR [rax],al
    5603:	00 00                	add    BYTE PTR [rax],al
    5605:	00 00                	add    BYTE PTR [rax],al
	...

0000000000005608 <initial-flags>:
    5608:	55                   	push   rbp
    5609:	48 89 e5             	mov    rbp,rsp
    560c:	48 83 ec 20          	sub    rsp,0x20
    5610:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    5614:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    5619:	4c 89 74 24 10       	mov    QWORD PTR [rsp+0x10],r14
    561e:	4c 89 7c 24 18       	mov    QWORD PTR [rsp+0x18],r15
    5623:	49 89 d4             	mov    r12,rdx
    5626:	49 89 f7             	mov    r15,rsi
    5629:	bf 03 00 00 00       	mov    edi,0x3
    562e:	48 8d 35 5f 15 00 00 	lea    rsi,[rip+0x155f]        # 6b94 <cljn_gc_enter>
    5635:	ff d6                	call   rsi
    5637:	48 89 c3             	mov    rbx,rax
    563a:	4d 89 f8             	mov    r8,r15
    563d:	49 83 f8 01          	cmp    r8,0x1
    5641:	0f 84 34 00 00 00    	je     567b <initial-flags+0x73>
    5647:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    564e:	48 8d 05 27 22 00 00 	lea    rax,[rip+0x2227]        # 787c <cljn_check_arity>
    5655:	4c 89 c7             	mov    rdi,r8
    5658:	ff d0                	call   rax
    565a:	b8 02 00 00 00       	mov    eax,0x2
    565f:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    5663:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    5668:	4c 8b 74 24 10       	mov    r14,QWORD PTR [rsp+0x10]
    566d:	4c 8b 7c 24 18       	mov    r15,QWORD PTR [rsp+0x18]
    5672:	48 83 c4 20          	add    rsp,0x20
    5676:	48 89 ec             	mov    rsp,rbp
    5679:	5d                   	pop    rbp
    567a:	c3                   	ret
    567b:	4c 89 e2             	mov    rdx,r12
    567e:	4c 8b 3a             	mov    r15,QWORD PTR [rdx]
    5681:	49 89 de             	mov    r14,rbx
    5684:	49 8d 16             	lea    rdx,[r14]
    5687:	4c 8d 05 f2 e9 00 00 	lea    r8,[rip+0xe9f2]        # 14080 <gc_stack>
    568e:	48 6b d2 08          	imul   rdx,rdx,0x8
    5692:	4d 89 3c 10          	mov    QWORD PTR [r8+rdx*1],r15
    5696:	bf 01 00 00 00       	mov    edi,0x1
    569b:	49 89 fc             	mov    r12,rdi
    569e:	4c 8d 0d 8a 28 00 00 	lea    r9,[rip+0x288a]        # 7f2f <cljn_vec_empty>
    56a5:	41 ff d1             	call   r9
    56a8:	4c 8d 0d d1 e9 00 02 	lea    r9,[rip+0x200e9d1]        # 2014080 <gc_sp>
    56af:	4d 8b 11             	mov    r10,QWORD PTR [r9]
    56b2:	4c 8d 1d c7 e9 00 00 	lea    r11,[rip+0xe9c7]        # 14080 <gc_stack>
    56b9:	49 6b f2 08          	imul   rsi,r10,0x8
    56bd:	49 89 04 33          	mov    QWORD PTR [r11+rsi*1],rax
    56c1:	49 81 c2 01 00 00 00 	add    r10,0x1
    56c8:	4d 89 11             	mov    QWORD PTR [r9],r10
    56cb:	4c 8d 1d ae e9 00 02 	lea    r11,[rip+0x200e9ae]        # 2014080 <gc_sp>
    56d2:	49 83 03 ff          	add    QWORD PTR [r11],0xffffffffffffffff
    56d6:	48 8d 35 a3 e9 00 02 	lea    rsi,[rip+0x200e9a3]        # 2014080 <gc_sp>
    56dd:	48 8b 3e             	mov    rdi,QWORD PTR [rsi]
    56e0:	48 8d 0d 99 e9 00 00 	lea    rcx,[rip+0xe999]        # 14080 <gc_stack>
    56e7:	48 6b d7 08          	imul   rdx,rdi,0x8
    56eb:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    56ef:	48 81 c7 01 00 00 00 	add    rdi,0x1
    56f6:	48 89 3e             	mov    QWORD PTR [rsi],rdi
    56f9:	48 8d 0d 2f 70 00 00 	lea    rcx,[rip+0x702f]        # c72f <cljn_transient>
    5700:	48 89 c7             	mov    rdi,rax
    5703:	ff d1                	call   rcx
    5705:	48 8d 0d 74 e9 00 02 	lea    rcx,[rip+0x200e974]        # 2014080 <gc_sp>
    570c:	48 83 01 ff          	add    QWORD PTR [rcx],0xffffffffffffffff
    5710:	48 8d 0d 69 e9 00 02 	lea    rcx,[rip+0x200e969]        # 2014080 <gc_sp>
    5717:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    571a:	4c 8d 05 5f e9 00 00 	lea    r8,[rip+0xe95f]        # 14080 <gc_stack>
    5721:	4c 6b ca 08          	imul   r9,rdx,0x8
    5725:	4b 89 04 08          	mov    QWORD PTR [r8+r9*1],rax
    5729:	48 81 c2 01 00 00 00 	add    rdx,0x1
    5730:	48 89 11             	mov    QWORD PTR [rcx],rdx
    5733:	4d 8d 46 02          	lea    r8,[r14+0x2]
    5737:	4c 8d 0d 42 e9 00 00 	lea    r9,[rip+0xe942]        # 14080 <gc_stack>
    573e:	4d 6b c0 08          	imul   r8,r8,0x8
    5742:	4b 89 04 01          	mov    QWORD PTR [r9+r8*1],rax
    5746:	4c 8d 0d 33 e9 00 02 	lea    r9,[rip+0x200e933]        # 2014080 <gc_sp>
    574d:	49 83 01 ff          	add    QWORD PTR [r9],0xffffffffffffffff
    5751:	48 89 c3             	mov    rbx,rax
    5754:	4c 89 e7             	mov    rdi,r12
    5757:	49 89 fb             	mov    r11,rdi
    575a:	4d 23 df             	and    r11,r15
    575d:	49 89 fc             	mov    r12,rdi
    5760:	49 f7 c3 01 00 00 00 	test   r11,0x1
    5767:	0f 85 14 00 00 00    	jne    5781 <initial-flags+0x179>
    576d:	48 8d 05 f7 7c 00 00 	lea    rax,[rip+0x7cf7]        # d46b <cljn_gt>
    5774:	4c 89 fe             	mov    rsi,r15
    5777:	4c 89 e7             	mov    rdi,r12
    577a:	ff d0                	call   rax
    577c:	e9 1f 00 00 00       	jmp    57a0 <initial-flags+0x198>
    5781:	4c 89 e7             	mov    rdi,r12
    5784:	48 89 f9             	mov    rcx,rdi
    5787:	48 d1 f9             	sar    rcx,1
    578a:	4c 89 fa             	mov    rdx,r15
    578d:	48 d1 fa             	sar    rdx,1
    5790:	b8 06 00 00 00       	mov    eax,0x6
    5795:	48 3b ca             	cmp    rcx,rdx
    5798:	48 0f 4f 05 c0 01 00 	cmovg  rax,QWORD PTR [rip+0x1c0]        # 5960 <initial-flags+0x358>
    579f:	00 
    57a0:	48 83 f8 06          	cmp    rax,0x6
    57a4:	41 0f 95 c0          	setne  r8b
    57a8:	48 83 f8 02          	cmp    rax,0x2
    57ac:	41 0f 95 c1          	setne  r9b
    57b0:	45 84 c1             	test   r9b,r8b
    57b3:	0f 85 b8 00 00 00    	jne    5871 <initial-flags+0x269>
    57b9:	4c 89 e7             	mov    rdi,r12
    57bc:	48 f7 c7 01 00 00 00 	test   rdi,0x1
    57c3:	0f 84 29 00 00 00    	je     57f2 <initial-flags+0x1ea>
    57c9:	4c 89 e7             	mov    rdi,r12
    57cc:	48 89 f8             	mov    rax,rdi
    57cf:	48 d1 f8             	sar    rax,1
    57d2:	4c 8d 40 01          	lea    r8,[rax+0x1]
    57d6:	4c 3b 05 8b 01 00 00 	cmp    r8,QWORD PTR [rip+0x18b]        # 5968 <initial-flags+0x360>
    57dd:	0f 9d c1             	setge  cl
    57e0:	4c 3b 05 89 01 00 00 	cmp    r8,QWORD PTR [rip+0x189]        # 5970 <initial-flags+0x368>
    57e7:	0f 9e c2             	setle  dl
    57ea:	84 ca                	test   dl,cl
    57ec:	0f 85 15 00 00 00    	jne    5807 <initial-flags+0x1ff>
    57f2:	4c 8d 15 83 7a 00 00 	lea    r10,[rip+0x7a83]        # d27c <cljn_inc>
    57f9:	4c 89 e7             	mov    rdi,r12
    57fc:	41 ff d2             	call   r10
    57ff:	49 89 c4             	mov    r12,rax
    5802:	e9 0a 00 00 00       	jmp    5811 <initial-flags+0x209>
    5807:	49 d1 e0             	shl    r8,1
    580a:	49 83 c8 01          	or     r8,0x1
    580e:	4d 89 c4             	mov    r12,r8
    5811:	be 0a 00 00 00       	mov    esi,0xa
    5816:	48 8d 05 89 70 00 00 	lea    rax,[rip+0x7089]        # c8a6 <cljn_conj_bang>
    581d:	48 89 df             	mov    rdi,rbx
    5820:	ff d0                	call   rax
    5822:	48 8d 35 57 e8 00 02 	lea    rsi,[rip+0x200e857]        # 2014080 <gc_sp>
    5829:	48 8b 3e             	mov    rdi,QWORD PTR [rsi]
    582c:	48 8d 0d 4d e8 00 00 	lea    rcx,[rip+0xe84d]        # 14080 <gc_stack>
    5833:	48 6b d7 08          	imul   rdx,rdi,0x8
    5837:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    583b:	48 81 c7 01 00 00 00 	add    rdi,0x1
    5842:	48 89 3e             	mov    QWORD PTR [rsi],rdi
    5845:	49 8d 4e 02          	lea    rcx,[r14+0x2]
    5849:	4c 89 f3             	mov    rbx,r14
    584c:	48 8d 15 2d e8 00 00 	lea    rdx,[rip+0xe82d]        # 14080 <gc_stack>
    5853:	48 6b c9 08          	imul   rcx,rcx,0x8
    5857:	48 89 04 0a          	mov    QWORD PTR [rdx+rcx*1],rax
    585b:	48 8d 0d 1e e8 00 02 	lea    rcx,[rip+0x200e81e]        # 2014080 <gc_sp>
    5862:	48 83 01 ff          	add    QWORD PTR [rcx],0xffffffffffffffff
    5866:	4c 89 e7             	mov    rdi,r12
    5869:	48 89 c3             	mov    rbx,rax
    586c:	e9 e6 fe ff ff       	jmp    5757 <initial-flags+0x14f>
    5871:	48 89 df             	mov    rdi,rbx
    5874:	4c 89 f3             	mov    rbx,r14
    5877:	be 01 00 00 00       	mov    esi,0x1
    587c:	ba 06 00 00 00       	mov    edx,0x6
    5881:	4c 8d 05 7b 72 00 00 	lea    r8,[rip+0x727b]        # cb03 <cljn_assoc_bang>
    5888:	41 ff d0             	call   r8
    588b:	4c 8d 05 ee e7 00 02 	lea    r8,[rip+0x200e7ee]        # 2014080 <gc_sp>
    5892:	4d 8b 08             	mov    r9,QWORD PTR [r8]
    5895:	4c 8d 15 e4 e7 00 00 	lea    r10,[rip+0xe7e4]        # 14080 <gc_stack>
    589c:	4d 6b d9 08          	imul   r11,r9,0x8
    58a0:	4b 89 04 1a          	mov    QWORD PTR [r10+r11*1],rax
    58a4:	49 81 c1 01 00 00 00 	add    r9,0x1
    58ab:	4d 89 08             	mov    QWORD PTR [r8],r9
    58ae:	be 03 00 00 00       	mov    esi,0x3
    58b3:	ba 06 00 00 00       	mov    edx,0x6
    58b8:	4c 8d 15 44 72 00 00 	lea    r10,[rip+0x7244]        # cb03 <cljn_assoc_bang>
    58bf:	48 89 c7             	mov    rdi,rax
    58c2:	41 ff d2             	call   r10
    58c5:	4c 8d 15 b4 e7 00 02 	lea    r10,[rip+0x200e7b4]        # 2014080 <gc_sp>
    58cc:	49 83 02 ff          	add    QWORD PTR [r10],0xffffffffffffffff
    58d0:	4c 8d 1d a9 e7 00 02 	lea    r11,[rip+0x200e7a9]        # 2014080 <gc_sp>
    58d7:	49 8b 33             	mov    rsi,QWORD PTR [r11]
    58da:	48 8d 3d 9f e7 00 00 	lea    rdi,[rip+0xe79f]        # 14080 <gc_stack>
    58e1:	48 6b ce 08          	imul   rcx,rsi,0x8
    58e5:	48 89 04 0f          	mov    QWORD PTR [rdi+rcx*1],rax
    58e9:	48 81 c6 01 00 00 00 	add    rsi,0x1
    58f0:	49 89 33             	mov    QWORD PTR [r11],rsi
    58f3:	48 8d 0d 51 74 00 00 	lea    rcx,[rip+0x7451]        # cd4b <cljn_persistent_bang>
    58fa:	48 89 c7             	mov    rdi,rax
    58fd:	ff d1                	call   rcx
    58ff:	48 8d 3d 7a e7 00 02 	lea    rdi,[rip+0x200e77a]        # 2014080 <gc_sp>
    5906:	48 83 07 ff          	add    QWORD PTR [rdi],0xffffffffffffffff
    590a:	48 8d 0d 6f e7 00 02 	lea    rcx,[rip+0x200e76f]        # 2014080 <gc_sp>
    5911:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    5914:	4c 8d 05 65 e7 00 00 	lea    r8,[rip+0xe765]        # 14080 <gc_stack>
    591b:	4c 6b ca 08          	imul   r9,rdx,0x8
    591f:	4b 89 04 08          	mov    QWORD PTR [r8+r9*1],rax
    5923:	49 89 c7             	mov    r15,rax
    5926:	48 81 c2 01 00 00 00 	add    rdx,0x1
    592d:	48 89 11             	mov    QWORD PTR [rcx],rdx
    5930:	4c 8d 05 13 13 00 00 	lea    r8,[rip+0x1313]        # 6c4a <cljn_gc_leave>
    5937:	48 89 df             	mov    rdi,rbx
    593a:	41 ff d0             	call   r8
    593d:	4c 89 f8             	mov    rax,r15
    5940:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    5944:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    5949:	4c 8b 74 24 10       	mov    r14,QWORD PTR [rsp+0x10]
    594e:	4c 8b 7c 24 18       	mov    r15,QWORD PTR [rsp+0x18]
    5953:	48 83 c4 20          	add    rsp,0x20
    5957:	48 89 ec             	mov    rsp,rbp
    595a:	5d                   	pop    rbp
    595b:	c3                   	ret
    595c:	00 00                	add    BYTE PTR [rax],al
    595e:	00 00                	add    BYTE PTR [rax],al
    5960:	0a 00                	or     al,BYTE PTR [rax]
	...
    596e:	00 c0                	add    al,al
    5970:	ff                   	(bad)
    5971:	ff                   	(bad)
    5972:	ff                   	(bad)
    5973:	ff                   	(bad)
    5974:	ff                   	(bad)
    5975:	ff                   	(bad)
    5976:	ff                   	(bad)
    5977:	3f                   	(bad)

0000000000005978 <mark-multiples>:
    5978:	55                   	push   rbp
    5979:	48 89 e5             	mov    rbp,rsp
    597c:	48 83 ec 40          	sub    rsp,0x40
    5980:	48 89 5c 24 10       	mov    QWORD PTR [rsp+0x10],rbx
    5985:	4c 89 64 24 18       	mov    QWORD PTR [rsp+0x18],r12
    598a:	4c 89 6c 24 20       	mov    QWORD PTR [rsp+0x20],r13
    598f:	4c 89 74 24 28       	mov    QWORD PTR [rsp+0x28],r14
    5994:	4c 89 7c 24 30       	mov    QWORD PTR [rsp+0x30],r15
    5999:	48 89 d3             	mov    rbx,rdx
    599c:	49 89 f6             	mov    r14,rsi
    599f:	bf 05 00 00 00       	mov    edi,0x5
    59a4:	48 8d 0d e9 11 00 00 	lea    rcx,[rip+0x11e9]        # 6b94 <cljn_gc_enter>
    59ab:	ff d1                	call   rcx
    59ad:	4c 89 f1             	mov    rcx,r14
    59b0:	49 89 c4             	mov    r12,rax
    59b3:	48 83 f9 03          	cmp    rcx,0x3
    59b7:	0f 84 3b 00 00 00    	je     59f8 <mark-multiples+0x80>
    59bd:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    59c4:	4c 8d 05 b1 1e 00 00 	lea    r8,[rip+0x1eb1]        # 787c <cljn_check_arity>
    59cb:	48 89 cf             	mov    rdi,rcx
    59ce:	41 ff d0             	call   r8
    59d1:	b8 02 00 00 00       	mov    eax,0x2
    59d6:	48 8b 5c 24 10       	mov    rbx,QWORD PTR [rsp+0x10]
    59db:	4c 8b 64 24 18       	mov    r12,QWORD PTR [rsp+0x18]
    59e0:	4c 8b 6c 24 20       	mov    r13,QWORD PTR [rsp+0x20]
    59e5:	4c 8b 74 24 28       	mov    r14,QWORD PTR [rsp+0x28]
    59ea:	4c 8b 7c 24 30       	mov    r15,QWORD PTR [rsp+0x30]
    59ef:	48 83 c4 40          	add    rsp,0x40
    59f3:	48 89 ec             	mov    rsp,rbp
    59f6:	5d                   	pop    rbp
    59f7:	c3                   	ret
    59f8:	48 89 da             	mov    rdx,rbx
    59fb:	48 8b 3a             	mov    rdi,QWORD PTR [rdx]
    59fe:	4c 89 e0             	mov    rax,r12
    5a01:	4c 8d 10             	lea    r10,[rax]
    5a04:	4c 8d 1d 75 e6 00 00 	lea    r11,[rip+0xe675]        # 14080 <gc_stack>
    5a0b:	4d 6b d2 08          	imul   r10,r10,0x8
    5a0f:	4b 89 3c 13          	mov    QWORD PTR [r11+r10*1],rdi
    5a13:	49 89 fd             	mov    r13,rdi
    5a16:	4c 8b 72 08          	mov    r14,QWORD PTR [rdx+0x8]
    5a1a:	4c 8d 58 01          	lea    r11,[rax+0x1]
    5a1e:	48 8d 35 5b e6 00 00 	lea    rsi,[rip+0xe65b]        # 14080 <gc_stack>
    5a25:	4d 6b db 08          	imul   r11,r11,0x8
    5a29:	4e 89 34 1e          	mov    QWORD PTR [rsi+r11*1],r14
    5a2d:	4c 8b 7a 10          	mov    r15,QWORD PTR [rdx+0x10]
    5a31:	48 8d 70 02          	lea    rsi,[rax+0x2]
    5a35:	48 8d 3d 44 e6 00 00 	lea    rdi,[rip+0xe644]        # 14080 <gc_stack>
    5a3c:	48 6b f6 08          	imul   rsi,rsi,0x8
    5a40:	4c 89 3c 37          	mov    QWORD PTR [rdi+rsi*1],r15
    5a44:	4c 89 f0             	mov    rax,r14
    5a47:	49 23 c6             	and    rax,r14
    5a4a:	48 a9 01 00 00 00    	test   rax,0x1
    5a50:	0f 84 45 00 00 00    	je     5a9b <mark-multiples+0x123>
    5a56:	4c 89 f0             	mov    rax,r14
    5a59:	48 d1 f8             	sar    rax,1
    5a5c:	4d 89 f0             	mov    r8,r14
    5a5f:	49 d1 f8             	sar    r8,1
    5a62:	48 89 c6             	mov    rsi,rax
    5a65:	49 0f af f0          	imul   rsi,r8
    5a69:	49 f7 e8             	imul   r8
    5a6c:	49 89 f0             	mov    r8,rsi
    5a6f:	49 c1 f8 3f          	sar    r8,0x3f
    5a73:	49 3b d0             	cmp    rdx,r8
    5a76:	41 0f 94 c0          	sete   r8b
    5a7a:	48 3b 35 3f 02 00 00 	cmp    rsi,QWORD PTR [rip+0x23f]        # 5cc0 <mark-multiples+0x348>
    5a81:	0f 9d c2             	setge  dl
    5a84:	48 3b 35 3d 02 00 00 	cmp    rsi,QWORD PTR [rip+0x23d]        # 5cc8 <mark-multiples+0x350>
    5a8b:	41 0f 9e c2          	setle  r10b
    5a8f:	41 23 d2             	and    edx,r10d
    5a92:	44 84 c2             	test   dl,r8b
    5a95:	0f 85 1a 00 00 00    	jne    5ab5 <mark-multiples+0x13d>
    5a9b:	48 8d 05 e5 75 00 00 	lea    rax,[rip+0x75e5]        # d087 <cljn_mul>
    5aa2:	4c 89 f6             	mov    rsi,r14
    5aa5:	4c 89 f7             	mov    rdi,r14
    5aa8:	ff d0                	call   rax
    5aaa:	48 89 c6             	mov    rsi,rax
    5aad:	4c 89 e0             	mov    rax,r12
    5ab0:	e9 0a 00 00 00       	jmp    5abf <mark-multiples+0x147>
    5ab5:	48 d1 e6             	shl    rsi,1
    5ab8:	48 83 ce 01          	or     rsi,0x1
    5abc:	4c 89 e0             	mov    rax,r12
    5abf:	4c 89 e0             	mov    rax,r12
    5ac2:	48 8d 48 04          	lea    rcx,[rax+0x4]
    5ac6:	48 8d 15 b3 e5 00 00 	lea    rdx,[rip+0xe5b3]        # 14080 <gc_stack>
    5acd:	48 6b c9 08          	imul   rcx,rcx,0x8
    5ad1:	4c 89 ef             	mov    rdi,r13
    5ad4:	48 89 3c 0a          	mov    QWORD PTR [rdx+rcx*1],rdi
    5ad8:	48 89 f3             	mov    rbx,rsi
    5adb:	49 89 d8             	mov    r8,rbx
    5ade:	4d 23 c7             	and    r8,r15
    5ae1:	49 f7 c0 01 00 00 00 	test   r8,0x1
    5ae8:	0f 85 15 00 00 00    	jne    5b03 <mark-multiples+0x18b>
    5aee:	4c 8d 15 76 79 00 00 	lea    r10,[rip+0x7976]        # d46b <cljn_gt>
    5af5:	4c 89 fe             	mov    rsi,r15
    5af8:	48 89 df             	mov    rdi,rbx
    5afb:	41 ff d2             	call   r10
    5afe:	e9 1c 00 00 00       	jmp    5b1f <mark-multiples+0x1a7>
    5b03:	49 89 db             	mov    r11,rbx
    5b06:	49 d1 fb             	sar    r11,1
    5b09:	4c 89 fe             	mov    rsi,r15
    5b0c:	48 d1 fe             	sar    rsi,1
    5b0f:	b8 06 00 00 00       	mov    eax,0x6
    5b14:	4c 3b de             	cmp    r11,rsi
    5b17:	48 0f 4f 05 b1 01 00 	cmovg  rax,QWORD PTR [rip+0x1b1]        # 5cd0 <mark-multiples+0x358>
    5b1e:	00 
    5b1f:	48 83 f8 06          	cmp    rax,0x6
    5b23:	0f 95 c1             	setne  cl
    5b26:	48 83 f8 02          	cmp    rax,0x2
    5b2a:	0f 95 c2             	setne  dl
    5b2d:	84 ca                	test   dl,cl
    5b2f:	0f 85 2c 01 00 00    	jne    5c61 <mark-multiples+0x2e9>
    5b35:	49 89 d8             	mov    r8,rbx
    5b38:	4d 23 c6             	and    r8,r14
    5b3b:	49 f7 c0 01 00 00 00 	test   r8,0x1
    5b42:	0f 84 2f 00 00 00    	je     5b77 <mark-multiples+0x1ff>
    5b48:	49 89 db             	mov    r11,rbx
    5b4b:	49 d1 fb             	sar    r11,1
    5b4e:	4c 89 f6             	mov    rsi,r14
    5b51:	48 d1 fe             	sar    rsi,1
    5b54:	49 8d 04 33          	lea    rax,[r11+rsi*1]
    5b58:	48 3b 05 61 01 00 00 	cmp    rax,QWORD PTR [rip+0x161]        # 5cc0 <mark-multiples+0x348>
    5b5f:	40 0f 9d c6          	setge  sil
    5b63:	48 3b 05 5e 01 00 00 	cmp    rax,QWORD PTR [rip+0x15e]        # 5cc8 <mark-multiples+0x350>
    5b6a:	40 0f 9e c7          	setle  dil
    5b6e:	40 84 f7             	test   dil,sil
    5b71:	0f 85 18 00 00 00    	jne    5b8f <mark-multiples+0x217>
    5b77:	48 8d 15 a5 73 00 00 	lea    rdx,[rip+0x73a5]        # cf23 <cljn_add>
    5b7e:	4c 89 f6             	mov    rsi,r14
    5b81:	48 89 df             	mov    rdi,rbx
    5b84:	ff d2                	call   rdx
    5b86:	48 89 04 24          	mov    QWORD PTR [rsp],rax
    5b8a:	e9 0b 00 00 00       	jmp    5b9a <mark-multiples+0x222>
    5b8f:	48 d1 e0             	shl    rax,1
    5b92:	48 83 c8 01          	or     rax,0x1
    5b96:	48 89 04 24          	mov    QWORD PTR [rsp],rax
    5b9a:	ba 06 00 00 00       	mov    edx,0x6
    5b9f:	4c 8d 0d da e4 00 02 	lea    r9,[rip+0x200e4da]        # 2014080 <gc_sp>
    5ba6:	4d 8b 11             	mov    r10,QWORD PTR [r9]
    5ba9:	4c 8d 1d d0 e4 00 00 	lea    r11,[rip+0xe4d0]        # 14080 <gc_stack>
    5bb0:	49 6b f2 08          	imul   rsi,r10,0x8
    5bb4:	4c 89 ef             	mov    rdi,r13
    5bb7:	49 89 3c 33          	mov    QWORD PTR [r11+rsi*1],rdi
    5bbb:	49 81 c2 01 00 00 00 	add    r10,0x1
    5bc2:	4d 89 11             	mov    QWORD PTR [r9],r10
    5bc5:	4c 8d 1d f2 66 00 00 	lea    r11,[rip+0x66f2]        # c2be <cljn_assoc>
    5bcc:	48 89 de             	mov    rsi,rbx
    5bcf:	41 ff d3             	call   r11
    5bd2:	48 89 c2             	mov    rdx,rax
    5bd5:	4c 8d 1d a4 e4 00 02 	lea    r11,[rip+0x200e4a4]        # 2014080 <gc_sp>
    5bdc:	49 83 03 ff          	add    QWORD PTR [r11],0xffffffffffffffff
    5be0:	48 8d 35 99 e4 00 02 	lea    rsi,[rip+0x200e499]        # 2014080 <gc_sp>
    5be7:	48 8b 3e             	mov    rdi,QWORD PTR [rsi]
    5bea:	48 8d 0d 8f e4 00 00 	lea    rcx,[rip+0xe48f]        # 14080 <gc_stack>
    5bf1:	48 6b c7 08          	imul   rax,rdi,0x8
    5bf5:	49 89 d1             	mov    r9,rdx
    5bf8:	4c 89 0c 01          	mov    QWORD PTR [rcx+rax*1],r9
    5bfc:	48 81 c7 01 00 00 00 	add    rdi,0x1
    5c03:	48 89 3e             	mov    QWORD PTR [rsi],rdi
    5c06:	48 8d 05 73 e4 00 02 	lea    rax,[rip+0x200e473]        # 2014080 <gc_sp>
    5c0d:	48 83 00 ff          	add    QWORD PTR [rax],0xffffffffffffffff
    5c11:	48 8d 0d 68 e4 00 02 	lea    rcx,[rip+0x200e468]        # 2014080 <gc_sp>
    5c18:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    5c1b:	4c 8d 05 5e e4 00 00 	lea    r8,[rip+0xe45e]        # 14080 <gc_stack>
    5c22:	4c 6b d2 08          	imul   r10,rdx,0x8
    5c26:	4f 89 0c 10          	mov    QWORD PTR [r8+r10*1],r9
    5c2a:	48 81 c2 01 00 00 00 	add    rdx,0x1
    5c31:	48 89 11             	mov    QWORD PTR [rcx],rdx
    5c34:	4d 89 e3             	mov    r11,r12
    5c37:	4d 8d 43 04          	lea    r8,[r11+0x4]
    5c3b:	4c 8d 15 3e e4 00 00 	lea    r10,[rip+0xe43e]        # 14080 <gc_stack>
    5c42:	4d 6b c0 08          	imul   r8,r8,0x8
    5c46:	4f 89 0c 02          	mov    QWORD PTR [r10+r8*1],r9
    5c4a:	4c 8d 15 2f e4 00 02 	lea    r10,[rip+0x200e42f]        # 2014080 <gc_sp>
    5c51:	49 83 02 ff          	add    QWORD PTR [r10],0xffffffffffffffff
    5c55:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    5c59:	4d 89 cd             	mov    r13,r9
    5c5c:	e9 7a fe ff ff       	jmp    5adb <mark-multiples+0x163>
    5c61:	4c 89 ef             	mov    rdi,r13
    5c64:	4c 8d 1d 15 e4 00 02 	lea    r11,[rip+0x200e415]        # 2014080 <gc_sp>
    5c6b:	49 8b 33             	mov    rsi,QWORD PTR [r11]
    5c6e:	48 8d 05 0b e4 00 00 	lea    rax,[rip+0xe40b]        # 14080 <gc_stack>
    5c75:	48 6b ce 08          	imul   rcx,rsi,0x8
    5c79:	48 89 3c 08          	mov    QWORD PTR [rax+rcx*1],rdi
    5c7d:	48 81 c6 01 00 00 00 	add    rsi,0x1
    5c84:	49 89 33             	mov    QWORD PTR [r11],rsi
    5c87:	49 89 fd             	mov    r13,rdi
    5c8a:	48 8d 05 b9 0f 00 00 	lea    rax,[rip+0xfb9]        # 6c4a <cljn_gc_leave>
    5c91:	4c 89 e7             	mov    rdi,r12
    5c94:	ff d0                	call   rax
    5c96:	4c 89 e8             	mov    rax,r13
    5c99:	48 8b 5c 24 10       	mov    rbx,QWORD PTR [rsp+0x10]
    5c9e:	4c 8b 64 24 18       	mov    r12,QWORD PTR [rsp+0x18]
    5ca3:	4c 8b 6c 24 20       	mov    r13,QWORD PTR [rsp+0x20]
    5ca8:	4c 8b 74 24 28       	mov    r14,QWORD PTR [rsp+0x28]
    5cad:	4c 8b 7c 24 30       	mov    r15,QWORD PTR [rsp+0x30]
    5cb2:	48 83 c4 40          	add    rsp,0x40
    5cb6:	48 89 ec             	mov    rsp,rbp
    5cb9:	5d                   	pop    rbp
    5cba:	c3                   	ret
	...
    5cc7:	c0 ff ff             	sar    bh,0xff
    5cca:	ff                   	(bad)
    5ccb:	ff                   	(bad)
    5ccc:	ff                   	(bad)
    5ccd:	ff                   	(bad)
    5cce:	ff                   	(bad)
    5ccf:	3f                   	(bad)
    5cd0:	0a 00                	or     al,BYTE PTR [rax]
    5cd2:	00 00                	add    BYTE PTR [rax],al
    5cd4:	00 00                	add    BYTE PTR [rax],al
	...

0000000000005cd8 <count-primes>:
    5cd8:	55                   	push   rbp
    5cd9:	48 89 e5             	mov    rbp,rsp
    5cdc:	48 83 ec 40          	sub    rsp,0x40
    5ce0:	48 89 5c 24 10       	mov    QWORD PTR [rsp+0x10],rbx
    5ce5:	4c 89 64 24 18       	mov    QWORD PTR [rsp+0x18],r12
    5cea:	4c 89 6c 24 20       	mov    QWORD PTR [rsp+0x20],r13
    5cef:	4c 89 74 24 28       	mov    QWORD PTR [rsp+0x28],r14
    5cf4:	4c 89 7c 24 30       	mov    QWORD PTR [rsp+0x30],r15
    5cf9:	49 89 f5             	mov    r13,rsi
    5cfc:	49 89 d7             	mov    r15,rdx
    5cff:	bf 05 00 00 00       	mov    edi,0x5
    5d04:	48 8d 15 89 0e 00 00 	lea    rdx,[rip+0xe89]        # 6b94 <cljn_gc_enter>
    5d0b:	ff d2                	call   rdx
    5d0d:	4d 89 eb             	mov    r11,r13
    5d10:	49 83 fb 01          	cmp    r11,0x1
    5d14:	0f 84 3b 00 00 00    	je     5d55 <count-primes+0x7d>
    5d1a:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    5d21:	4c 8d 0d 54 1b 00 00 	lea    r9,[rip+0x1b54]        # 787c <cljn_check_arity>
    5d28:	4c 89 df             	mov    rdi,r11
    5d2b:	41 ff d1             	call   r9
    5d2e:	b8 02 00 00 00       	mov    eax,0x2
    5d33:	48 8b 5c 24 10       	mov    rbx,QWORD PTR [rsp+0x10]
    5d38:	4c 8b 64 24 18       	mov    r12,QWORD PTR [rsp+0x18]
    5d3d:	4c 8b 6c 24 20       	mov    r13,QWORD PTR [rsp+0x20]
    5d42:	4c 8b 74 24 28       	mov    r14,QWORD PTR [rsp+0x28]
    5d47:	4c 8b 7c 24 30       	mov    r15,QWORD PTR [rsp+0x30]
    5d4c:	48 83 c4 40          	add    rsp,0x40
    5d50:	48 89 ec             	mov    rsp,rbp
    5d53:	5d                   	pop    rbp
    5d54:	c3                   	ret
    5d55:	4c 89 fa             	mov    rdx,r15
    5d58:	4c 8b 3a             	mov    r15,QWORD PTR [rdx]
    5d5b:	4c 8d 18             	lea    r11,[rax]
    5d5e:	49 89 c4             	mov    r12,rax
    5d61:	48 8d 35 18 e3 00 00 	lea    rsi,[rip+0xe318]        # 14080 <gc_stack>
    5d68:	4d 6b db 08          	imul   r11,r11,0x8
    5d6c:	4e 89 3c 1e          	mov    QWORD PTR [rsi+r11*1],r15
    5d70:	bb 05 00 00 00       	mov    ebx,0x5
    5d75:	48 8d 3d 04 e3 00 02 	lea    rdi,[rip+0x200e304]        # 2014080 <gc_sp>
    5d7c:	48 8b 07             	mov    rax,QWORD PTR [rdi]
    5d7f:	48 8d 0d fa e2 00 00 	lea    rcx,[rip+0xe2fa]        # 14080 <gc_stack>
    5d86:	48 6b d0 08          	imul   rdx,rax,0x8
    5d8a:	4c 89 3c 11          	mov    QWORD PTR [rcx+rdx*1],r15
    5d8e:	48 81 c0 01 00 00 00 	add    rax,0x1
    5d95:	48 89 07             	mov    QWORD PTR [rdi],rax
    5d98:	41 be 01 00 00 00    	mov    r14d,0x1
    5d9e:	48 8d 0d aa 1a 00 00 	lea    rcx,[rip+0x1aaa]        # 784f <cljn_argv>
    5da5:	4c 89 f7             	mov    rdi,r14
    5da8:	ff d1                	call   rcx
    5daa:	bf 02 00 00 00       	mov    edi,0x2
    5daf:	48 89 c2             	mov    rdx,rax
    5db2:	4c 89 f6             	mov    rsi,r14
    5db5:	e8 4e f8 ff ff       	call   5608 <initial-flags>
    5dba:	48 8d 0d bf e2 00 02 	lea    rcx,[rip+0x200e2bf]        # 2014080 <gc_sp>
    5dc1:	48 83 01 ff          	add    QWORD PTR [rcx],0xffffffffffffffff
    5dc5:	48 8d 15 b4 e2 00 02 	lea    rdx,[rip+0x200e2b4]        # 2014080 <gc_sp>
    5dcc:	4c 8b 02             	mov    r8,QWORD PTR [rdx]
    5dcf:	4c 8d 0d aa e2 00 00 	lea    r9,[rip+0xe2aa]        # 14080 <gc_stack>
    5dd6:	4d 6b d0 08          	imul   r10,r8,0x8
    5dda:	4b 89 04 11          	mov    QWORD PTR [r9+r10*1],rax
    5dde:	49 81 c0 01 00 00 00 	add    r8,0x1
    5de5:	4c 89 02             	mov    QWORD PTR [rdx],r8
    5de8:	4d 8d 4c 24 02       	lea    r9,[r12+0x2]
    5ded:	4c 8d 15 8c e2 00 00 	lea    r10,[rip+0xe28c]        # 14080 <gc_stack>
    5df4:	4d 6b c9 08          	imul   r9,r9,0x8
    5df8:	4b 89 04 0a          	mov    QWORD PTR [r10+r9*1],rax
    5dfc:	48 89 44 24 08       	mov    QWORD PTR [rsp+0x8],rax
    5e01:	4c 8d 15 78 e2 00 02 	lea    r10,[rip+0x200e278]        # 2014080 <gc_sp>
    5e08:	49 83 02 ff          	add    QWORD PTR [r10],0xffffffffffffffff
    5e0c:	48 89 de             	mov    rsi,rbx
    5e0f:	48 23 f3             	and    rsi,rbx
    5e12:	48 f7 c6 01 00 00 00 	test   rsi,0x1
    5e19:	0f 84 43 00 00 00    	je     5e62 <count-primes+0x18a>
    5e1f:	48 89 d8             	mov    rax,rbx
    5e22:	48 d1 f8             	sar    rax,1
    5e25:	48 89 d9             	mov    rcx,rbx
    5e28:	48 d1 f9             	sar    rcx,1
    5e2b:	48 89 c7             	mov    rdi,rax
    5e2e:	48 0f af f9          	imul   rdi,rcx
    5e32:	48 f7 e9             	imul   rcx
    5e35:	48 89 f8             	mov    rax,rdi
    5e38:	48 c1 f8 3f          	sar    rax,0x3f
    5e3c:	48 3b d0             	cmp    rdx,rax
    5e3f:	0f 94 c1             	sete   cl
    5e42:	48 3b 3d ef 05 00 00 	cmp    rdi,QWORD PTR [rip+0x5ef]        # 6438 <count-primes+0x760>
    5e49:	0f 9d c0             	setge  al
    5e4c:	48 3b 3d ed 05 00 00 	cmp    rdi,QWORD PTR [rip+0x5ed]        # 6440 <count-primes+0x768>
    5e53:	41 0f 9e c0          	setle  r8b
    5e57:	41 23 c0             	and    eax,r8d
    5e5a:	84 c8                	test   al,cl
    5e5c:	0f 85 18 00 00 00    	jne    5e7a <count-primes+0x1a2>
    5e62:	4c 8d 1d 1e 72 00 00 	lea    r11,[rip+0x721e]        # d087 <cljn_mul>
    5e69:	48 89 de             	mov    rsi,rbx
    5e6c:	48 89 df             	mov    rdi,rbx
    5e6f:	41 ff d3             	call   r11
    5e72:	48 89 c7             	mov    rdi,rax
    5e75:	e9 07 00 00 00       	jmp    5e81 <count-primes+0x1a9>
    5e7a:	48 d1 e7             	shl    rdi,1
    5e7d:	48 83 cf 01          	or     rdi,0x1
    5e81:	48 89 f8             	mov    rax,rdi
    5e84:	49 23 c7             	and    rax,r15
    5e87:	48 a9 01 00 00 00    	test   rax,0x1
    5e8d:	0f 85 11 00 00 00    	jne    5ea4 <count-primes+0x1cc>
    5e93:	48 8d 0d d1 75 00 00 	lea    rcx,[rip+0x75d1]        # d46b <cljn_gt>
    5e9a:	4c 89 fe             	mov    rsi,r15
    5e9d:	ff d1                	call   rcx
    5e9f:	e9 19 00 00 00       	jmp    5ebd <count-primes+0x1e5>
    5ea4:	48 d1 ff             	sar    rdi,1
    5ea7:	4c 89 fa             	mov    rdx,r15
    5eaa:	48 d1 fa             	sar    rdx,1
    5ead:	b8 06 00 00 00       	mov    eax,0x6
    5eb2:	48 3b fa             	cmp    rdi,rdx
    5eb5:	48 0f 4f 05 8b 05 00 	cmovg  rax,QWORD PTR [rip+0x58b]        # 6448 <count-primes+0x770>
    5ebc:	00 
    5ebd:	48 83 f8 06          	cmp    rax,0x6
    5ec1:	41 0f 95 c2          	setne  r10b
    5ec5:	48 83 f8 02          	cmp    rax,0x2
    5ec9:	41 0f 95 c3          	setne  r11b
    5ecd:	45 84 d3             	test   r11b,r10b
    5ed0:	0f 85 bc 02 00 00    	jne    6192 <count-primes+0x4ba>
    5ed6:	48 f7 c3 01 00 00 00 	test   rbx,0x1
    5edd:	0f 84 2c 00 00 00    	je     5f0f <count-primes+0x237>
    5ee3:	48 89 da             	mov    rdx,rbx
    5ee6:	48 d1 fa             	sar    rdx,1
    5ee9:	48 8d 42 01          	lea    rax,[rdx+0x1]
    5eed:	48 3b 05 44 05 00 00 	cmp    rax,QWORD PTR [rip+0x544]        # 6438 <count-primes+0x760>
    5ef4:	49 89 c5             	mov    r13,rax
    5ef7:	41 0f 9d c0          	setge  r8b
    5efb:	48 3b 05 3e 05 00 00 	cmp    rax,QWORD PTR [rip+0x53e]        # 6440 <count-primes+0x768>
    5f02:	41 0f 9e c1          	setle  r9b
    5f06:	45 84 c1             	test   r9b,r8b
    5f09:	0f 85 19 00 00 00    	jne    5f28 <count-primes+0x250>
    5f0f:	48 8d 35 66 73 00 00 	lea    rsi,[rip+0x7366]        # d27c <cljn_inc>
    5f16:	48 89 df             	mov    rdi,rbx
    5f19:	ff d6                	call   rsi
    5f1b:	49 89 c5             	mov    r13,rax
    5f1e:	48 8b 44 24 08       	mov    rax,QWORD PTR [rsp+0x8]
    5f23:	e9 15 00 00 00       	jmp    5f3d <count-primes+0x265>
    5f28:	4c 89 e8             	mov    rax,r13
    5f2b:	48 d1 e0             	shl    rax,1
    5f2e:	49 89 c5             	mov    r13,rax
    5f31:	48 83 c8 01          	or     rax,0x1
    5f35:	49 89 c5             	mov    r13,rax
    5f38:	48 8b 44 24 08       	mov    rax,QWORD PTR [rsp+0x8]
    5f3d:	48 a9 07 00 00 00    	test   rax,0x7
    5f43:	40 0f 94 c7          	sete   dil
    5f47:	48 8b 44 24 08       	mov    rax,QWORD PTR [rsp+0x8]
    5f4c:	48 8b 4c 24 08       	mov    rcx,QWORD PTR [rsp+0x8]
    5f51:	48 85 c8             	test   rax,rcx
    5f54:	0f 95 c2             	setne  dl
    5f57:	23 fa                	and    edi,edx
    5f59:	48 f7 c3 01 00 00 00 	test   rbx,0x1
    5f60:	41 0f 95 c0          	setne  r8b
    5f64:	41 84 f8             	test   r8b,dil
    5f67:	0f 84 b0 00 00 00    	je     601d <count-primes+0x345>
    5f6d:	48 8b 44 24 08       	mov    rax,QWORD PTR [rsp+0x8]
    5f72:	4c 0f b6 18          	movzx  r11,BYTE PTR [rax]
    5f76:	41 80 fb 05          	cmp    r11b,0x5
    5f7a:	0f 85 9d 00 00 00    	jne    601d <count-primes+0x345>
    5f80:	48 89 da             	mov    rdx,rbx
    5f83:	48 d1 fa             	sar    rdx,1
    5f86:	48 8b 44 24 08       	mov    rax,QWORD PTR [rsp+0x8]
    5f8b:	4c 8b 48 10          	mov    r9,QWORD PTR [rax+0x10]
    5f8f:	48 89 d6             	mov    rsi,rdx
    5f92:	48 c1 ee 3f          	shr    rsi,0x3f
    5f96:	49 3b d1             	cmp    rdx,r9
    5f99:	0f 9d c1             	setge  cl
    5f9c:	0b f1                	or     esi,ecx
    5f9e:	40 84 f6             	test   sil,sil
    5fa1:	0f 85 76 00 00 00    	jne    601d <count-primes+0x345>
    5fa7:	48 8b 44 24 08       	mov    rax,QWORD PTR [rsp+0x8]
    5fac:	4c 8b 50 30          	mov    r10,QWORD PTR [rax+0x30]
    5fb0:	4d 2b ca             	sub    r9,r10
    5fb3:	49 3b d1             	cmp    rdx,r9
    5fb6:	0f 8d 47 00 00 00    	jge    6003 <count-primes+0x32b>
    5fbc:	48 8b 44 24 08       	mov    rax,QWORD PTR [rsp+0x8]
    5fc1:	4c 8b 50 20          	mov    r10,QWORD PTR [rax+0x20]
    5fc5:	48 8b 48 18          	mov    rcx,QWORD PTR [rax+0x18]
    5fc9:	48 85 c9             	test   rcx,rcx
    5fcc:	0f 8f 12 00 00 00    	jg     5fe4 <count-primes+0x30c>
    5fd2:	48 83 e2 1f          	and    rdx,0x1f
    5fd6:	48 6b c2 08          	imul   rax,rdx,0x8
    5fda:	49 8b 7c 02 18       	mov    rdi,QWORD PTR [r10+rax*1+0x18]
    5fdf:	e9 4e 00 00 00       	jmp    6032 <count-primes+0x35a>
    5fe4:	48 89 d0             	mov    rax,rdx
    5fe7:	48 d3 f8             	sar    rax,cl
    5fea:	48 83 e0 1f          	and    rax,0x1f
    5fee:	4c 6b c0 08          	imul   r8,rax,0x8
    5ff2:	4f 8b 54 02 18       	mov    r10,QWORD PTR [r10+r8*1+0x18]
    5ff7:	48 81 c1 fb ff ff ff 	add    rcx,0xfffffffffffffffb
    5ffe:	e9 c6 ff ff ff       	jmp    5fc9 <count-primes+0x2f1>
    6003:	48 8b 44 24 08       	mov    rax,QWORD PTR [rsp+0x8]
    6008:	4c 8b 40 28          	mov    r8,QWORD PTR [rax+0x28]
    600c:	49 2b d1             	sub    rdx,r9
    600f:	48 6b d2 08          	imul   rdx,rdx,0x8
    6013:	49 8b 7c 10 18       	mov    rdi,QWORD PTR [r8+rdx*1+0x18]
    6018:	e9 15 00 00 00       	jmp    6032 <count-primes+0x35a>
    601d:	4c 8d 05 5d 65 00 00 	lea    r8,[rip+0x655d]        # c581 <cljn_nth>
    6024:	48 89 de             	mov    rsi,rbx
    6027:	48 8b 7c 24 08       	mov    rdi,QWORD PTR [rsp+0x8]
    602c:	41 ff d0             	call   r8
    602f:	48 89 c7             	mov    rdi,rax
    6032:	4c 8d 0d 47 e0 00 02 	lea    r9,[rip+0x200e047]        # 2014080 <gc_sp>
    6039:	4d 8b 11             	mov    r10,QWORD PTR [r9]
    603c:	4c 8d 1d 3d e0 00 00 	lea    r11,[rip+0xe03d]        # 14080 <gc_stack>
    6043:	49 6b f2 08          	imul   rsi,r10,0x8
    6047:	49 89 3c 33          	mov    QWORD PTR [r11+rsi*1],rdi
    604b:	49 81 c2 01 00 00 00 	add    r10,0x1
    6052:	4d 89 11             	mov    QWORD PTR [r9],r10
    6055:	4c 8d 1d 23 7c 00 00 	lea    r11,[rip+0x7c23]        # dc7f <cljn_truthy>
    605c:	41 ff d3             	call   r11
    605f:	4c 8d 1d 1a e0 00 02 	lea    r11,[rip+0x200e01a]        # 2014080 <gc_sp>
    6066:	49 83 03 ff          	add    QWORD PTR [r11],0xffffffffffffffff
    606a:	85 c0                	test   eax,eax
    606c:	0f 85 2d 00 00 00    	jne    609f <count-primes+0x3c7>
    6072:	48 8d 0d 07 e0 00 02 	lea    rcx,[rip+0x200e007]        # 2014080 <gc_sp>
    6079:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    607c:	48 8d 05 fd df 00 00 	lea    rax,[rip+0xdffd]        # 14080 <gc_stack>
    6083:	4c 6b c2 08          	imul   r8,rdx,0x8
    6087:	4c 8b 4c 24 08       	mov    r9,QWORD PTR [rsp+0x8]
    608c:	4e 89 0c 00          	mov    QWORD PTR [rax+r8*1],r9
    6090:	48 81 c2 01 00 00 00 	add    rdx,0x1
    6097:	48 89 11             	mov    QWORD PTR [rcx],rdx
    609a:	e9 c7 00 00 00       	jmp    6166 <count-primes+0x48e>
    609f:	4c 8d 05 da df 00 02 	lea    r8,[rip+0x200dfda]        # 2014080 <gc_sp>
    60a6:	4d 8b 08             	mov    r9,QWORD PTR [r8]
    60a9:	4c 8d 15 d0 df 00 00 	lea    r10,[rip+0xdfd0]        # 14080 <gc_stack>
    60b0:	4d 6b d9 08          	imul   r11,r9,0x8
    60b4:	48 8b 44 24 08       	mov    rax,QWORD PTR [rsp+0x8]
    60b9:	4b 89 04 1a          	mov    QWORD PTR [r10+r11*1],rax
    60bd:	49 81 c1 01 00 00 00 	add    r9,0x1
    60c4:	4d 89 08             	mov    QWORD PTR [r8],r9
    60c7:	4c 8d 15 b2 df 00 02 	lea    r10,[rip+0x200dfb2]        # 2014080 <gc_sp>
    60ce:	4d 8b 1a             	mov    r11,QWORD PTR [r10]
    60d1:	48 8d 35 a8 df 00 00 	lea    rsi,[rip+0xdfa8]        # 14080 <gc_stack>
    60d8:	49 6b fb 08          	imul   rdi,r11,0x8
    60dc:	48 89 1c 3e          	mov    QWORD PTR [rsi+rdi*1],rbx
    60e0:	49 81 c3 01 00 00 00 	add    r11,0x1
    60e7:	4d 89 1a             	mov    QWORD PTR [r10],r11
    60ea:	48 8d 35 8f df 00 02 	lea    rsi,[rip+0x200df8f]        # 2014080 <gc_sp>
    60f1:	48 8b 3e             	mov    rdi,QWORD PTR [rsi]
    60f4:	48 8d 05 85 df 00 00 	lea    rax,[rip+0xdf85]        # 14080 <gc_stack>
    60fb:	48 6b cf 08          	imul   rcx,rdi,0x8
    60ff:	4c 89 3c 08          	mov    QWORD PTR [rax+rcx*1],r15
    6103:	48 81 c7 01 00 00 00 	add    rdi,0x1
    610a:	48 89 3e             	mov    QWORD PTR [rsi],rdi
    610d:	bb 03 00 00 00       	mov    ebx,0x3
    6112:	48 8d 05 36 17 00 00 	lea    rax,[rip+0x1736]        # 784f <cljn_argv>
    6119:	48 89 df             	mov    rdi,rbx
    611c:	ff d0                	call   rax
    611e:	bf 02 00 00 00       	mov    edi,0x2
    6123:	48 89 c2             	mov    rdx,rax
    6126:	48 89 de             	mov    rsi,rbx
    6129:	e8 4a f8 ff ff       	call   5978 <mark-multiples>
    612e:	48 89 44 24 08       	mov    QWORD PTR [rsp+0x8],rax
    6133:	48 8d 05 46 df 00 02 	lea    rax,[rip+0x200df46]        # 2014080 <gc_sp>
    613a:	48 83 00 fd          	add    QWORD PTR [rax],0xfffffffffffffffd
    613e:	48 8d 0d 3b df 00 02 	lea    rcx,[rip+0x200df3b]        # 2014080 <gc_sp>
    6145:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    6148:	4c 8d 05 31 df 00 00 	lea    r8,[rip+0xdf31]        # 14080 <gc_stack>
    614f:	4c 6b ca 08          	imul   r9,rdx,0x8
    6153:	48 8b 44 24 08       	mov    rax,QWORD PTR [rsp+0x8]
    6158:	4b 89 04 08          	mov    QWORD PTR [r8+r9*1],rax
    615c:	48 81 c2 01 00 00 00 	add    rdx,0x1
    6163:	48 89 11             	mov    QWORD PTR [rcx],rdx
    6166:	4d 8d 4c 24 02       	lea    r9,[r12+0x2]
    616b:	4c 8d 15 0e df 00 00 	lea    r10,[rip+0xdf0e]        # 14080 <gc_stack>
    6172:	4d 6b c9 08          	imul   r9,r9,0x8
    6176:	48 8b 44 24 08       	mov    rax,QWORD PTR [rsp+0x8]
    617b:	4b 89 04 0a          	mov    QWORD PTR [r10+r9*1],rax
    617f:	4c 8d 15 fa de 00 02 	lea    r10,[rip+0x200defa]        # 2014080 <gc_sp>
    6186:	49 83 02 ff          	add    QWORD PTR [r10],0xffffffffffffffff
    618a:	4c 89 eb             	mov    rbx,r13
    618d:	e9 7a fc ff ff       	jmp    5e0c <count-primes+0x134>
    6192:	be 05 00 00 00       	mov    esi,0x5
    6197:	48 89 34 24          	mov    QWORD PTR [rsp],rsi
    619b:	bf 01 00 00 00       	mov    edi,0x1
    61a0:	49 89 fe             	mov    r14,rdi
    61a3:	48 89 f7             	mov    rdi,rsi
    61a6:	49 23 ff             	and    rdi,r15
    61a9:	48 89 34 24          	mov    QWORD PTR [rsp],rsi
    61ad:	48 f7 c7 01 00 00 00 	test   rdi,0x1
    61b4:	0f 85 15 00 00 00    	jne    61cf <count-primes+0x4f7>
    61ba:	48 8d 0d aa 72 00 00 	lea    rcx,[rip+0x72aa]        # d46b <cljn_gt>
    61c1:	4c 89 fe             	mov    rsi,r15
    61c4:	48 8b 3c 24          	mov    rdi,QWORD PTR [rsp]
    61c8:	ff d1                	call   rcx
    61ca:	e9 20 00 00 00       	jmp    61ef <count-primes+0x517>
    61cf:	48 8b 34 24          	mov    rsi,QWORD PTR [rsp]
    61d3:	48 89 f2             	mov    rdx,rsi
    61d6:	48 d1 fa             	sar    rdx,1
    61d9:	4d 89 f8             	mov    r8,r15
    61dc:	49 d1 f8             	sar    r8,1
    61df:	b8 06 00 00 00       	mov    eax,0x6
    61e4:	49 3b d0             	cmp    rdx,r8
    61e7:	48 0f 4f 05 59 02 00 	cmovg  rax,QWORD PTR [rip+0x259]        # 6448 <count-primes+0x770>
    61ee:	00 
    61ef:	48 83 f8 06          	cmp    rax,0x6
    61f3:	41 0f 95 c2          	setne  r10b
    61f7:	48 83 f8 02          	cmp    rax,0x2
    61fb:	41 0f 95 c3          	setne  r11b
    61ff:	45 84 d3             	test   r11b,r10b
    6202:	0f 85 f6 01 00 00    	jne    63fe <count-primes+0x726>
    6208:	48 8b 34 24          	mov    rsi,QWORD PTR [rsp]
    620c:	48 f7 c6 01 00 00 00 	test   rsi,0x1
    6213:	0f 84 30 00 00 00    	je     6249 <count-primes+0x571>
    6219:	48 8b 34 24          	mov    rsi,QWORD PTR [rsp]
    621d:	48 89 f2             	mov    rdx,rsi
    6220:	48 d1 fa             	sar    rdx,1
    6223:	48 8d 42 01          	lea    rax,[rdx+0x1]
    6227:	48 3b 05 0a 02 00 00 	cmp    rax,QWORD PTR [rip+0x20a]        # 6438 <count-primes+0x760>
    622e:	49 89 c5             	mov    r13,rax
    6231:	41 0f 9d c0          	setge  r8b
    6235:	48 3b 05 04 02 00 00 	cmp    rax,QWORD PTR [rip+0x204]        # 6440 <count-primes+0x768>
    623c:	41 0f 9e c1          	setle  r9b
    6240:	45 84 c1             	test   r9b,r8b
    6243:	0f 85 1a 00 00 00    	jne    6263 <count-primes+0x58b>
    6249:	48 8d 35 2c 70 00 00 	lea    rsi,[rip+0x702c]        # d27c <cljn_inc>
    6250:	48 8b 3c 24          	mov    rdi,QWORD PTR [rsp]
    6254:	ff d6                	call   rsi
    6256:	49 89 c5             	mov    r13,rax
    6259:	48 8b 5c 24 08       	mov    rbx,QWORD PTR [rsp+0x8]
    625e:	e9 15 00 00 00       	jmp    6278 <count-primes+0x5a0>
    6263:	4c 89 e8             	mov    rax,r13
    6266:	48 d1 e0             	shl    rax,1
    6269:	49 89 c5             	mov    r13,rax
    626c:	48 83 c8 01          	or     rax,0x1
    6270:	49 89 c5             	mov    r13,rax
    6273:	48 8b 5c 24 08       	mov    rbx,QWORD PTR [rsp+0x8]
    6278:	48 f7 c3 07 00 00 00 	test   rbx,0x7
    627f:	40 0f 94 c7          	sete   dil
    6283:	48 85 db             	test   rbx,rbx
    6286:	0f 95 c2             	setne  dl
    6289:	23 fa                	and    edi,edx
    628b:	48 8b 34 24          	mov    rsi,QWORD PTR [rsp]
    628f:	48 f7 c6 01 00 00 00 	test   rsi,0x1
    6296:	41 0f 95 c0          	setne  r8b
    629a:	41 84 f8             	test   r8b,dil
    629d:	0f 84 9b 00 00 00    	je     633e <count-primes+0x666>
    62a3:	4c 0f b6 1b          	movzx  r11,BYTE PTR [rbx]
    62a7:	41 80 fb 05          	cmp    r11b,0x5
    62ab:	0f 85 8d 00 00 00    	jne    633e <count-primes+0x666>
    62b1:	48 8b 34 24          	mov    rsi,QWORD PTR [rsp]
    62b5:	48 89 f2             	mov    rdx,rsi
    62b8:	48 d1 fa             	sar    rdx,1
    62bb:	4c 8b 4b 10          	mov    r9,QWORD PTR [rbx+0x10]
    62bf:	48 89 d6             	mov    rsi,rdx
    62c2:	48 c1 ee 3f          	shr    rsi,0x3f
    62c6:	49 3b d1             	cmp    rdx,r9
    62c9:	0f 9d c1             	setge  cl
    62cc:	0b f1                	or     esi,ecx
    62ce:	40 84 f6             	test   sil,sil
    62d1:	0f 85 67 00 00 00    	jne    633e <count-primes+0x666>
    62d7:	4c 8b 53 30          	mov    r10,QWORD PTR [rbx+0x30]
    62db:	4d 2b ca             	sub    r9,r10
    62de:	49 3b d1             	cmp    rdx,r9
    62e1:	0f 8d 42 00 00 00    	jge    6329 <count-primes+0x651>
    62e7:	48 8b 73 20          	mov    rsi,QWORD PTR [rbx+0x20]
    62eb:	48 8b 4b 18          	mov    rcx,QWORD PTR [rbx+0x18]
    62ef:	48 85 c9             	test   rcx,rcx
    62f2:	0f 8f 12 00 00 00    	jg     630a <count-primes+0x632>
    62f8:	48 83 e2 1f          	and    rdx,0x1f
    62fc:	48 6b c2 08          	imul   rax,rdx,0x8
    6300:	48 8b 7c 06 18       	mov    rdi,QWORD PTR [rsi+rax*1+0x18]
    6305:	e9 48 00 00 00       	jmp    6352 <count-primes+0x67a>
    630a:	48 89 d0             	mov    rax,rdx
    630d:	48 d3 f8             	sar    rax,cl
    6310:	48 83 e0 1f          	and    rax,0x1f
    6314:	4c 6b c0 08          	imul   r8,rax,0x8
    6318:	4a 8b 74 06 18       	mov    rsi,QWORD PTR [rsi+r8*1+0x18]
    631d:	48 81 c1 fb ff ff ff 	add    rcx,0xfffffffffffffffb
    6324:	e9 c6 ff ff ff       	jmp    62ef <count-primes+0x617>
    6329:	4c 8b 43 28          	mov    r8,QWORD PTR [rbx+0x28]
    632d:	49 2b d1             	sub    rdx,r9
    6330:	48 6b d2 08          	imul   rdx,rdx,0x8
    6334:	49 8b 7c 10 18       	mov    rdi,QWORD PTR [r8+rdx*1+0x18]
    6339:	e9 14 00 00 00       	jmp    6352 <count-primes+0x67a>
    633e:	4c 8d 05 3c 62 00 00 	lea    r8,[rip+0x623c]        # c581 <cljn_nth>
    6345:	48 8b 34 24          	mov    rsi,QWORD PTR [rsp]
    6349:	48 89 df             	mov    rdi,rbx
    634c:	41 ff d0             	call   r8
    634f:	48 89 c7             	mov    rdi,rax
    6352:	4c 8d 0d 27 dd 00 02 	lea    r9,[rip+0x200dd27]        # 2014080 <gc_sp>
    6359:	4d 8b 11             	mov    r10,QWORD PTR [r9]
    635c:	4c 8d 1d 1d dd 00 00 	lea    r11,[rip+0xdd1d]        # 14080 <gc_stack>
    6363:	49 6b f2 08          	imul   rsi,r10,0x8
    6367:	49 89 3c 33          	mov    QWORD PTR [r11+rsi*1],rdi
    636b:	49 81 c2 01 00 00 00 	add    r10,0x1
    6372:	4d 89 11             	mov    QWORD PTR [r9],r10
    6375:	4c 8d 1d 03 79 00 00 	lea    r11,[rip+0x7903]        # dc7f <cljn_truthy>
    637c:	41 ff d3             	call   r11
    637f:	4c 8d 1d fa dc 00 02 	lea    r11,[rip+0x200dcfa]        # 2014080 <gc_sp>
    6386:	49 83 03 ff          	add    QWORD PTR [r11],0xffffffffffffffff
    638a:	85 c0                	test   eax,eax
    638c:	0f 85 08 00 00 00    	jne    639a <count-primes+0x6c2>
    6392:	4c 89 f7             	mov    rdi,r14
    6395:	e9 54 00 00 00       	jmp    63ee <count-primes+0x716>
    639a:	4c 89 f7             	mov    rdi,r14
    639d:	48 f7 c7 01 00 00 00 	test   rdi,0x1
    63a4:	0f 84 29 00 00 00    	je     63d3 <count-primes+0x6fb>
    63aa:	49 89 f9             	mov    r9,rdi
    63ad:	49 d1 f9             	sar    r9,1
    63b0:	49 8d 71 01          	lea    rsi,[r9+0x1]
    63b4:	48 3b 35 7d 00 00 00 	cmp    rsi,QWORD PTR [rip+0x7d]        # 6438 <count-primes+0x760>
    63bb:	41 0f 9d c2          	setge  r10b
    63bf:	48 3b 35 7a 00 00 00 	cmp    rsi,QWORD PTR [rip+0x7a]        # 6440 <count-primes+0x768>
    63c6:	41 0f 9e c3          	setle  r11b
    63ca:	45 84 d3             	test   r11b,r10b
    63cd:	0f 85 11 00 00 00    	jne    63e4 <count-primes+0x70c>
    63d3:	48 8d 05 a2 6e 00 00 	lea    rax,[rip+0x6ea2]        # d27c <cljn_inc>
    63da:	ff d0                	call   rax
    63dc:	48 89 c7             	mov    rdi,rax
    63df:	e9 0a 00 00 00       	jmp    63ee <count-primes+0x716>
    63e4:	48 d1 e6             	shl    rsi,1
    63e7:	48 83 ce 01          	or     rsi,0x1
    63eb:	48 89 f7             	mov    rdi,rsi
    63ee:	4c 89 ee             	mov    rsi,r13
    63f1:	49 89 fe             	mov    r14,rdi
    63f4:	48 89 5c 24 08       	mov    QWORD PTR [rsp+0x8],rbx
    63f9:	e9 a5 fd ff ff       	jmp    61a3 <count-primes+0x4cb>
    63fe:	4c 89 f7             	mov    rdi,r14
    6401:	49 89 fd             	mov    r13,rdi
    6404:	4c 8d 1d 3f 08 00 00 	lea    r11,[rip+0x83f]        # 6c4a <cljn_gc_leave>
    640b:	4c 89 e7             	mov    rdi,r12
    640e:	41 ff d3             	call   r11
    6411:	4c 89 e8             	mov    rax,r13
    6414:	48 8b 5c 24 10       	mov    rbx,QWORD PTR [rsp+0x10]
    6419:	4c 8b 64 24 18       	mov    r12,QWORD PTR [rsp+0x18]
    641e:	4c 8b 6c 24 20       	mov    r13,QWORD PTR [rsp+0x20]
    6423:	4c 8b 74 24 28       	mov    r14,QWORD PTR [rsp+0x28]
    6428:	4c 8b 7c 24 30       	mov    r15,QWORD PTR [rsp+0x30]
    642d:	48 83 c4 40          	add    rsp,0x40
    6431:	48 89 ec             	mov    rsp,rbp
    6434:	5d                   	pop    rbp
    6435:	c3                   	ret
	...
    643e:	00 c0                	add    al,al
    6440:	ff                   	(bad)
    6441:	ff                   	(bad)
    6442:	ff                   	(bad)
    6443:	ff                   	(bad)
    6444:	ff                   	(bad)
    6445:	ff                   	(bad)
    6446:	ff                   	(bad)
    6447:	3f                   	(bad)
    6448:	0a 00                	or     al,BYTE PTR [rax]
    644a:	00 00                	add    BYTE PTR [rax],al
    644c:	00 00                	add    BYTE PTR [rax],al
	...

0000000000006450 <benchmark>:
    6450:	55                   	push   rbp
    6451:	48 89 e5             	mov    rbp,rsp
    6454:	48 83 ec 40          	sub    rsp,0x40
    6458:	48 89 5c 24 10       	mov    QWORD PTR [rsp+0x10],rbx
    645d:	4c 89 64 24 18       	mov    QWORD PTR [rsp+0x18],r12
    6462:	4c 89 6c 24 20       	mov    QWORD PTR [rsp+0x20],r13
    6467:	4c 89 74 24 28       	mov    QWORD PTR [rsp+0x28],r14
    646c:	4c 89 7c 24 30       	mov    QWORD PTR [rsp+0x30],r15
    6471:	49 89 f5             	mov    r13,rsi
    6474:	49 89 d7             	mov    r15,rdx
    6477:	bf 03 00 00 00       	mov    edi,0x3
    647c:	48 8d 05 11 07 00 00 	lea    rax,[rip+0x711]        # 6b94 <cljn_gc_enter>
    6483:	ff d0                	call   rax
    6485:	4c 89 ef             	mov    rdi,r13
    6488:	48 83 ff 01          	cmp    rdi,0x1
    648c:	0f 84 37 00 00 00    	je     64c9 <benchmark+0x79>
    6492:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    6499:	48 8d 0d dc 13 00 00 	lea    rcx,[rip+0x13dc]        # 787c <cljn_check_arity>
    64a0:	ff d1                	call   rcx
    64a2:	b8 02 00 00 00       	mov    eax,0x2
    64a7:	48 8b 5c 24 10       	mov    rbx,QWORD PTR [rsp+0x10]
    64ac:	4c 8b 64 24 18       	mov    r12,QWORD PTR [rsp+0x18]
    64b1:	4c 8b 6c 24 20       	mov    r13,QWORD PTR [rsp+0x20]
    64b6:	4c 8b 74 24 28       	mov    r14,QWORD PTR [rsp+0x28]
    64bb:	4c 8b 7c 24 30       	mov    r15,QWORD PTR [rsp+0x30]
    64c0:	48 83 c4 40          	add    rsp,0x40
    64c4:	48 89 ec             	mov    rsp,rbp
    64c7:	5d                   	pop    rbp
    64c8:	c3                   	ret
    64c9:	4c 89 fa             	mov    rdx,r15
    64cc:	48 8b 3a             	mov    rdi,QWORD PTR [rdx]
    64cf:	4c 8d 00             	lea    r8,[rax]
    64d2:	4c 8d 0d a7 db 00 00 	lea    r9,[rip+0xdba7]        # 14080 <gc_stack>
    64d9:	4d 6b c0 08          	imul   r8,r8,0x8
    64dd:	4b 89 3c 01          	mov    QWORD PTR [r9+r8*1],rdi
    64e1:	4c 8d 50 01          	lea    r10,[rax+0x1]
    64e5:	49 89 c6             	mov    r14,rax
    64e8:	4c 8d 1d 91 db 00 00 	lea    r11,[rip+0xdb91]        # 14080 <gc_stack>
    64ef:	4d 6b d2 08          	imul   r10,r10,0x8
    64f3:	4b 89 3c 13          	mov    QWORD PTR [r11+r10*1],rdi
    64f7:	48 89 fb             	mov    rbx,rdi
    64fa:	bf 01 00 00 00       	mov    edi,0x1
    64ff:	48 89 3c 24          	mov    QWORD PTR [rsp],rdi
    6503:	be 01 00 00 00       	mov    esi,0x1
    6508:	48 89 df             	mov    rdi,rbx
    650b:	48 89 f8             	mov    rax,rdi
    650e:	48 83 e0 01          	and    rax,0x1
    6512:	48 a9 01 00 00 00    	test   rax,0x1
    6518:	0f 85 11 00 00 00    	jne    652f <benchmark+0xdf>
    651e:	48 8d 05 46 6f 00 00 	lea    rax,[rip+0x6f46]        # d46b <cljn_gt>
    6525:	48 89 df             	mov    rdi,rbx
    6528:	ff d0                	call   rax
    652a:	e9 1c 00 00 00       	jmp    654b <benchmark+0xfb>
    652f:	48 89 df             	mov    rdi,rbx
    6532:	48 89 f9             	mov    rcx,rdi
    6535:	48 d1 f9             	sar    rcx,1
    6538:	48 d1 fe             	sar    rsi,1
    653b:	b8 06 00 00 00       	mov    eax,0x6
    6540:	48 3b ce             	cmp    rcx,rsi
    6543:	48 0f 4f 05 6d 04 00 	cmovg  rax,QWORD PTR [rip+0x46d]        # 69b8 <benchmark+0x568>
    654a:	00 
    654b:	48 83 f8 06          	cmp    rax,0x6
    654f:	41 0f 95 c1          	setne  r9b
    6553:	48 83 f8 02          	cmp    rax,0x2
    6557:	41 0f 95 c2          	setne  r10b
    655b:	45 84 ca             	test   r10b,r9b
    655e:	0f 85 35 00 00 00    	jne    6599 <benchmark+0x149>
    6564:	4c 8b 24 24          	mov    r12,QWORD PTR [rsp]
    6568:	48 8d 05 db 06 00 00 	lea    rax,[rip+0x6db]        # 6c4a <cljn_gc_leave>
    656f:	4c 89 f7             	mov    rdi,r14
    6572:	ff d0                	call   rax
    6574:	4c 89 e0             	mov    rax,r12
    6577:	48 8b 5c 24 10       	mov    rbx,QWORD PTR [rsp+0x10]
    657c:	4c 8b 64 24 18       	mov    r12,QWORD PTR [rsp+0x18]
    6581:	4c 8b 6c 24 20       	mov    r13,QWORD PTR [rsp+0x20]
    6586:	4c 8b 74 24 28       	mov    r14,QWORD PTR [rsp+0x28]
    658b:	4c 8b 7c 24 30       	mov    r15,QWORD PTR [rsp+0x30]
    6590:	48 83 c4 40          	add    rsp,0x40
    6594:	48 89 ec             	mov    rsp,rbp
    6597:	5d                   	pop    rbp
    6598:	c3                   	ret
    6599:	48 89 df             	mov    rdi,rbx
    659c:	48 f7 c7 01 00 00 00 	test   rdi,0x1
    65a3:	0f 84 2c 00 00 00    	je     65d5 <benchmark+0x185>
    65a9:	48 89 df             	mov    rdi,rbx
    65ac:	49 89 f9             	mov    r9,rdi
    65af:	49 d1 f9             	sar    r9,1
    65b2:	49 8d 41 ff          	lea    rax,[r9-0x1]
    65b6:	48 3b 05 03 04 00 00 	cmp    rax,QWORD PTR [rip+0x403]        # 69c0 <benchmark+0x570>
    65bd:	41 0f 9d c2          	setge  r10b
    65c1:	48 3b 05 00 04 00 00 	cmp    rax,QWORD PTR [rip+0x400]        # 69c8 <benchmark+0x578>
    65c8:	41 0f 9e c3          	setle  r11b
    65cc:	45 84 d3             	test   r11b,r10b
    65cf:	0f 85 16 00 00 00    	jne    65eb <benchmark+0x19b>
    65d5:	48 8d 05 2e 6d 00 00 	lea    rax,[rip+0x6d2e]        # d30a <cljn_dec>
    65dc:	48 89 df             	mov    rdi,rbx
    65df:	ff d0                	call   rax
    65e1:	48 89 44 24 08       	mov    QWORD PTR [rsp+0x8],rax
    65e6:	e9 0c 00 00 00       	jmp    65f7 <benchmark+0x1a7>
    65eb:	48 d1 e0             	shl    rax,1
    65ee:	48 83 c8 01          	or     rax,0x1
    65f2:	48 89 44 24 08       	mov    QWORD PTR [rsp+0x8],rax
    65f7:	48 8d 15 82 da 00 02 	lea    rdx,[rip+0x200da82]        # 2014080 <gc_sp>
    65fe:	4c 8b 02             	mov    r8,QWORD PTR [rdx]
    6601:	4c 8d 0d 78 da 00 00 	lea    r9,[rip+0xda78]        # 14080 <gc_stack>
    6608:	4d 6b d0 08          	imul   r10,r8,0x8
    660c:	4b c7 04 11 c9 00 00 	mov    QWORD PTR [r9+r10*1],0xc9
    6613:	00 
    6614:	49 81 c0 01 00 00 00 	add    r8,0x1
    661b:	4c 89 02             	mov    QWORD PTR [rdx],r8
    661e:	41 bc 01 00 00 00    	mov    r12d,0x1
    6624:	4c 8d 0d 24 12 00 00 	lea    r9,[rip+0x1224]        # 784f <cljn_argv>
    662b:	4c 89 e7             	mov    rdi,r12
    662e:	41 ff d1             	call   r9
    6631:	bf 02 00 00 00       	mov    edi,0x2
    6636:	48 89 c2             	mov    rdx,rax
    6639:	4c 89 e6             	mov    rsi,r12
    663c:	e8 97 f6 ff ff       	call   5cd8 <count-primes>
    6641:	4c 8d 0d 38 da 00 02 	lea    r9,[rip+0x200da38]        # 2014080 <gc_sp>
    6648:	49 83 01 ff          	add    QWORD PTR [r9],0xffffffffffffffff
    664c:	4c 8d 15 2d da 00 02 	lea    r10,[rip+0x200da2d]        # 2014080 <gc_sp>
    6653:	4d 8b 1a             	mov    r11,QWORD PTR [r10]
    6656:	48 8d 35 23 da 00 00 	lea    rsi,[rip+0xda23]        # 14080 <gc_stack>
    665d:	49 6b fb 08          	imul   rdi,r11,0x8
    6661:	49 89 c5             	mov    r13,rax
    6664:	4c 89 2c 3e          	mov    QWORD PTR [rsi+rdi*1],r13
    6668:	49 81 c3 01 00 00 00 	add    r11,0x1
    666f:	4d 89 1a             	mov    QWORD PTR [r10],r11
    6672:	48 8d 35 07 da 00 02 	lea    rsi,[rip+0x200da07]        # 2014080 <gc_sp>
    6679:	48 8b 3e             	mov    rdi,QWORD PTR [rsi]
    667c:	48 8d 05 fd d9 00 00 	lea    rax,[rip+0xd9fd]        # 14080 <gc_stack>
    6683:	48 6b cf 08          	imul   rcx,rdi,0x8
    6687:	48 c7 04 08 2d 01 00 	mov    QWORD PTR [rax+rcx*1],0x12d
    668e:	00 
    668f:	48 81 c7 01 00 00 00 	add    rdi,0x1
    6696:	48 89 3e             	mov    QWORD PTR [rsi],rdi
    6699:	41 bc 01 00 00 00    	mov    r12d,0x1
    669f:	48 8d 05 a9 11 00 00 	lea    rax,[rip+0x11a9]        # 784f <cljn_argv>
    66a6:	4c 89 e7             	mov    rdi,r12
    66a9:	ff d0                	call   rax
    66ab:	bf 02 00 00 00       	mov    edi,0x2
    66b0:	48 89 c2             	mov    rdx,rax
    66b3:	4c 89 e6             	mov    rsi,r12
    66b6:	e8 1d f6 ff ff       	call   5cd8 <count-primes>
    66bb:	48 89 c6             	mov    rsi,rax
    66be:	48 8d 05 bb d9 00 02 	lea    rax,[rip+0x200d9bb]        # 2014080 <gc_sp>
    66c5:	48 83 00 ff          	add    QWORD PTR [rax],0xffffffffffffffff
    66c9:	48 8d 0d b0 d9 00 02 	lea    rcx,[rip+0x200d9b0]        # 2014080 <gc_sp>
    66d0:	48 8b 11             	mov    rdx,QWORD PTR [rcx]
    66d3:	4c 8d 05 a6 d9 00 00 	lea    r8,[rip+0xd9a6]        # 14080 <gc_stack>
    66da:	4c 6b ca 08          	imul   r9,rdx,0x8
    66de:	49 89 f4             	mov    r12,rsi
    66e1:	4f 89 24 08          	mov    QWORD PTR [r8+r9*1],r12
    66e5:	48 81 c2 01 00 00 00 	add    rdx,0x1
    66ec:	48 89 11             	mov    QWORD PTR [rcx],rdx
    66ef:	41 bf a1 00 00 00    	mov    r15d,0xa1
    66f5:	be 29 00 00 00       	mov    esi,0x29
    66fa:	48 89 df             	mov    rdi,rbx
    66fd:	49 89 f8             	mov    r8,rdi
    6700:	49 83 e0 29          	and    r8,0x29
    6704:	49 f7 c0 01 00 00 00 	test   r8,0x1
    670b:	0f 84 5b 00 00 00    	je     676c <benchmark+0x31c>
    6711:	48 89 df             	mov    rdi,rbx
    6714:	48 89 f8             	mov    rax,rdi
    6717:	48 d1 f8             	sar    rax,1
    671a:	49 89 f3             	mov    r11,rsi
    671d:	49 d1 fb             	sar    r11,1
    6720:	4d 85 db             	test   r11,r11
    6723:	0f 84 43 00 00 00    	je     676c <benchmark+0x31c>
    6729:	48 99                	cqo
    672b:	49 83 fb ff          	cmp    r11,0xffffffffffffffff
    672f:	0f 85 0a 00 00 00    	jne    673f <benchmark+0x2ef>
    6735:	ba 00 00 00 00       	mov    edx,0x0
    673a:	e9 03 00 00 00       	jmp    6742 <benchmark+0x2f2>
    673f:	49 f7 fb             	idiv   r11
    6742:	48 85 d2             	test   rdx,rdx
    6745:	0f 95 c0             	setne  al
    6748:	48 89 d7             	mov    rdi,rdx
    674b:	49 33 fb             	xor    rdi,r11
    674e:	48 c1 ef 3f          	shr    rdi,0x3f
    6752:	4a 8d 0c 1a          	lea    rcx,[rdx+r11*1]
    6756:	40 84 c7             	test   dil,al
    6759:	48 0f 45 d1          	cmovne rdx,rcx
    675d:	48 d1 e2             	shl    rdx,1
    6760:	48 83 ca 01          	or     rdx,0x1
    6764:	48 89 d6             	mov    rsi,rdx
    6767:	e9 0f 00 00 00       	jmp    677b <benchmark+0x32b>
    676c:	48 8d 15 69 6a 00 00 	lea    rdx,[rip+0x6a69]        # d1dc <cljn_mod>
    6773:	48 89 df             	mov    rdi,rbx
    6776:	ff d2                	call   rdx
    6778:	48 89 c6             	mov    rsi,rax
    677b:	49 89 f0             	mov    r8,rsi
    677e:	49 81 e0 a1 00 00 00 	and    r8,0xa1
    6785:	49 f7 c0 01 00 00 00 	test   r8,0x1
    678c:	0f 84 2e 00 00 00    	je     67c0 <benchmark+0x370>
    6792:	4d 89 fb             	mov    r11,r15
    6795:	49 d1 fb             	sar    r11,1
    6798:	48 89 f7             	mov    rdi,rsi
    679b:	48 d1 ff             	sar    rdi,1
    679e:	49 8d 04 3b          	lea    rax,[r11+rdi*1]
    67a2:	48 3b 05 17 02 00 00 	cmp    rax,QWORD PTR [rip+0x217]        # 69c0 <benchmark+0x570>
    67a9:	40 0f 9d c7          	setge  dil
    67ad:	48 3b 05 14 02 00 00 	cmp    rax,QWORD PTR [rip+0x214]        # 69c8 <benchmark+0x578>
    67b4:	0f 9e c1             	setle  cl
    67b7:	40 84 f9             	test   cl,dil
    67ba:	0f 85 11 00 00 00    	jne    67d1 <benchmark+0x381>
    67c0:	48 8d 15 5c 67 00 00 	lea    rdx,[rip+0x675c]        # cf23 <cljn_add>
    67c7:	4c 89 ff             	mov    rdi,r15
    67ca:	ff d2                	call   rdx
    67cc:	e9 07 00 00 00       	jmp    67d8 <benchmark+0x388>
    67d1:	48 d1 e0             	shl    rax,1
    67d4:	48 83 c8 01          	or     rax,0x1
    67d8:	4c 8d 0d a1 d8 00 02 	lea    r9,[rip+0x200d8a1]        # 2014080 <gc_sp>
    67df:	4d 8b 11             	mov    r10,QWORD PTR [r9]
    67e2:	4c 8d 1d 97 d8 00 00 	lea    r11,[rip+0xd897]        # 14080 <gc_stack>
    67e9:	49 6b f2 08          	imul   rsi,r10,0x8
    67ed:	49 89 04 33          	mov    QWORD PTR [r11+rsi*1],rax
    67f1:	49 81 c2 01 00 00 00 	add    r10,0x1
    67f8:	4d 89 11             	mov    QWORD PTR [r9],r10
    67fb:	bb 01 00 00 00       	mov    ebx,0x1
    6800:	4c 8d 1d 48 10 00 00 	lea    r11,[rip+0x1048]        # 784f <cljn_argv>
    6807:	48 89 df             	mov    rdi,rbx
    680a:	41 ff d3             	call   r11
    680d:	bf 02 00 00 00       	mov    edi,0x2
    6812:	48 89 c2             	mov    rdx,rax
    6815:	48 89 de             	mov    rsi,rbx
    6818:	e8 bb f4 ff ff       	call   5cd8 <count-primes>
    681d:	4c 8d 1d 5c d8 00 02 	lea    r11,[rip+0x200d85c]        # 2014080 <gc_sp>
    6824:	49 83 03 ff          	add    QWORD PTR [r11],0xffffffffffffffff
    6828:	48 8d 35 51 d8 00 02 	lea    rsi,[rip+0x200d851]        # 2014080 <gc_sp>
    682f:	48 8b 3e             	mov    rdi,QWORD PTR [rsi]
    6832:	48 8d 0d 47 d8 00 00 	lea    rcx,[rip+0xd847]        # 14080 <gc_stack>
    6839:	48 6b d7 08          	imul   rdx,rdi,0x8
    683d:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    6841:	49 89 c7             	mov    r15,rax
    6844:	48 81 c7 01 00 00 00 	add    rdi,0x1
    684b:	48 89 3e             	mov    QWORD PTR [rsi],rdi
    684e:	48 8b 3c 24          	mov    rdi,QWORD PTR [rsp]
    6852:	48 89 f8             	mov    rax,rdi
    6855:	49 23 c5             	and    rax,r13
    6858:	48 a9 01 00 00 00    	test   rax,0x1
    685e:	0f 84 33 00 00 00    	je     6897 <benchmark+0x447>
    6864:	48 8b 3c 24          	mov    rdi,QWORD PTR [rsp]
    6868:	49 89 f8             	mov    r8,rdi
    686b:	49 d1 f8             	sar    r8,1
    686e:	4d 89 e9             	mov    r9,r13
    6871:	49 d1 f9             	sar    r9,1
    6874:	4b 8d 3c 08          	lea    rdi,[r8+r9*1]
    6878:	48 3b 3d 41 01 00 00 	cmp    rdi,QWORD PTR [rip+0x141]        # 69c0 <benchmark+0x570>
    687f:	41 0f 9d c1          	setge  r9b
    6883:	48 3b 3d 3e 01 00 00 	cmp    rdi,QWORD PTR [rip+0x13e]        # 69c8 <benchmark+0x578>
    688a:	41 0f 9e c2          	setle  r10b
    688e:	45 84 ca             	test   r10b,r9b
    6891:	0f 85 18 00 00 00    	jne    68af <benchmark+0x45f>
    6897:	48 8d 05 85 66 00 00 	lea    rax,[rip+0x6685]        # cf23 <cljn_add>
    689e:	4c 89 ee             	mov    rsi,r13
    68a1:	48 8b 3c 24          	mov    rdi,QWORD PTR [rsp]
    68a5:	ff d0                	call   rax
    68a7:	48 89 c7             	mov    rdi,rax
    68aa:	e9 07 00 00 00       	jmp    68b6 <benchmark+0x466>
    68af:	48 d1 e7             	shl    rdi,1
    68b2:	48 83 cf 01          	or     rdi,0x1
    68b6:	48 89 f9             	mov    rcx,rdi
    68b9:	49 23 cc             	and    rcx,r12
    68bc:	48 f7 c1 01 00 00 00 	test   rcx,0x1
    68c3:	0f 85 08 00 00 00    	jne    68d1 <benchmark+0x481>
    68c9:	48 89 fe             	mov    rsi,rdi
    68cc:	e9 32 00 00 00       	jmp    6903 <benchmark+0x4b3>
    68d1:	49 89 f9             	mov    r9,rdi
    68d4:	49 d1 f9             	sar    r9,1
    68d7:	48 89 fe             	mov    rsi,rdi
    68da:	4d 89 e2             	mov    r10,r12
    68dd:	49 d1 fa             	sar    r10,1
    68e0:	4b 8d 3c 11          	lea    rdi,[r9+r10*1]
    68e4:	48 3b 3d d5 00 00 00 	cmp    rdi,QWORD PTR [rip+0xd5]        # 69c0 <benchmark+0x570>
    68eb:	41 0f 9d c2          	setge  r10b
    68ef:	48 3b 3d d2 00 00 00 	cmp    rdi,QWORD PTR [rip+0xd2]        # 69c8 <benchmark+0x578>
    68f6:	41 0f 9e c3          	setle  r11b
    68fa:	45 84 d3             	test   r11b,r10b
    68fd:	0f 85 1a 00 00 00    	jne    691d <benchmark+0x4cd>
    6903:	48 8d 05 19 66 00 00 	lea    rax,[rip+0x6619]        # cf23 <cljn_add>
    690a:	48 89 f7             	mov    rdi,rsi
    690d:	4c 89 e6             	mov    rsi,r12
    6910:	ff d0                	call   rax
    6912:	48 89 c7             	mov    rdi,rax
    6915:	4c 89 f8             	mov    rax,r15
    6918:	e9 0a 00 00 00       	jmp    6927 <benchmark+0x4d7>
    691d:	48 d1 e7             	shl    rdi,1
    6920:	48 83 cf 01          	or     rdi,0x1
    6924:	4c 89 f8             	mov    rax,r15
    6927:	48 89 fa             	mov    rdx,rdi
    692a:	48 23 d0             	and    rdx,rax
    692d:	48 f7 c2 01 00 00 00 	test   rdx,0x1
    6934:	0f 84 31 00 00 00    	je     696b <benchmark+0x51b>
    693a:	49 89 fa             	mov    r10,rdi
    693d:	49 d1 fa             	sar    r10,1
    6940:	49 89 c3             	mov    r11,rax
    6943:	49 d1 fb             	sar    r11,1
    6946:	4d 03 d3             	add    r10,r11
    6949:	4c 89 d1             	mov    rcx,r10
    694c:	48 3b 0d 6d 00 00 00 	cmp    rcx,QWORD PTR [rip+0x6d]        # 69c0 <benchmark+0x570>
    6953:	41 0f 9d c3          	setge  r11b
    6957:	48 3b 0d 6a 00 00 00 	cmp    rcx,QWORD PTR [rip+0x6a]        # 69c8 <benchmark+0x578>
    695e:	40 0f 9e c6          	setle  sil
    6962:	44 84 de             	test   sil,r11b
    6965:	0f 85 14 00 00 00    	jne    697f <benchmark+0x52f>
    696b:	48 8d 0d b1 65 00 00 	lea    rcx,[rip+0x65b1]        # cf23 <cljn_add>
    6972:	48 89 c6             	mov    rsi,rax
    6975:	ff d1                	call   rcx
    6977:	48 89 c1             	mov    rcx,rax
    697a:	e9 07 00 00 00       	jmp    6986 <benchmark+0x536>
    697f:	48 d1 e1             	shl    rcx,1
    6982:	48 83 c9 01          	or     rcx,0x1
    6986:	4c 8d 05 f3 d6 00 02 	lea    r8,[rip+0x200d6f3]        # 2014080 <gc_sp>
    698d:	49 83 00 fd          	add    QWORD PTR [r8],0xfffffffffffffffd
    6991:	4c 89 f7             	mov    rdi,r14
    6994:	4c 8d 4f 01          	lea    r9,[rdi+0x1]
    6998:	4c 8d 15 e1 d6 00 00 	lea    r10,[rip+0xd6e1]        # 14080 <gc_stack>
    699f:	4d 6b c9 08          	imul   r9,r9,0x8
    69a3:	48 8b 44 24 08       	mov    rax,QWORD PTR [rsp+0x8]
    69a8:	4b 89 04 0a          	mov    QWORD PTR [r10+r9*1],rax
    69ac:	48 89 c3             	mov    rbx,rax
    69af:	48 89 0c 24          	mov    QWORD PTR [rsp],rcx
    69b3:	e9 4b fb ff ff       	jmp    6503 <benchmark+0xb3>
    69b8:	0a 00                	or     al,BYTE PTR [rax]
	...
    69c6:	00 c0                	add    al,al
    69c8:	ff                   	(bad)
    69c9:	ff                   	(bad)
    69ca:	ff                   	(bad)
    69cb:	ff                   	(bad)
    69cc:	ff                   	(bad)
    69cd:	ff                   	(bad)
    69ce:	ff                   	(bad)
    69cf:	3f                   	(bad)

00000000000069d0 <-main>:
    69d0:	55                   	push   rbp
    69d1:	48 89 e5             	mov    rbp,rsp
    69d4:	48 83 ec 20          	sub    rsp,0x20
    69d8:	48 89 1c 24          	mov    QWORD PTR [rsp],rbx
    69dc:	4c 89 64 24 08       	mov    QWORD PTR [rsp+0x8],r12
    69e1:	4c 89 6c 24 10       	mov    QWORD PTR [rsp+0x10],r13
    69e6:	4c 89 7c 24 18       	mov    QWORD PTR [rsp+0x18],r15
    69eb:	49 89 f7             	mov    r15,rsi
    69ee:	48 33 ff             	xor    rdi,rdi
    69f1:	48 8d 05 9c 01 00 00 	lea    rax,[rip+0x19c]        # 6b94 <cljn_gc_enter>
    69f8:	ff d0                	call   rax
    69fa:	48 89 c3             	mov    rbx,rax
    69fd:	4c 89 f8             	mov    rax,r15
    6a00:	48 85 c0             	test   rax,rax
    6a03:	0f 84 34 00 00 00    	je     6a3d <-main+0x6d>
    6a09:	48 c7 c6 ff ff ff ff 	mov    rsi,0xffffffffffffffff
    6a10:	48 8d 15 65 0e 00 00 	lea    rdx,[rip+0xe65]        # 787c <cljn_check_arity>
    6a17:	48 89 c7             	mov    rdi,rax
    6a1a:	ff d2                	call   rdx
    6a1c:	b8 02 00 00 00       	mov    eax,0x2
    6a21:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    6a25:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    6a2a:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    6a2f:	4c 8b 7c 24 18       	mov    r15,QWORD PTR [rsp+0x18]
    6a34:	48 83 c4 20          	add    rsp,0x20
    6a38:	48 89 ec             	mov    rsp,rbp
    6a3b:	5d                   	pop    rbp
    6a3c:	c3                   	ret
    6a3d:	4c 8d 15 3c d6 00 02 	lea    r10,[rip+0x200d63c]        # 2014080 <gc_sp>
    6a44:	4d 8b 1a             	mov    r11,QWORD PTR [r10]
    6a47:	48 8d 35 32 d6 00 00 	lea    rsi,[rip+0xd632]        # 14080 <gc_stack>
    6a4e:	49 6b fb 08          	imul   rdi,r11,0x8
    6a52:	48 c7 04 3e 59 1b 00 	mov    QWORD PTR [rsi+rdi*1],0x1b59
    6a59:	00 
    6a5a:	49 81 c3 01 00 00 00 	add    r11,0x1
    6a61:	4d 89 1a             	mov    QWORD PTR [r10],r11
    6a64:	41 bc 01 00 00 00    	mov    r12d,0x1
    6a6a:	48 8d 35 de 0d 00 00 	lea    rsi,[rip+0xdde]        # 784f <cljn_argv>
    6a71:	4c 89 e7             	mov    rdi,r12
    6a74:	ff d6                	call   rsi
    6a76:	bf 02 00 00 00       	mov    edi,0x2
    6a7b:	48 89 c2             	mov    rdx,rax
    6a7e:	4c 89 e6             	mov    rsi,r12
    6a81:	e8 ca f9 ff ff       	call   6450 <benchmark>
    6a86:	48 8d 35 f3 d5 00 02 	lea    rsi,[rip+0x200d5f3]        # 2014080 <gc_sp>
    6a8d:	48 83 06 ff          	add    QWORD PTR [rsi],0xffffffffffffffff
    6a91:	48 8d 3d e8 d5 00 02 	lea    rdi,[rip+0x200d5e8]        # 2014080 <gc_sp>
    6a98:	48 8b 0f             	mov    rcx,QWORD PTR [rdi]
    6a9b:	48 8d 15 de d5 00 00 	lea    rdx,[rip+0xd5de]        # 14080 <gc_stack>
    6aa2:	4c 6b c1 08          	imul   r8,rcx,0x8
    6aa6:	4a 89 04 02          	mov    QWORD PTR [rdx+r8*1],rax
    6aaa:	48 81 c1 01 00 00 00 	add    rcx,0x1
    6ab1:	48 89 0f             	mov    QWORD PTR [rdi],rcx
    6ab4:	48 8d 0d 46 85 00 00 	lea    rcx,[rip+0x8546]        # f001 <cljn_print>
    6abb:	48 89 c7             	mov    rdi,rax
    6abe:	ff d1                	call   rcx
    6ac0:	48 8d 15 b9 d5 00 02 	lea    rdx,[rip+0x200d5b9]        # 2014080 <gc_sp>
    6ac7:	48 83 02 ff          	add    QWORD PTR [rdx],0xffffffffffffffff
    6acb:	4c 8d 05 67 87 00 00 	lea    r8,[rip+0x8767]        # f239 <cljn_print_newline>
    6ad2:	41 ff d0             	call   r8
    6ad5:	41 bd 02 00 00 00    	mov    r13d,0x2
    6adb:	4c 8d 0d 68 01 00 00 	lea    r9,[rip+0x168]        # 6c4a <cljn_gc_leave>
    6ae2:	48 89 df             	mov    rdi,rbx
    6ae5:	41 ff d1             	call   r9
    6ae8:	4c 89 e8             	mov    rax,r13
    6aeb:	48 8b 1c 24          	mov    rbx,QWORD PTR [rsp]
    6aef:	4c 8b 64 24 08       	mov    r12,QWORD PTR [rsp+0x8]
    6af4:	4c 8b 6c 24 10       	mov    r13,QWORD PTR [rsp+0x10]
    6af9:	4c 8b 7c 24 18       	mov    r15,QWORD PTR [rsp+0x18]
    6afe:	48 83 c4 20          	add    rsp,0x20
    6b02:	48 89 ec             	mov    rsp,rbp
    6b05:	5d                   	pop    rbp
    6b06:	c3                   	ret

0000000000006b07 <main>:
    6b07:	55                   	push   rbp
    6b08:	48 89 e5             	mov    rbp,rsp
    6b0b:	48 83 ec 10          	sub    rsp,0x10
    6b0f:	4c 89 34 24          	mov    QWORD PTR [rsp],r14
    6b13:	4c 89 7c 24 08       	mov    QWORD PTR [rsp+0x8],r15
    6b18:	48 33 ff             	xor    rdi,rdi
    6b1b:	48 8d 35 72 00 00 00 	lea    rsi,[rip+0x72]        # 6b94 <cljn_gc_enter>
    6b22:	ff d6                	call   rsi
    6b24:	49 89 c7             	mov    r15,rax
    6b27:	4d 33 f6             	xor    r14,r14
    6b2a:	48 8d 35 1e 0d 00 00 	lea    rsi,[rip+0xd1e]        # 784f <cljn_argv>
    6b31:	4c 89 f7             	mov    rdi,r14
    6b34:	ff d6                	call   rsi
    6b36:	48 89 c2             	mov    rdx,rax
    6b39:	4c 89 f6             	mov    rsi,r14
    6b3c:	bf 02 00 00 00       	mov    edi,0x2
    6b41:	e8 8a fe ff ff       	call   69d0 <-main>
    6b46:	48 8d 35 33 d5 00 02 	lea    rsi,[rip+0x200d533]        # 2014080 <gc_sp>
    6b4d:	48 8b 3e             	mov    rdi,QWORD PTR [rsi]
    6b50:	48 8d 0d 29 d5 00 00 	lea    rcx,[rip+0xd529]        # 14080 <gc_stack>
    6b57:	48 6b d7 08          	imul   rdx,rdi,0x8
    6b5b:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    6b5f:	48 81 c7 01 00 00 00 	add    rdi,0x1
    6b66:	48 89 3e             	mov    QWORD PTR [rsi],rdi
    6b69:	48 8d 05 10 d5 00 02 	lea    rax,[rip+0x200d510]        # 2014080 <gc_sp>
    6b70:	48 83 00 ff          	add    QWORD PTR [rax],0xffffffffffffffff
    6b74:	48 8d 0d cf 00 00 00 	lea    rcx,[rip+0xcf]        # 6c4a <cljn_gc_leave>
    6b7b:	4c 89 ff             	mov    rdi,r15
    6b7e:	ff d1                	call   rcx
    6b80:	33 c0                	xor    eax,eax
    6b82:	4c 8b 34 24          	mov    r14,QWORD PTR [rsp]
    6b86:	4c 8b 7c 24 08       	mov    r15,QWORD PTR [rsp+0x8]
    6b8b:	48 83 c4 10          	add    rsp,0x10
    6b8f:	48 89 ec             	mov    rsp,rbp
    6b92:	5d                   	pop    rbp
    6b93:	c3                   	ret

0000000000006b94 <cljn_gc_enter>:
    6b94:	f3 0f 1e fa          	endbr64
    6b98:	55                   	push   rbp
    6b99:	48 89 e5             	mov    rbp,rsp
    6b9c:	48 83 ec 30          	sub    rsp,0x30
    6ba0:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    6ba4:	48 8b 05 d5 d4 00 02 	mov    rax,QWORD PTR [rip+0x200d4d5]        # 2014080 <gc_sp>
    6bab:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    6baf:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    6bb3:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    6bb7:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    6bbb:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    6bbf:	48 01 d0             	add    rax,rdx
    6bc2:	48 3d 00 00 40 00    	cmp    rax,0x400000
    6bc8:	76 2d                	jbe    6bf7 <cljn_gc_enter+0x63>
    6bca:	48 8b 05 8f d4 00 00 	mov    rax,QWORD PTR [rip+0xd48f]        # 14060 <stderr@GLIBC_2.2.5>
    6bd1:	48 89 c1             	mov    rcx,rax
    6bd4:	ba 25 00 00 00       	mov    edx,0x25
    6bd9:	be 01 00 00 00       	mov    esi,0x1
    6bde:	48 8d 05 23 94 00 00 	lea    rax,[rip+0x9423]        # 10008 <_IO_stdin_used+0x8>
    6be5:	48 89 c7             	mov    rdi,rax
    6be8:	e8 23 a5 ff ff       	call   1110 <fwrite@plt>
    6bed:	bf 01 00 00 00       	mov    edi,0x1
    6bf2:	e8 09 a5 ff ff       	call   1100 <exit@plt>
    6bf7:	48 c7 45 e8 00 00 00 	mov    QWORD PTR [rbp-0x18],0x0
    6bfe:	00 
    6bff:	eb 27                	jmp    6c28 <cljn_gc_enter+0x94>
    6c01:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    6c05:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    6c09:	48 01 d0             	add    rax,rdx
    6c0c:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    6c13:	00 
    6c14:	48 8d 05 65 d4 00 00 	lea    rax,[rip+0xd465]        # 14080 <gc_stack>
    6c1b:	48 c7 04 02 02 00 00 	mov    QWORD PTR [rdx+rax*1],0x2
    6c22:	00 
    6c23:	48 83 45 e8 01       	add    QWORD PTR [rbp-0x18],0x1
    6c28:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    6c2c:	48 3b 45 f8          	cmp    rax,QWORD PTR [rbp-0x8]
    6c30:	72 cf                	jb     6c01 <cljn_gc_enter+0x6d>
    6c32:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    6c36:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    6c3a:	48 01 d0             	add    rax,rdx
    6c3d:	48 89 05 3c d4 00 02 	mov    QWORD PTR [rip+0x200d43c],rax        # 2014080 <gc_sp>
    6c44:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    6c48:	c9                   	leave
    6c49:	c3                   	ret

0000000000006c4a <cljn_gc_leave>:
    6c4a:	f3 0f 1e fa          	endbr64
    6c4e:	55                   	push   rbp
    6c4f:	48 89 e5             	mov    rbp,rsp
    6c52:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    6c56:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    6c5a:	48 89 05 1f d4 00 02 	mov    QWORD PTR [rip+0x200d41f],rax        # 2014080 <gc_sp>
    6c61:	90                   	nop
    6c62:	5d                   	pop    rbp
    6c63:	c3                   	ret

0000000000006c64 <cljn_gc_push>:
    6c64:	f3 0f 1e fa          	endbr64
    6c68:	55                   	push   rbp
    6c69:	48 89 e5             	mov    rbp,rsp
    6c6c:	48 83 ec 10          	sub    rsp,0x10
    6c70:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    6c74:	48 8b 05 05 d4 00 02 	mov    rax,QWORD PTR [rip+0x200d405]        # 2014080 <gc_sp>
    6c7b:	48 3d ff ff 3f 00    	cmp    rax,0x3fffff
    6c81:	7e 2d                	jle    6cb0 <cljn_gc_push+0x4c>
    6c83:	48 8b 05 d6 d3 00 00 	mov    rax,QWORD PTR [rip+0xd3d6]        # 14060 <stderr@GLIBC_2.2.5>
    6c8a:	48 89 c1             	mov    rcx,rax
    6c8d:	ba 25 00 00 00       	mov    edx,0x25
    6c92:	be 01 00 00 00       	mov    esi,0x1
    6c97:	48 8d 05 6a 93 00 00 	lea    rax,[rip+0x936a]        # 10008 <_IO_stdin_used+0x8>
    6c9e:	48 89 c7             	mov    rdi,rax
    6ca1:	e8 6a a4 ff ff       	call   1110 <fwrite@plt>
    6ca6:	bf 01 00 00 00       	mov    edi,0x1
    6cab:	e8 50 a4 ff ff       	call   1100 <exit@plt>
    6cb0:	48 8b 05 c9 d3 00 02 	mov    rax,QWORD PTR [rip+0x200d3c9]        # 2014080 <gc_sp>
    6cb7:	48 8d 50 01          	lea    rdx,[rax+0x1]
    6cbb:	48 89 15 be d3 00 02 	mov    QWORD PTR [rip+0x200d3be],rdx        # 2014080 <gc_sp>
    6cc2:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    6cc9:	00 
    6cca:	48 8d 15 af d3 00 00 	lea    rdx,[rip+0xd3af]        # 14080 <gc_stack>
    6cd1:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    6cd5:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    6cd9:	90                   	nop
    6cda:	c9                   	leave
    6cdb:	c3                   	ret

0000000000006cdc <cljn_gc_popn>:
    6cdc:	f3 0f 1e fa          	endbr64
    6ce0:	55                   	push   rbp
    6ce1:	48 89 e5             	mov    rbp,rsp
    6ce4:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    6ce8:	48 8b 05 91 d3 00 02 	mov    rax,QWORD PTR [rip+0x200d391]        # 2014080 <gc_sp>
    6cef:	48 89 c2             	mov    rdx,rax
    6cf2:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    6cf6:	48 29 c2             	sub    rdx,rax
    6cf9:	48 89 d0             	mov    rax,rdx
    6cfc:	48 89 05 7d d3 00 02 	mov    QWORD PTR [rip+0x200d37d],rax        # 2014080 <gc_sp>
    6d03:	90                   	nop
    6d04:	5d                   	pop    rbp
    6d05:	c3                   	ret

0000000000006d06 <cljn_gc_set>:
    6d06:	f3 0f 1e fa          	endbr64
    6d0a:	55                   	push   rbp
    6d0b:	48 89 e5             	mov    rbp,rsp
    6d0e:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    6d12:	48 89 75 f0          	mov    QWORD PTR [rbp-0x10],rsi
    6d16:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    6d1a:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    6d21:	00 
    6d22:	48 8d 15 57 d3 00 00 	lea    rdx,[rip+0xd357]        # 14080 <gc_stack>
    6d29:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    6d2d:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    6d31:	90                   	nop
    6d32:	5d                   	pop    rbp
    6d33:	c3                   	ret

0000000000006d34 <xalloc>:
    6d34:	f3 0f 1e fa          	endbr64
    6d38:	55                   	push   rbp
    6d39:	48 89 e5             	mov    rbp,rsp
    6d3c:	48 83 ec 20          	sub    rsp,0x20
    6d40:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    6d44:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    6d48:	48 89 c7             	mov    rdi,rax
    6d4b:	e8 80 a3 ff ff       	call   10d0 <malloc@plt>
    6d50:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    6d54:	48 83 7d f8 00       	cmp    QWORD PTR [rbp-0x8],0x0
    6d59:	75 2d                	jne    6d88 <xalloc+0x54>
    6d5b:	48 8b 05 fe d2 00 00 	mov    rax,QWORD PTR [rip+0xd2fe]        # 14060 <stderr@GLIBC_2.2.5>
    6d62:	48 89 c1             	mov    rcx,rax
    6d65:	ba 13 00 00 00       	mov    edx,0x13
    6d6a:	be 01 00 00 00       	mov    esi,0x1
    6d6f:	48 8d 05 b8 92 00 00 	lea    rax,[rip+0x92b8]        # 1002e <_IO_stdin_used+0x2e>
    6d76:	48 89 c7             	mov    rdi,rax
    6d79:	e8 92 a3 ff ff       	call   1110 <fwrite@plt>
    6d7e:	bf 01 00 00 00       	mov    edi,0x1
    6d83:	e8 78 a3 ff ff       	call   1100 <exit@plt>
    6d88:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    6d8c:	c9                   	leave
    6d8d:	c3                   	ret

0000000000006d8e <die>:
    6d8e:	f3 0f 1e fa          	endbr64
    6d92:	55                   	push   rbp
    6d93:	48 89 e5             	mov    rbp,rsp
    6d96:	48 83 ec 10          	sub    rsp,0x10
    6d9a:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    6d9e:	48 8b 05 bb d2 00 00 	mov    rax,QWORD PTR [rip+0xd2bb]        # 14060 <stderr@GLIBC_2.2.5>
    6da5:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    6da9:	48 8d 0d 92 92 00 00 	lea    rcx,[rip+0x9292]        # 10042 <_IO_stdin_used+0x42>
    6db0:	48 89 ce             	mov    rsi,rcx
    6db3:	48 89 c7             	mov    rdi,rax
    6db6:	b8 00 00 00 00       	mov    eax,0x0
    6dbb:	e8 f0 a2 ff ff       	call   10b0 <fprintf@plt>
    6dc0:	bf 01 00 00 00       	mov    edi,0x1
    6dc5:	e8 36 a3 ff ff       	call   1100 <exit@plt>

0000000000006dca <obj_type>:
    6dca:	f3 0f 1e fa          	endbr64
    6dce:	55                   	push   rbp
    6dcf:	48 89 e5             	mov    rbp,rsp
    6dd2:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    6dd6:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    6dda:	83 e0 07             	and    eax,0x7
    6ddd:	48 85 c0             	test   rax,rax
    6de0:	75 13                	jne    6df5 <obj_type+0x2b>
    6de2:	48 83 7d f8 00       	cmp    QWORD PTR [rbp-0x8],0x0
    6de7:	74 0c                	je     6df5 <obj_type+0x2b>
    6de9:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    6ded:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    6df0:	0f b6 c0             	movzx  eax,al
    6df3:	eb 05                	jmp    6dfa <obj_type+0x30>
    6df5:	b8 00 00 00 00       	mov    eax,0x0
    6dfa:	5d                   	pop    rbp
    6dfb:	c3                   	ret

0000000000006dfc <slab_bump>:
    6dfc:	f3 0f 1e fa          	endbr64
    6e00:	55                   	push   rbp
    6e01:	48 89 e5             	mov    rbp,rsp
    6e04:	48 83 ec 20          	sub    rsp,0x20
    6e08:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    6e0c:	48 8b 15 95 d3 00 02 	mov    rdx,QWORD PTR [rip+0x200d395]        # 20141a8 <slab_ptr>
    6e13:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    6e17:	48 01 d0             	add    rax,rdx
    6e1a:	48 8b 15 8f d3 00 02 	mov    rdx,QWORD PTR [rip+0x200d38f]        # 20141b0 <slab_end>
    6e21:	48 39 c2             	cmp    rdx,rax
    6e24:	73 42                	jae    6e68 <slab_bump+0x6c>
    6e26:	48 c7 45 f0 00 00 10 	mov    QWORD PTR [rbp-0x10],0x100000
    6e2d:	00 
    6e2e:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    6e32:	48 39 45 f0          	cmp    QWORD PTR [rbp-0x10],rax
    6e36:	73 08                	jae    6e40 <slab_bump+0x44>
    6e38:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    6e3c:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    6e40:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    6e44:	48 89 c7             	mov    rdi,rax
    6e47:	e8 e8 fe ff ff       	call   6d34 <xalloc>
    6e4c:	48 89 05 55 d3 00 02 	mov    QWORD PTR [rip+0x200d355],rax        # 20141a8 <slab_ptr>
    6e53:	48 8b 15 4e d3 00 02 	mov    rdx,QWORD PTR [rip+0x200d34e]        # 20141a8 <slab_ptr>
    6e5a:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    6e5e:	48 01 d0             	add    rax,rdx
    6e61:	48 89 05 48 d3 00 02 	mov    QWORD PTR [rip+0x200d348],rax        # 20141b0 <slab_end>
    6e68:	48 8b 05 39 d3 00 02 	mov    rax,QWORD PTR [rip+0x200d339]        # 20141a8 <slab_ptr>
    6e6f:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    6e73:	48 8b 15 2e d3 00 02 	mov    rdx,QWORD PTR [rip+0x200d32e]        # 20141a8 <slab_ptr>
    6e7a:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    6e7e:	48 01 d0             	add    rax,rdx
    6e81:	48 89 05 20 d3 00 02 	mov    QWORD PTR [rip+0x200d320],rax        # 20141a8 <slab_ptr>
    6e88:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    6e8c:	c9                   	leave
    6e8d:	c3                   	ret

0000000000006e8e <gc_init_env>:
    6e8e:	f3 0f 1e fa          	endbr64
    6e92:	55                   	push   rbp
    6e93:	48 89 e5             	mov    rbp,rsp
    6e96:	48 83 ec 10          	sub    rsp,0x10
    6e9a:	48 8d 05 ab 91 00 00 	lea    rax,[rip+0x91ab]        # 1004c <_IO_stdin_used+0x4c>
    6ea1:	48 89 c7             	mov    rdi,rax
    6ea4:	e8 87 a1 ff ff       	call   1030 <getenv@plt>
    6ea9:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    6ead:	48 83 7d f0 00       	cmp    QWORD PTR [rbp-0x10],0x0
    6eb2:	74 1d                	je     6ed1 <gc_init_env+0x43>
    6eb4:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    6eb8:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    6ebb:	84 c0                	test   al,al
    6ebd:	74 12                	je     6ed1 <gc_init_env+0x43>
    6ebf:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    6ec3:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    6ec6:	3c 30                	cmp    al,0x30
    6ec8:	74 07                	je     6ed1 <gc_init_env+0x43>
    6eca:	b8 01 00 00 00       	mov    eax,0x1
    6ecf:	eb 05                	jmp    6ed6 <gc_init_env+0x48>
    6ed1:	b8 00 00 00 00       	mov    eax,0x0
    6ed6:	89 05 3c d1 00 00    	mov    DWORD PTR [rip+0xd13c],eax        # 14018 <gc_stress>
    6edc:	48 8d 05 78 91 00 00 	lea    rax,[rip+0x9178]        # 1005b <_IO_stdin_used+0x5b>
    6ee3:	48 89 c7             	mov    rdi,rax
    6ee6:	e8 45 a1 ff ff       	call   1030 <getenv@plt>
    6eeb:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    6eef:	48 83 7d f8 00       	cmp    QWORD PTR [rbp-0x8],0x0
    6ef4:	74 1d                	je     6f13 <gc_init_env+0x85>
    6ef6:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    6efa:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    6efd:	84 c0                	test   al,al
    6eff:	74 12                	je     6f13 <gc_init_env+0x85>
    6f01:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    6f05:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    6f08:	3c 30                	cmp    al,0x30
    6f0a:	74 07                	je     6f13 <gc_init_env+0x85>
    6f0c:	b8 01 00 00 00       	mov    eax,0x1
    6f11:	eb 05                	jmp    6f18 <gc_init_env+0x8a>
    6f13:	b8 00 00 00 00       	mov    eax,0x0
    6f18:	89 05 fe d0 00 00    	mov    DWORD PTR [rip+0xd0fe],eax        # 1401c <gc_off>
    6f1e:	90                   	nop
    6f1f:	c9                   	leave
    6f20:	c3                   	ret

0000000000006f21 <maybe_gc>:
    6f21:	f3 0f 1e fa          	endbr64
    6f25:	55                   	push   rbp
    6f26:	48 89 e5             	mov    rbp,rsp
    6f29:	8b 05 e9 d0 00 00    	mov    eax,DWORD PTR [rip+0xd0e9]        # 14018 <gc_stress>
    6f2f:	85 c0                	test   eax,eax
    6f31:	79 05                	jns    6f38 <maybe_gc+0x17>
    6f33:	e8 56 ff ff ff       	call   6e8e <gc_init_env>
    6f38:	8b 05 de d0 00 00    	mov    eax,DWORD PTR [rip+0xd0de]        # 1401c <gc_off>
    6f3e:	85 c0                	test   eax,eax
    6f40:	75 2e                	jne    6f70 <maybe_gc+0x4f>
    6f42:	8b 05 50 d1 00 02    	mov    eax,DWORD PTR [rip+0x200d150]        # 2014098 <gc_disabled>
    6f48:	85 c0                	test   eax,eax
    6f4a:	75 24                	jne    6f70 <maybe_gc+0x4f>
    6f4c:	8b 05 c6 d0 00 00    	mov    eax,DWORD PTR [rip+0xd0c6]        # 14018 <gc_stress>
    6f52:	85 c0                	test   eax,eax
    6f54:	75 13                	jne    6f69 <maybe_gc+0x48>
    6f56:	48 8b 15 33 d1 00 02 	mov    rdx,QWORD PTR [rip+0x200d133]        # 2014090 <alloc_since_gc>
    6f5d:	48 8b 05 ac d0 00 00 	mov    rax,QWORD PTR [rip+0xd0ac]        # 14010 <gc_threshold>
    6f64:	48 39 c2             	cmp    rdx,rax
    6f67:	72 08                	jb     6f71 <maybe_gc+0x50>
    6f69:	e8 7e 06 00 00       	call   75ec <gc_collect>
    6f6e:	eb 01                	jmp    6f71 <maybe_gc+0x50>
    6f70:	90                   	nop
    6f71:	5d                   	pop    rbp
    6f72:	c3                   	ret

0000000000006f73 <obj_alloc>:
    6f73:	f3 0f 1e fa          	endbr64
    6f77:	55                   	push   rbp
    6f78:	48 89 e5             	mov    rbp,rsp
    6f7b:	48 83 ec 30          	sub    rsp,0x30
    6f7f:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    6f83:	89 75 d4             	mov    DWORD PTR [rbp-0x2c],esi
    6f86:	8b 05 8c d0 00 00    	mov    eax,DWORD PTR [rip+0xd08c]        # 14018 <gc_stress>
    6f8c:	85 c0                	test   eax,eax
    6f8e:	79 05                	jns    6f95 <obj_alloc+0x22>
    6f90:	e8 f9 fe ff ff       	call   6e8e <gc_init_env>
    6f95:	8b 05 fd d0 00 02    	mov    eax,DWORD PTR [rip+0x200d0fd]        # 2014098 <gc_disabled>
    6f9b:	85 c0                	test   eax,eax
    6f9d:	75 2c                	jne    6fcb <obj_alloc+0x58>
    6f9f:	8b 05 77 d0 00 00    	mov    eax,DWORD PTR [rip+0xd077]        # 1401c <gc_off>
    6fa5:	85 c0                	test   eax,eax
    6fa7:	75 22                	jne    6fcb <obj_alloc+0x58>
    6fa9:	8b 05 69 d0 00 00    	mov    eax,DWORD PTR [rip+0xd069]        # 14018 <gc_stress>
    6faf:	85 c0                	test   eax,eax
    6fb1:	75 13                	jne    6fc6 <obj_alloc+0x53>
    6fb3:	48 8b 15 d6 d0 00 02 	mov    rdx,QWORD PTR [rip+0x200d0d6]        # 2014090 <alloc_since_gc>
    6fba:	48 8b 05 4f d0 00 00 	mov    rax,QWORD PTR [rip+0xd04f]        # 14010 <gc_threshold>
    6fc1:	48 39 c2             	cmp    rdx,rax
    6fc4:	72 05                	jb     6fcb <obj_alloc+0x58>
    6fc6:	e8 21 06 00 00       	call   75ec <gc_collect>
    6fcb:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    6fcf:	48 83 c0 0f          	add    rax,0xf
    6fd3:	48 83 e0 f0          	and    rax,0xfffffffffffffff0
    6fd7:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    6fdb:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    6fdf:	48 c1 e8 04          	shr    rax,0x4
    6fe3:	89 45 ec             	mov    DWORD PTR [rbp-0x14],eax
    6fe6:	83 7d ec 20          	cmp    DWORD PTR [rbp-0x14],0x20
    6fea:	77 55                	ja     7041 <obj_alloc+0xce>
    6fec:	8b 45 ec             	mov    eax,DWORD PTR [rbp-0x14]
    6fef:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    6ff6:	00 
    6ff7:	48 8d 05 a2 d0 00 02 	lea    rax,[rip+0x200d0a2]        # 20140a0 <freelist>
    6ffe:	48 8b 04 02          	mov    rax,QWORD PTR [rdx+rax*1]
    7002:	48 85 c0             	test   rax,rax
    7005:	74 3a                	je     7041 <obj_alloc+0xce>
    7007:	8b 45 ec             	mov    eax,DWORD PTR [rbp-0x14]
    700a:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    7011:	00 
    7012:	48 8d 05 87 d0 00 02 	lea    rax,[rip+0x200d087]        # 20140a0 <freelist>
    7019:	48 8b 04 02          	mov    rax,QWORD PTR [rdx+rax*1]
    701d:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    7021:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    7025:	48 8b 40 08          	mov    rax,QWORD PTR [rax+0x8]
    7029:	8b 55 ec             	mov    edx,DWORD PTR [rbp-0x14]
    702c:	48 8d 0c d5 00 00 00 	lea    rcx,[rdx*8+0x0]
    7033:	00 
    7034:	48 8d 15 65 d0 00 02 	lea    rdx,[rip+0x200d065]        # 20140a0 <freelist>
    703b:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    703f:	eb 2f                	jmp    7070 <obj_alloc+0xfd>
    7041:	83 7d ec 20          	cmp    DWORD PTR [rbp-0x14],0x20
    7045:	77 12                	ja     7059 <obj_alloc+0xe6>
    7047:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    704b:	48 89 c7             	mov    rdi,rax
    704e:	e8 a9 fd ff ff       	call   6dfc <slab_bump>
    7053:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    7057:	eb 17                	jmp    7070 <obj_alloc+0xfd>
    7059:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    705d:	48 89 c7             	mov    rdi,rax
    7060:	e8 cf fc ff ff       	call   6d34 <xalloc>
    7065:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    7069:	c7 45 ec 00 00 00 00 	mov    DWORD PTR [rbp-0x14],0x0
    7070:	8b 45 d4             	mov    eax,DWORD PTR [rbp-0x2c]
    7073:	89 c2                	mov    edx,eax
    7075:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    7079:	88 10                	mov    BYTE PTR [rax],dl
    707b:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    707f:	c6 40 01 00          	mov    BYTE PTR [rax+0x1],0x0
    7083:	8b 45 ec             	mov    eax,DWORD PTR [rbp-0x14]
    7086:	89 c2                	mov    edx,eax
    7088:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    708c:	66 89 50 02          	mov    WORD PTR [rax+0x2],dx
    7090:	48 8b 15 f1 cf 00 02 	mov    rdx,QWORD PTR [rip+0x200cff1]        # 2014088 <all_objs>
    7097:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    709b:	48 89 50 08          	mov    QWORD PTR [rax+0x8],rdx
    709f:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    70a3:	48 89 05 de cf 00 02 	mov    QWORD PTR [rip+0x200cfde],rax        # 2014088 <all_objs>
    70aa:	48 8b 05 df cf 00 02 	mov    rax,QWORD PTR [rip+0x200cfdf]        # 2014090 <alloc_since_gc>
    70b1:	48 83 c0 01          	add    rax,0x1
    70b5:	48 89 05 d4 cf 00 02 	mov    QWORD PTR [rip+0x200cfd4],rax        # 2014090 <alloc_since_gc>
    70bc:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    70c0:	c9                   	leave
    70c1:	c3                   	ret

00000000000070c2 <gc_mark>:
    70c2:	f3 0f 1e fa          	endbr64
    70c6:	55                   	push   rbp
    70c7:	48 89 e5             	mov    rbp,rsp
    70ca:	48 81 ec a0 00 00 00 	sub    rsp,0xa0
    70d1:	48 89 bd 68 ff ff ff 	mov    QWORD PTR [rbp-0x98],rdi
    70d8:	e9 f9 03 00 00       	jmp    74d6 <gc_mark+0x414>
    70dd:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    70e4:	48 89 45 a8          	mov    QWORD PTR [rbp-0x58],rax
    70e8:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    70ec:	0f b6 40 01          	movzx  eax,BYTE PTR [rax+0x1]
    70f0:	84 c0                	test   al,al
    70f2:	0f 85 fd 03 00 00    	jne    74f5 <gc_mark+0x433>
    70f8:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    70fc:	c6 40 01 01          	mov    BYTE PTR [rax+0x1],0x1
    7100:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    7104:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    7107:	3c 02                	cmp    al,0x2
    7109:	75 2a                	jne    7135 <gc_mark+0x73>
    710b:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    7112:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    7116:	48 89 c7             	mov    rdi,rax
    7119:	e8 a4 ff ff ff       	call   70c2 <gc_mark>
    711e:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    7125:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    7129:	48 89 85 68 ff ff ff 	mov    QWORD PTR [rbp-0x98],rax
    7130:	e9 a1 03 00 00       	jmp    74d6 <gc_mark+0x414>
    7135:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    7139:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    713c:	3c 03                	cmp    al,0x3
    713e:	75 46                	jne    7186 <gc_mark+0xc4>
    7140:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    7147:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    714b:	48 c7 45 88 00 00 00 	mov    QWORD PTR [rbp-0x78],0x0
    7152:	00 
    7153:	eb 1e                	jmp    7173 <gc_mark+0xb1>
    7155:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7159:	48 8b 55 88          	mov    rdx,QWORD PTR [rbp-0x78]
    715d:	48 83 c2 04          	add    rdx,0x4
    7161:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    7166:	48 89 c7             	mov    rdi,rax
    7169:	e8 54 ff ff ff       	call   70c2 <gc_mark>
    716e:	48 83 45 88 01       	add    QWORD PTR [rbp-0x78],0x1
    7173:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7177:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    717b:	48 39 45 88          	cmp    QWORD PTR [rbp-0x78],rax
    717f:	7c d4                	jl     7155 <gc_mark+0x93>
    7181:	e9 73 03 00 00       	jmp    74f9 <gc_mark+0x437>
    7186:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    718a:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    718d:	3c 07                	cmp    al,0x7
    718f:	75 46                	jne    71d7 <gc_mark+0x115>
    7191:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    7198:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    719c:	48 c7 45 90 00 00 00 	mov    QWORD PTR [rbp-0x70],0x0
    71a3:	00 
    71a4:	eb 1e                	jmp    71c4 <gc_mark+0x102>
    71a6:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    71aa:	48 8b 55 90          	mov    rdx,QWORD PTR [rbp-0x70]
    71ae:	48 83 c2 02          	add    rdx,0x2
    71b2:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    71b7:	48 89 c7             	mov    rdi,rax
    71ba:	e8 03 ff ff ff       	call   70c2 <gc_mark>
    71bf:	48 83 45 90 01       	add    QWORD PTR [rbp-0x70],0x1
    71c4:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    71c8:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    71cc:	48 39 45 90          	cmp    QWORD PTR [rbp-0x70],rax
    71d0:	7c d4                	jl     71a6 <gc_mark+0xe4>
    71d2:	e9 22 03 00 00       	jmp    74f9 <gc_mark+0x437>
    71d7:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    71db:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    71de:	3c 09                	cmp    al,0x9
    71e0:	75 5a                	jne    723c <gc_mark+0x17a>
    71e2:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    71e9:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    71ed:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    71f1:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    71f5:	48 89 c7             	mov    rdi,rax
    71f8:	e8 c5 fe ff ff       	call   70c2 <gc_mark>
    71fd:	c7 85 7c ff ff ff 00 	mov    DWORD PTR [rbp-0x84],0x0
    7204:	00 00 00 
    7207:	eb 25                	jmp    722e <gc_mark+0x16c>
    7209:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    720d:	8b 95 7c ff ff ff    	mov    edx,DWORD PTR [rbp-0x84]
    7213:	48 63 d2             	movsxd rdx,edx
    7216:	48 83 c2 02          	add    rdx,0x2
    721a:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    721f:	48 89 c7             	mov    rdi,rax
    7222:	e8 9b fe ff ff       	call   70c2 <gc_mark>
    7227:	83 85 7c ff ff ff 01 	add    DWORD PTR [rbp-0x84],0x1
    722e:	83 bd 7c ff ff ff 1f 	cmp    DWORD PTR [rbp-0x84],0x1f
    7235:	7e d2                	jle    7209 <gc_mark+0x147>
    7237:	e9 bd 02 00 00       	jmp    74f9 <gc_mark+0x437>
    723c:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    7240:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    7243:	3c 05                	cmp    al,0x5
    7245:	75 30                	jne    7277 <gc_mark+0x1b5>
    7247:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    724e:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    7252:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    7256:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    725a:	48 89 c7             	mov    rdi,rax
    725d:	e8 60 fe ff ff       	call   70c2 <gc_mark>
    7262:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    7266:	48 8b 40 28          	mov    rax,QWORD PTR [rax+0x28]
    726a:	48 89 c7             	mov    rdi,rax
    726d:	e8 50 fe ff ff       	call   70c2 <gc_mark>
    7272:	e9 82 02 00 00       	jmp    74f9 <gc_mark+0x437>
    7277:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    727b:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    727e:	3c 06                	cmp    al,0x6
    7280:	75 49                	jne    72cb <gc_mark+0x209>
    7282:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    7289:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    728d:	48 c7 45 98 00 00 00 	mov    QWORD PTR [rbp-0x68],0x0
    7294:	00 
    7295:	eb 1e                	jmp    72b5 <gc_mark+0x1f3>
    7297:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    729b:	48 8b 55 98          	mov    rdx,QWORD PTR [rbp-0x68]
    729f:	48 83 c2 02          	add    rdx,0x2
    72a3:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    72a8:	48 89 c7             	mov    rdi,rax
    72ab:	e8 12 fe ff ff       	call   70c2 <gc_mark>
    72b0:	48 83 45 98 01       	add    QWORD PTR [rbp-0x68],0x1
    72b5:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    72b9:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    72bd:	48 01 c0             	add    rax,rax
    72c0:	48 39 45 98          	cmp    QWORD PTR [rbp-0x68],rax
    72c4:	7c d1                	jl     7297 <gc_mark+0x1d5>
    72c6:	e9 2e 02 00 00       	jmp    74f9 <gc_mark+0x437>
    72cb:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    72cf:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    72d2:	3c 0a                	cmp    al,0xa
    72d4:	74 0b                	je     72e1 <gc_mark+0x21f>
    72d6:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    72da:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    72dd:	3c 0d                	cmp    al,0xd
    72df:	75 18                	jne    72f9 <gc_mark+0x237>
    72e1:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    72e8:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    72ec:	48 89 c7             	mov    rdi,rax
    72ef:	e8 ce fd ff ff       	call   70c2 <gc_mark>
    72f4:	e9 00 02 00 00       	jmp    74f9 <gc_mark+0x437>
    72f9:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    72fd:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    7300:	3c 0f                	cmp    al,0xf
    7302:	74 0b                	je     730f <gc_mark+0x24d>
    7304:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    7308:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    730b:	3c 10                	cmp    al,0x10
    730d:	75 18                	jne    7327 <gc_mark+0x265>
    730f:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    7316:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    731a:	48 89 c7             	mov    rdi,rax
    731d:	e8 a0 fd ff ff       	call   70c2 <gc_mark>
    7322:	e9 d2 01 00 00       	jmp    74f9 <gc_mark+0x437>
    7327:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    732b:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    732e:	3c 11                	cmp    al,0x11
    7330:	75 3f                	jne    7371 <gc_mark+0x2af>
    7332:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    7339:	48 89 45 d0          	mov    QWORD PTR [rbp-0x30],rax
    733d:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    7341:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    7345:	48 89 c7             	mov    rdi,rax
    7348:	e8 75 fd ff ff       	call   70c2 <gc_mark>
    734d:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    7351:	48 8b 40 38          	mov    rax,QWORD PTR [rax+0x38]
    7355:	48 89 c7             	mov    rdi,rax
    7358:	e8 65 fd ff ff       	call   70c2 <gc_mark>
    735d:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    7361:	48 8b 40 28          	mov    rax,QWORD PTR [rax+0x28]
    7365:	48 89 85 68 ff ff ff 	mov    QWORD PTR [rbp-0x98],rax
    736c:	e9 65 01 00 00       	jmp    74d6 <gc_mark+0x414>
    7371:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    7375:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    7378:	3c 12                	cmp    al,0x12
    737a:	75 17                	jne    7393 <gc_mark+0x2d1>
    737c:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    7383:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    7387:	48 89 85 68 ff ff ff 	mov    QWORD PTR [rbp-0x98],rax
    738e:	e9 43 01 00 00       	jmp    74d6 <gc_mark+0x414>
    7393:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    7397:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    739a:	3c 0e                	cmp    al,0xe
    739c:	75 4f                	jne    73ed <gc_mark+0x32b>
    739e:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    73a5:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
    73a9:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    73ad:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    73b1:	48 89 c7             	mov    rdi,rax
    73b4:	e8 09 fd ff ff       	call   70c2 <gc_mark>
    73b9:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    73bd:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    73c1:	48 89 c7             	mov    rdi,rax
    73c4:	e8 f9 fc ff ff       	call   70c2 <gc_mark>
    73c9:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    73cd:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    73d1:	48 89 c7             	mov    rdi,rax
    73d4:	e8 e9 fc ff ff       	call   70c2 <gc_mark>
    73d9:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    73dd:	48 8b 40 28          	mov    rax,QWORD PTR [rax+0x28]
    73e1:	48 89 85 68 ff ff ff 	mov    QWORD PTR [rbp-0x98],rax
    73e8:	e9 e9 00 00 00       	jmp    74d6 <gc_mark+0x414>
    73ed:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    73f1:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    73f4:	3c 0b                	cmp    al,0xb
    73f6:	75 56                	jne    744e <gc_mark+0x38c>
    73f8:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    73ff:	48 89 45 c0          	mov    QWORD PTR [rbp-0x40],rax
    7403:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    7407:	8b 40 10             	mov    eax,DWORD PTR [rax+0x10]
    740a:	89 c0                	mov    eax,eax
    740c:	48 89 c7             	mov    rdi,rax
    740f:	e8 9c 86 00 00       	call   fab0 <__popcountdi2>
    7414:	01 c0                	add    eax,eax
    7416:	89 45 84             	mov    DWORD PTR [rbp-0x7c],eax
    7419:	c7 45 80 00 00 00 00 	mov    DWORD PTR [rbp-0x80],0x0
    7420:	eb 1f                	jmp    7441 <gc_mark+0x37f>
    7422:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    7426:	8b 55 80             	mov    edx,DWORD PTR [rbp-0x80]
    7429:	48 63 d2             	movsxd rdx,edx
    742c:	48 83 c2 02          	add    rdx,0x2
    7430:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    7435:	48 89 c7             	mov    rdi,rax
    7438:	e8 85 fc ff ff       	call   70c2 <gc_mark>
    743d:	83 45 80 01          	add    DWORD PTR [rbp-0x80],0x1
    7441:	8b 45 80             	mov    eax,DWORD PTR [rbp-0x80]
    7444:	3b 45 84             	cmp    eax,DWORD PTR [rbp-0x7c]
    7447:	7c d9                	jl     7422 <gc_mark+0x360>
    7449:	e9 ab 00 00 00       	jmp    74f9 <gc_mark+0x437>
    744e:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    7452:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    7455:	3c 0c                	cmp    al,0xc
    7457:	75 45                	jne    749e <gc_mark+0x3dc>
    7459:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    7460:	48 89 45 b8          	mov    QWORD PTR [rbp-0x48],rax
    7464:	48 c7 45 a0 00 00 00 	mov    QWORD PTR [rbp-0x60],0x0
    746b:	00 
    746c:	eb 1d                	jmp    748b <gc_mark+0x3c9>
    746e:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    7472:	48 8b 55 a0          	mov    rdx,QWORD PTR [rbp-0x60]
    7476:	48 83 c2 04          	add    rdx,0x4
    747a:	48 8b 04 d0          	mov    rax,QWORD PTR [rax+rdx*8]
    747e:	48 89 c7             	mov    rdi,rax
    7481:	e8 3c fc ff ff       	call   70c2 <gc_mark>
    7486:	48 83 45 a0 01       	add    QWORD PTR [rbp-0x60],0x1
    748b:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    748f:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    7493:	48 01 c0             	add    rax,rax
    7496:	48 39 45 a0          	cmp    QWORD PTR [rbp-0x60],rax
    749a:	7c d2                	jl     746e <gc_mark+0x3ac>
    749c:	eb 5b                	jmp    74f9 <gc_mark+0x437>
    749e:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    74a2:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    74a5:	3c 08                	cmp    al,0x8
    74a7:	75 4f                	jne    74f8 <gc_mark+0x436>
    74a9:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    74b0:	48 89 45 b0          	mov    QWORD PTR [rbp-0x50],rax
    74b4:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    74b8:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    74bc:	48 89 c7             	mov    rdi,rax
    74bf:	e8 fe fb ff ff       	call   70c2 <gc_mark>
    74c4:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    74c8:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    74cc:	48 89 c7             	mov    rdi,rax
    74cf:	e8 ee fb ff ff       	call   70c2 <gc_mark>
    74d4:	eb 23                	jmp    74f9 <gc_mark+0x437>
    74d6:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    74dd:	83 e0 07             	and    eax,0x7
    74e0:	48 85 c0             	test   rax,rax
    74e3:	75 14                	jne    74f9 <gc_mark+0x437>
    74e5:	48 83 bd 68 ff ff ff 	cmp    QWORD PTR [rbp-0x98],0x0
    74ec:	00 
    74ed:	0f 85 ea fb ff ff    	jne    70dd <gc_mark+0x1b>
    74f3:	eb 04                	jmp    74f9 <gc_mark+0x437>
    74f5:	90                   	nop
    74f6:	eb 01                	jmp    74f9 <gc_mark+0x437>
    74f8:	90                   	nop
    74f9:	c9                   	leave
    74fa:	c3                   	ret

00000000000074fb <gc_sweep>:
    74fb:	f3 0f 1e fa          	endbr64
    74ff:	55                   	push   rbp
    7500:	48 89 e5             	mov    rbp,rsp
    7503:	48 83 ec 10          	sub    rsp,0x10
    7507:	48 8d 05 7a cb 00 02 	lea    rax,[rip+0x200cb7a]        # 2014088 <all_objs>
    750e:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    7512:	e9 c1 00 00 00       	jmp    75d8 <gc_sweep+0xdd>
    7517:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    751b:	48 8b 00             	mov    rax,QWORD PTR [rax]
    751e:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    7522:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7526:	0f b6 40 01          	movzx  eax,BYTE PTR [rax+0x1]
    752a:	84 c0                	test   al,al
    752c:	74 19                	je     7547 <gc_sweep+0x4c>
    752e:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7532:	c6 40 01 00          	mov    BYTE PTR [rax+0x1],0x0
    7536:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    753a:	48 83 c0 08          	add    rax,0x8
    753e:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    7542:	e9 91 00 00 00       	jmp    75d8 <gc_sweep+0xdd>
    7547:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    754b:	48 8b 50 08          	mov    rdx,QWORD PTR [rax+0x8]
    754f:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    7553:	48 89 10             	mov    QWORD PTR [rax],rdx
    7556:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    755a:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    755d:	3c 01                	cmp    al,0x1
    755f:	75 10                	jne    7571 <gc_sweep+0x76>
    7561:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7565:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    7569:	48 89 c7             	mov    rdi,rax
    756c:	e8 cf 9a ff ff       	call   1040 <free@plt>
    7571:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7575:	0f b7 40 02          	movzx  eax,WORD PTR [rax+0x2]
    7579:	66 85 c0             	test   ax,ax
    757c:	75 0e                	jne    758c <gc_sweep+0x91>
    757e:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7582:	48 89 c7             	mov    rdi,rax
    7585:	e8 b6 9a ff ff       	call   1040 <free@plt>
    758a:	eb 4c                	jmp    75d8 <gc_sweep+0xdd>
    758c:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7590:	0f b7 40 02          	movzx  eax,WORD PTR [rax+0x2]
    7594:	0f b7 c0             	movzx  eax,ax
    7597:	48 98                	cdqe
    7599:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    75a0:	00 
    75a1:	48 8d 05 f8 ca 00 02 	lea    rax,[rip+0x200caf8]        # 20140a0 <freelist>
    75a8:	48 8b 14 02          	mov    rdx,QWORD PTR [rdx+rax*1]
    75ac:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    75b0:	48 89 50 08          	mov    QWORD PTR [rax+0x8],rdx
    75b4:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    75b8:	0f b7 40 02          	movzx  eax,WORD PTR [rax+0x2]
    75bc:	0f b7 c0             	movzx  eax,ax
    75bf:	48 98                	cdqe
    75c1:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    75c8:	00 
    75c9:	48 8d 15 d0 ca 00 02 	lea    rdx,[rip+0x200cad0]        # 20140a0 <freelist>
    75d0:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    75d4:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    75d8:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    75dc:	48 8b 00             	mov    rax,QWORD PTR [rax]
    75df:	48 85 c0             	test   rax,rax
    75e2:	0f 85 2f ff ff ff    	jne    7517 <gc_sweep+0x1c>
    75e8:	90                   	nop
    75e9:	90                   	nop
    75ea:	c9                   	leave
    75eb:	c3                   	ret

00000000000075ec <gc_collect>:
    75ec:	f3 0f 1e fa          	endbr64
    75f0:	55                   	push   rbp
    75f1:	48 89 e5             	mov    rbp,rsp
    75f4:	48 83 ec 10          	sub    rsp,0x10
    75f8:	48 c7 45 f8 00 00 00 	mov    QWORD PTR [rbp-0x8],0x0
    75ff:	00 
    7600:	eb 24                	jmp    7626 <gc_collect+0x3a>
    7602:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7606:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    760d:	00 
    760e:	48 8d 05 6b ca 00 00 	lea    rax,[rip+0xca6b]        # 14080 <gc_stack>
    7615:	48 8b 04 02          	mov    rax,QWORD PTR [rdx+rax*1]
    7619:	48 89 c7             	mov    rdi,rax
    761c:	e8 a1 fa ff ff       	call   70c2 <gc_mark>
    7621:	48 83 45 f8 01       	add    QWORD PTR [rbp-0x8],0x1
    7626:	48 8b 05 53 ca 00 02 	mov    rax,QWORD PTR [rip+0x200ca53]        # 2014080 <gc_sp>
    762d:	48 39 45 f8          	cmp    QWORD PTR [rbp-0x8],rax
    7631:	7c cf                	jl     7602 <gc_collect+0x16>
    7633:	e8 04 47 00 00       	call   bd3c <gc_mark_method_table>
    7638:	e8 f9 80 00 00       	call   f736 <gc_mark_exceptions>
    763d:	e8 c9 83 00 00       	call   fa0b <gc_mark_multi>
    7642:	e8 b4 fe ff ff       	call   74fb <gc_sweep>
    7647:	48 c7 05 3e ca 00 02 	mov    QWORD PTR [rip+0x200ca3e],0x0        # 2014090 <alloc_since_gc>
    764e:	00 00 00 00 
    7652:	90                   	nop
    7653:	c9                   	leave
    7654:	c3                   	ret

0000000000007655 <cljn_str_from>:
    7655:	f3 0f 1e fa          	endbr64
    7659:	55                   	push   rbp
    765a:	48 89 e5             	mov    rbp,rsp
    765d:	48 83 ec 20          	sub    rsp,0x20
    7661:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    7665:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    7669:	be 01 00 00 00       	mov    esi,0x1
    766e:	bf 20 00 00 00       	mov    edi,0x20
    7673:	e8 fb f8 ff ff       	call   6f73 <obj_alloc>
    7678:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    767c:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    7680:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7684:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    7688:	48 83 7d e0 00       	cmp    QWORD PTR [rbp-0x20],0x0
    768d:	7e 0e                	jle    769d <cljn_str_from+0x48>
    768f:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    7693:	48 89 c7             	mov    rdi,rax
    7696:	e8 99 f6 ff ff       	call   6d34 <xalloc>
    769b:	eb 05                	jmp    76a2 <cljn_str_from+0x4d>
    769d:	b8 00 00 00 00       	mov    eax,0x0
    76a2:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    76a6:	48 89 42 18          	mov    QWORD PTR [rdx+0x18],rax
    76aa:	48 83 7d e0 00       	cmp    QWORD PTR [rbp-0x20],0x0
    76af:	7e 1b                	jle    76cc <cljn_str_from+0x77>
    76b1:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    76b5:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    76b9:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    76bd:	48 8b 4d e8          	mov    rcx,QWORD PTR [rbp-0x18]
    76c1:	48 89 ce             	mov    rsi,rcx
    76c4:	48 89 c7             	mov    rdi,rax
    76c7:	e8 f4 99 ff ff       	call   10c0 <memcpy@plt>
    76cc:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    76d0:	c9                   	leave
    76d1:	c3                   	ret

00000000000076d2 <cljn_empty>:
    76d2:	f3 0f 1e fa          	endbr64
    76d6:	55                   	push   rbp
    76d7:	48 89 e5             	mov    rbp,rsp
    76da:	b8 12 00 00 00       	mov    eax,0x12
    76df:	5d                   	pop    rbp
    76e0:	c3                   	ret

00000000000076e1 <cljn_cons>:
    76e1:	f3 0f 1e fa          	endbr64
    76e5:	55                   	push   rbp
    76e6:	48 89 e5             	mov    rbp,rsp
    76e9:	48 83 ec 20          	sub    rsp,0x20
    76ed:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    76f1:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    76f5:	be 02 00 00 00       	mov    esi,0x2
    76fa:	bf 20 00 00 00       	mov    edi,0x20
    76ff:	e8 6f f8 ff ff       	call   6f73 <obj_alloc>
    7704:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    7708:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    770c:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    7710:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    7714:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7718:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    771c:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    7720:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7724:	c9                   	leave
    7725:	c3                   	ret

0000000000007726 <cljn_make_fn>:
    7726:	f3 0f 1e fa          	endbr64
    772a:	55                   	push   rbp
    772b:	48 89 e5             	mov    rbp,rsp
    772e:	48 83 ec 30          	sub    rsp,0x30
    7732:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    7736:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    773a:	48 89 55 d8          	mov    QWORD PTR [rbp-0x28],rdx
    773e:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    7742:	48 83 c0 05          	add    rax,0x5
    7746:	48 c1 e0 03          	shl    rax,0x3
    774a:	be 03 00 00 00       	mov    esi,0x3
    774f:	48 89 c7             	mov    rdi,rax
    7752:	e8 1c f8 ff ff       	call   6f73 <obj_alloc>
    7757:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    775b:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    775f:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7763:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    7767:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    776b:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    776f:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    7773:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7777:	48 8b 55 d8          	mov    rdx,QWORD PTR [rbp-0x28]
    777b:	48 89 50 20          	mov    QWORD PTR [rax+0x20],rdx
    777f:	48 c7 45 f0 00 00 00 	mov    QWORD PTR [rbp-0x10],0x0
    7786:	00 
    7787:	eb 1a                	jmp    77a3 <cljn_make_fn+0x7d>
    7789:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    778d:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    7791:	48 83 c2 04          	add    rdx,0x4
    7795:	48 c7 44 d0 08 02 00 	mov    QWORD PTR [rax+rdx*8+0x8],0x2
    779c:	00 00 
    779e:	48 83 45 f0 01       	add    QWORD PTR [rbp-0x10],0x1
    77a3:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    77a7:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    77ab:	48 39 45 f0          	cmp    QWORD PTR [rbp-0x10],rax
    77af:	7c d8                	jl     7789 <cljn_make_fn+0x63>
    77b1:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    77b5:	c9                   	leave
    77b6:	c3                   	ret

00000000000077b7 <cljn_fn_set_free>:
    77b7:	f3 0f 1e fa          	endbr64
    77bb:	55                   	push   rbp
    77bc:	48 89 e5             	mov    rbp,rsp
    77bf:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    77c3:	48 89 75 f0          	mov    QWORD PTR [rbp-0x10],rsi
    77c7:	48 89 55 e8          	mov    QWORD PTR [rbp-0x18],rdx
    77cb:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    77cf:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    77d3:	48 8d 4a 04          	lea    rcx,[rdx+0x4]
    77d7:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    77db:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    77e0:	90                   	nop
    77e1:	5d                   	pop    rbp
    77e2:	c3                   	ret

00000000000077e3 <cljn_fn_free>:
    77e3:	f3 0f 1e fa          	endbr64
    77e7:	55                   	push   rbp
    77e8:	48 89 e5             	mov    rbp,rsp
    77eb:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    77ef:	48 89 75 f0          	mov    QWORD PTR [rbp-0x10],rsi
    77f3:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    77f7:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    77fb:	48 83 c2 04          	add    rdx,0x4
    77ff:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    7804:	5d                   	pop    rbp
    7805:	c3                   	ret

0000000000007806 <cljn_fn_code>:
    7806:	f3 0f 1e fa          	endbr64
    780a:	55                   	push   rbp
    780b:	48 89 e5             	mov    rbp,rsp
    780e:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    7812:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7816:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    781a:	5d                   	pop    rbp
    781b:	c3                   	ret

000000000000781c <cljn_check_fn>:
    781c:	f3 0f 1e fa          	endbr64
    7820:	55                   	push   rbp
    7821:	48 89 e5             	mov    rbp,rsp
    7824:	48 83 ec 10          	sub    rsp,0x10
    7828:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    782c:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7830:	48 89 c7             	mov    rdi,rax
    7833:	e8 92 f5 ff ff       	call   6dca <obj_type>
    7838:	83 f8 03             	cmp    eax,0x3
    783b:	74 0f                	je     784c <cljn_check_fn+0x30>
    783d:	48 8d 05 24 88 00 00 	lea    rax,[rip+0x8824]        # 10068 <_IO_stdin_used+0x68>
    7844:	48 89 c7             	mov    rdi,rax
    7847:	e8 42 f5 ff ff       	call   6d8e <die>
    784c:	90                   	nop
    784d:	c9                   	leave
    784e:	c3                   	ret

000000000000784f <cljn_argv>:
    784f:	f3 0f 1e fa          	endbr64
    7853:	55                   	push   rbp
    7854:	48 89 e5             	mov    rbp,rsp
    7857:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    785b:	48 8b 05 1e c8 00 02 	mov    rax,QWORD PTR [rip+0x200c81e]        # 2014080 <gc_sp>
    7862:	48 89 c2             	mov    rdx,rax
    7865:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7869:	48 29 c2             	sub    rdx,rax
    786c:	48 c1 e2 03          	shl    rdx,0x3
    7870:	48 8d 05 09 c8 00 00 	lea    rax,[rip+0xc809]        # 14080 <gc_stack>
    7877:	48 01 d0             	add    rax,rdx
    787a:	5d                   	pop    rbp
    787b:	c3                   	ret

000000000000787c <cljn_check_arity>:
    787c:	f3 0f 1e fa          	endbr64
    7880:	55                   	push   rbp
    7881:	48 89 e5             	mov    rbp,rsp
    7884:	48 83 ec 10          	sub    rsp,0x10
    7888:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    788c:	48 89 75 f0          	mov    QWORD PTR [rbp-0x10],rsi
    7890:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7894:	48 3b 45 f0          	cmp    rax,QWORD PTR [rbp-0x10]
    7898:	74 2d                	je     78c7 <cljn_check_arity+0x4b>
    789a:	48 8b 05 bf c7 00 00 	mov    rax,QWORD PTR [rip+0xc7bf]        # 14060 <stderr@GLIBC_2.2.5>
    78a1:	48 8b 4d f8          	mov    rcx,QWORD PTR [rbp-0x8]
    78a5:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    78a9:	48 8d 35 e0 87 00 00 	lea    rsi,[rip+0x87e0]        # 10090 <_IO_stdin_used+0x90>
    78b0:	48 89 c7             	mov    rdi,rax
    78b3:	b8 00 00 00 00       	mov    eax,0x0
    78b8:	e8 f3 97 ff ff       	call   10b0 <fprintf@plt>
    78bd:	bf 01 00 00 00       	mov    edi,0x1
    78c2:	e8 39 98 ff ff       	call   1100 <exit@plt>
    78c7:	90                   	nop
    78c8:	c9                   	leave
    78c9:	c3                   	ret

00000000000078ca <cljn_check_arity_min>:
    78ca:	f3 0f 1e fa          	endbr64
    78ce:	55                   	push   rbp
    78cf:	48 89 e5             	mov    rbp,rsp
    78d2:	48 83 ec 10          	sub    rsp,0x10
    78d6:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    78da:	48 89 75 f0          	mov    QWORD PTR [rbp-0x10],rsi
    78de:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    78e2:	48 3b 45 f0          	cmp    rax,QWORD PTR [rbp-0x10]
    78e6:	7d 2d                	jge    7915 <cljn_check_arity_min+0x4b>
    78e8:	48 8b 05 71 c7 00 00 	mov    rax,QWORD PTR [rip+0xc771]        # 14060 <stderr@GLIBC_2.2.5>
    78ef:	48 8b 4d f8          	mov    rcx,QWORD PTR [rbp-0x8]
    78f3:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    78f7:	48 8d 35 ca 87 00 00 	lea    rsi,[rip+0x87ca]        # 100c8 <_IO_stdin_used+0xc8>
    78fe:	48 89 c7             	mov    rdi,rax
    7901:	b8 00 00 00 00       	mov    eax,0x0
    7906:	e8 a5 97 ff ff       	call   10b0 <fprintf@plt>
    790b:	bf 01 00 00 00       	mov    edi,0x1
    7910:	e8 eb 97 ff ff       	call   1100 <exit@plt>
    7915:	90                   	nop
    7916:	c9                   	leave
    7917:	c3                   	ret

0000000000007918 <cljn_collect_rest>:
    7918:	f3 0f 1e fa          	endbr64
    791c:	55                   	push   rbp
    791d:	48 89 e5             	mov    rbp,rsp
    7920:	48 83 ec 50          	sub    rsp,0x50
    7924:	48 89 7d c8          	mov    QWORD PTR [rbp-0x38],rdi
    7928:	48 89 75 c0          	mov    QWORD PTR [rbp-0x40],rsi
    792c:	48 89 55 b8          	mov    QWORD PTR [rbp-0x48],rdx
    7930:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    7934:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    7938:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    793c:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    7940:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    7944:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    7948:	48 c7 45 d8 12 00 00 	mov    QWORD PTR [rbp-0x28],0x12
    794f:	00 
    7950:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    7954:	48 89 c7             	mov    rdi,rax
    7957:	e8 08 f3 ff ff       	call   6c64 <cljn_gc_push>
    795c:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    7960:	48 83 e8 01          	sub    rax,0x1
    7964:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    7968:	eb 50                	jmp    79ba <cljn_collect_rest+0xa2>
    796a:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    796e:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    7975:	00 
    7976:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    797a:	48 01 d0             	add    rax,rdx
    797d:	48 8b 00             	mov    rax,QWORD PTR [rax]
    7980:	48 8b 55 d8          	mov    rdx,QWORD PTR [rbp-0x28]
    7984:	48 89 d6             	mov    rsi,rdx
    7987:	48 89 c7             	mov    rdi,rax
    798a:	e8 52 fd ff ff       	call   76e1 <cljn_cons>
    798f:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    7993:	48 8b 05 e6 c6 00 02 	mov    rax,QWORD PTR [rip+0x200c6e6]        # 2014080 <gc_sp>
    799a:	48 83 e8 01          	sub    rax,0x1
    799e:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    79a5:	00 
    79a6:	48 8d 15 d3 c6 00 00 	lea    rdx,[rip+0xc6d3]        # 14080 <gc_stack>
    79ad:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    79b1:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    79b5:	48 83 6d e0 01       	sub    QWORD PTR [rbp-0x20],0x1
    79ba:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    79be:	48 3b 45 f8          	cmp    rax,QWORD PTR [rbp-0x8]
    79c2:	7d a6                	jge    796a <cljn_collect_rest+0x52>
    79c4:	bf 01 00 00 00       	mov    edi,0x1
    79c9:	e8 0e f3 ff ff       	call   6cdc <cljn_gc_popn>
    79ce:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    79d2:	c9                   	leave
    79d3:	c3                   	ret

00000000000079d4 <cljn_spread_args>:
    79d4:	f3 0f 1e fa          	endbr64
    79d8:	55                   	push   rbp
    79d9:	48 89 e5             	mov    rbp,rsp
    79dc:	48 83 ec 50          	sub    rsp,0x50
    79e0:	48 89 7d b8          	mov    QWORD PTR [rbp-0x48],rdi
    79e4:	48 89 75 b0          	mov    QWORD PTR [rbp-0x50],rsi
    79e8:	48 c7 45 d0 00 00 00 	mov    QWORD PTR [rbp-0x30],0x0
    79ef:	00 
    79f0:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    79f4:	48 89 c7             	mov    rdi,rax
    79f7:	e8 ce f3 ff ff       	call   6dca <obj_type>
    79fc:	89 45 cc             	mov    DWORD PTR [rbp-0x34],eax
    79ff:	83 7d cc 05          	cmp    DWORD PTR [rbp-0x34],0x5
    7a03:	75 4a                	jne    7a4f <cljn_spread_args+0x7b>
    7a05:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    7a09:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    7a0d:	48 c7 45 d8 00 00 00 	mov    QWORD PTR [rbp-0x28],0x0
    7a14:	00 
    7a15:	eb 25                	jmp    7a3c <cljn_spread_args+0x68>
    7a17:	48 8b 55 d8          	mov    rdx,QWORD PTR [rbp-0x28]
    7a1b:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7a1f:	48 89 d6             	mov    rsi,rdx
    7a22:	48 89 c7             	mov    rdi,rax
    7a25:	e8 b5 05 00 00       	call   7fdf <pv_nth>
    7a2a:	48 89 c7             	mov    rdi,rax
    7a2d:	e8 32 f2 ff ff       	call   6c64 <cljn_gc_push>
    7a32:	48 83 45 d0 01       	add    QWORD PTR [rbp-0x30],0x1
    7a37:	48 83 45 d8 01       	add    QWORD PTR [rbp-0x28],0x1
    7a3c:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7a40:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    7a44:	48 39 45 d8          	cmp    QWORD PTR [rbp-0x28],rax
    7a48:	7c cd                	jl     7a17 <cljn_spread_args+0x43>
    7a4a:	e9 e2 00 00 00       	jmp    7b31 <cljn_spread_args+0x15d>
    7a4f:	83 7d cc 07          	cmp    DWORD PTR [rbp-0x34],0x7
    7a53:	75 48                	jne    7a9d <cljn_spread_args+0xc9>
    7a55:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    7a59:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    7a5d:	48 c7 45 e0 00 00 00 	mov    QWORD PTR [rbp-0x20],0x0
    7a64:	00 
    7a65:	eb 23                	jmp    7a8a <cljn_spread_args+0xb6>
    7a67:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    7a6b:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    7a6f:	48 83 c2 02          	add    rdx,0x2
    7a73:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    7a78:	48 89 c7             	mov    rdi,rax
    7a7b:	e8 e4 f1 ff ff       	call   6c64 <cljn_gc_push>
    7a80:	48 83 45 d0 01       	add    QWORD PTR [rbp-0x30],0x1
    7a85:	48 83 45 e0 01       	add    QWORD PTR [rbp-0x20],0x1
    7a8a:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    7a8e:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    7a92:	48 39 45 e0          	cmp    QWORD PTR [rbp-0x20],rax
    7a96:	7c cf                	jl     7a67 <cljn_spread_args+0x93>
    7a98:	e9 94 00 00 00       	jmp    7b31 <cljn_spread_args+0x15d>
    7a9d:	83 7d cc 0d          	cmp    DWORD PTR [rbp-0x34],0xd
    7aa1:	75 16                	jne    7ab9 <cljn_spread_args+0xe5>
    7aa3:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    7aa7:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    7aab:	48 89 c7             	mov    rdi,rax
    7aae:	e8 bd 1e 00 00       	call   9970 <hnode_push_keys>
    7ab3:	48 89 45 d0          	mov    QWORD PTR [rbp-0x30],rax
    7ab7:	eb 78                	jmp    7b31 <cljn_spread_args+0x15d>
    7ab9:	83 7d cc 10          	cmp    DWORD PTR [rbp-0x34],0x10
    7abd:	74 06                	je     7ac5 <cljn_spread_args+0xf1>
    7abf:	83 7d cc 0f          	cmp    DWORD PTR [rbp-0x34],0xf
    7ac3:	75 22                	jne    7ae7 <cljn_spread_args+0x113>
    7ac5:	83 7d cc 0f          	cmp    DWORD PTR [rbp-0x34],0xf
    7ac9:	0f 94 c0             	sete   al
    7acc:	0f b6 d0             	movzx  edx,al
    7acf:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    7ad3:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    7ad7:	89 d6                	mov    esi,edx
    7ad9:	48 89 c7             	mov    rdi,rax
    7adc:	e8 8c 3d 00 00       	call   b86d <tn_push_spread>
    7ae1:	48 89 45 d0          	mov    QWORD PTR [rbp-0x30],rax
    7ae5:	eb 4a                	jmp    7b31 <cljn_spread_args+0x15d>
    7ae7:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    7aeb:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    7aef:	eb 21                	jmp    7b12 <cljn_spread_args+0x13e>
    7af1:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    7af5:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    7af9:	48 89 c7             	mov    rdi,rax
    7afc:	e8 63 f1 ff ff       	call   6c64 <cljn_gc_push>
    7b01:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    7b05:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    7b09:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    7b0d:	48 83 45 d0 01       	add    QWORD PTR [rbp-0x30],0x1
    7b12:	48 83 7d e8 12       	cmp    QWORD PTR [rbp-0x18],0x12
    7b17:	74 18                	je     7b31 <cljn_spread_args+0x15d>
    7b19:	48 83 7d e8 02       	cmp    QWORD PTR [rbp-0x18],0x2
    7b1e:	74 11                	je     7b31 <cljn_spread_args+0x15d>
    7b20:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    7b24:	48 89 c7             	mov    rdi,rax
    7b27:	e8 9e f2 ff ff       	call   6dca <obj_type>
    7b2c:	83 f8 02             	cmp    eax,0x2
    7b2f:	74 c0                	je     7af1 <cljn_spread_args+0x11d>
    7b31:	48 8b 55 b8          	mov    rdx,QWORD PTR [rbp-0x48]
    7b35:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    7b39:	48 01 d0             	add    rax,rdx
    7b3c:	c9                   	leave
    7b3d:	c3                   	ret

0000000000007b3e <cljn_kw>:
    7b3e:	f3 0f 1e fa          	endbr64
    7b42:	55                   	push   rbp
    7b43:	48 89 e5             	mov    rbp,rsp
    7b46:	48 83 ec 20          	sub    rsp,0x20
    7b4a:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    7b4e:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    7b52:	be 04 00 00 00       	mov    esi,0x4
    7b57:	bf 20 00 00 00       	mov    edi,0x20
    7b5c:	e8 12 f4 ff ff       	call   6f73 <obj_alloc>
    7b61:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    7b65:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    7b69:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7b6d:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    7b71:	48 83 7d e0 00       	cmp    QWORD PTR [rbp-0x20],0x0
    7b76:	7e 0e                	jle    7b86 <cljn_kw+0x48>
    7b78:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    7b7c:	48 89 c7             	mov    rdi,rax
    7b7f:	e8 b0 f1 ff ff       	call   6d34 <xalloc>
    7b84:	eb 05                	jmp    7b8b <cljn_kw+0x4d>
    7b86:	b8 00 00 00 00       	mov    eax,0x0
    7b8b:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    7b8f:	48 89 42 18          	mov    QWORD PTR [rdx+0x18],rax
    7b93:	48 83 7d e0 00       	cmp    QWORD PTR [rbp-0x20],0x0
    7b98:	7e 1b                	jle    7bb5 <cljn_kw+0x77>
    7b9a:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    7b9e:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7ba2:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    7ba6:	48 8b 4d e8          	mov    rcx,QWORD PTR [rbp-0x18]
    7baa:	48 89 ce             	mov    rsi,rcx
    7bad:	48 89 c7             	mov    rdi,rax
    7bb0:	e8 0b 95 ff ff       	call   10c0 <memcpy@plt>
    7bb5:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7bb9:	c9                   	leave
    7bba:	c3                   	ret

0000000000007bbb <vnode_new>:
    7bbb:	f3 0f 1e fa          	endbr64
    7bbf:	55                   	push   rbp
    7bc0:	48 89 e5             	mov    rbp,rsp
    7bc3:	48 83 ec 10          	sub    rsp,0x10
    7bc7:	be 09 00 00 00       	mov    esi,0x9
    7bcc:	bf 18 01 00 00       	mov    edi,0x118
    7bd1:	e8 9d f3 ff ff       	call   6f73 <obj_alloc>
    7bd6:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    7bda:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7bde:	48 c7 40 10 02 00 00 	mov    QWORD PTR [rax+0x10],0x2
    7be5:	00 
    7be6:	c7 45 f4 00 00 00 00 	mov    DWORD PTR [rbp-0xc],0x0
    7bed:	eb 1b                	jmp    7c0a <vnode_new+0x4f>
    7bef:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7bf3:	8b 55 f4             	mov    edx,DWORD PTR [rbp-0xc]
    7bf6:	48 63 d2             	movsxd rdx,edx
    7bf9:	48 83 c2 02          	add    rdx,0x2
    7bfd:	48 c7 44 d0 08 02 00 	mov    QWORD PTR [rax+rdx*8+0x8],0x2
    7c04:	00 00 
    7c06:	83 45 f4 01          	add    DWORD PTR [rbp-0xc],0x1
    7c0a:	83 7d f4 1f          	cmp    DWORD PTR [rbp-0xc],0x1f
    7c0e:	7e df                	jle    7bef <vnode_new+0x34>
    7c10:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7c14:	c9                   	leave
    7c15:	c3                   	ret

0000000000007c16 <vnode_copy>:
    7c16:	f3 0f 1e fa          	endbr64
    7c1a:	55                   	push   rbp
    7c1b:	48 89 e5             	mov    rbp,rsp
    7c1e:	48 83 ec 20          	sub    rsp,0x20
    7c22:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    7c26:	be 09 00 00 00       	mov    esi,0x9
    7c2b:	bf 18 01 00 00       	mov    edi,0x118
    7c30:	e8 3e f3 ff ff       	call   6f73 <obj_alloc>
    7c35:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    7c39:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7c3d:	48 c7 40 10 02 00 00 	mov    QWORD PTR [rax+0x10],0x2
    7c44:	00 
    7c45:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    7c49:	48 8d 48 18          	lea    rcx,[rax+0x18]
    7c4d:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7c51:	48 83 c0 18          	add    rax,0x18
    7c55:	ba 00 01 00 00       	mov    edx,0x100
    7c5a:	48 89 ce             	mov    rsi,rcx
    7c5d:	48 89 c7             	mov    rdi,rax
    7c60:	e8 5b 94 ff ff       	call   10c0 <memcpy@plt>
    7c65:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7c69:	c9                   	leave
    7c6a:	c3                   	ret

0000000000007c6b <cljn_edit_new>:
    7c6b:	f3 0f 1e fa          	endbr64
    7c6f:	55                   	push   rbp
    7c70:	48 89 e5             	mov    rbp,rsp
    7c73:	be 13 00 00 00       	mov    esi,0x13
    7c78:	bf 10 00 00 00       	mov    edi,0x10
    7c7d:	e8 f1 f2 ff ff       	call   6f73 <obj_alloc>
    7c82:	5d                   	pop    rbp
    7c83:	c3                   	ret

0000000000007c84 <vnode_new_edit>:
    7c84:	f3 0f 1e fa          	endbr64
    7c88:	55                   	push   rbp
    7c89:	48 89 e5             	mov    rbp,rsp
    7c8c:	48 83 ec 20          	sub    rsp,0x20
    7c90:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    7c94:	e8 22 ff ff ff       	call   7bbb <vnode_new>
    7c99:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    7c9d:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7ca1:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    7ca5:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    7ca9:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7cad:	c9                   	leave
    7cae:	c3                   	ret

0000000000007caf <vnode_copy_edit>:
    7caf:	f3 0f 1e fa          	endbr64
    7cb3:	55                   	push   rbp
    7cb4:	48 89 e5             	mov    rbp,rsp
    7cb7:	48 83 ec 20          	sub    rsp,0x20
    7cbb:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    7cbf:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    7cc3:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    7cc7:	48 89 c7             	mov    rdi,rax
    7cca:	e8 47 ff ff ff       	call   7c16 <vnode_copy>
    7ccf:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    7cd3:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7cd7:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    7cdb:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    7cdf:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7ce3:	c9                   	leave
    7ce4:	c3                   	ret

0000000000007ce5 <vnode_editable>:
    7ce5:	f3 0f 1e fa          	endbr64
    7ce9:	55                   	push   rbp
    7cea:	48 89 e5             	mov    rbp,rsp
    7ced:	48 83 ec 10          	sub    rsp,0x10
    7cf1:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    7cf5:	48 89 75 f0          	mov    QWORD PTR [rbp-0x10],rsi
    7cf9:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7cfd:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    7d01:	48 39 45 f0          	cmp    QWORD PTR [rbp-0x10],rax
    7d05:	74 15                	je     7d1c <vnode_editable+0x37>
    7d07:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    7d0b:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7d0f:	48 89 d6             	mov    rsi,rdx
    7d12:	48 89 c7             	mov    rdi,rax
    7d15:	e8 95 ff ff ff       	call   7caf <vnode_copy_edit>
    7d1a:	eb 04                	jmp    7d20 <vnode_editable+0x3b>
    7d1c:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7d20:	c9                   	leave
    7d21:	c3                   	ret

0000000000007d22 <new_path_edit>:
    7d22:	f3 0f 1e fa          	endbr64
    7d26:	55                   	push   rbp
    7d27:	48 89 e5             	mov    rbp,rsp
    7d2a:	48 83 ec 30          	sub    rsp,0x30
    7d2e:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    7d32:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    7d36:	48 89 55 d8          	mov    QWORD PTR [rbp-0x28],rdx
    7d3a:	48 83 7d e8 00       	cmp    QWORD PTR [rbp-0x18],0x0
    7d3f:	75 06                	jne    7d47 <new_path_edit+0x25>
    7d41:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    7d45:	eb 3a                	jmp    7d81 <new_path_edit+0x5f>
    7d47:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    7d4b:	48 89 c7             	mov    rdi,rax
    7d4e:	e8 31 ff ff ff       	call   7c84 <vnode_new_edit>
    7d53:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    7d57:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    7d5b:	48 8d 48 fb          	lea    rcx,[rax-0x5]
    7d5f:	48 8b 55 d8          	mov    rdx,QWORD PTR [rbp-0x28]
    7d63:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    7d67:	48 89 c6             	mov    rsi,rax
    7d6a:	48 89 cf             	mov    rdi,rcx
    7d6d:	e8 b0 ff ff ff       	call   7d22 <new_path_edit>
    7d72:	48 89 c2             	mov    rdx,rax
    7d75:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7d79:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    7d7d:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7d81:	c9                   	leave
    7d82:	c3                   	ret

0000000000007d83 <tv_push_tail>:
    7d83:	f3 0f 1e fa          	endbr64
    7d87:	55                   	push   rbp
    7d88:	48 89 e5             	mov    rbp,rsp
    7d8b:	48 83 ec 50          	sub    rsp,0x50
    7d8f:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    7d93:	48 89 75 d0          	mov    QWORD PTR [rbp-0x30],rsi
    7d97:	48 89 55 c8          	mov    QWORD PTR [rbp-0x38],rdx
    7d9b:	48 89 4d c0          	mov    QWORD PTR [rbp-0x40],rcx
    7d9f:	4c 89 45 b8          	mov    QWORD PTR [rbp-0x48],r8
    7da3:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    7da7:	48 83 e8 01          	sub    rax,0x1
    7dab:	48 8b 55 d8          	mov    rdx,QWORD PTR [rbp-0x28]
    7daf:	89 d1                	mov    ecx,edx
    7db1:	48 d3 f8             	sar    rax,cl
    7db4:	83 e0 1f             	and    eax,0x1f
    7db7:	89 45 e4             	mov    DWORD PTR [rbp-0x1c],eax
    7dba:	48 8b 55 b8          	mov    rdx,QWORD PTR [rbp-0x48]
    7dbe:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    7dc2:	48 89 d6             	mov    rsi,rdx
    7dc5:	48 89 c7             	mov    rdi,rax
    7dc8:	e8 18 ff ff ff       	call   7ce5 <vnode_editable>
    7dcd:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    7dd1:	48 83 7d d8 05       	cmp    QWORD PTR [rbp-0x28],0x5
    7dd6:	75 0a                	jne    7de2 <tv_push_tail+0x5f>
    7dd8:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    7ddc:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    7de0:	eb 6c                	jmp    7e4e <tv_push_tail+0xcb>
    7de2:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    7de6:	8b 55 e4             	mov    edx,DWORD PTR [rbp-0x1c]
    7de9:	48 63 d2             	movsxd rdx,edx
    7dec:	48 83 c2 02          	add    rdx,0x2
    7df0:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    7df5:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    7df9:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7dfd:	48 89 c7             	mov    rdi,rax
    7e00:	e8 c5 ef ff ff       	call   6dca <obj_type>
    7e05:	83 f8 09             	cmp    eax,0x9
    7e08:	75 25                	jne    7e2f <tv_push_tail+0xac>
    7e0a:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7e0e:	48 8b 55 d8          	mov    rdx,QWORD PTR [rbp-0x28]
    7e12:	48 8d 7a fb          	lea    rdi,[rdx-0x5]
    7e16:	48 8b 75 b8          	mov    rsi,QWORD PTR [rbp-0x48]
    7e1a:	48 8b 4d c0          	mov    rcx,QWORD PTR [rbp-0x40]
    7e1e:	48 8b 55 c8          	mov    rdx,QWORD PTR [rbp-0x38]
    7e22:	49 89 f0             	mov    r8,rsi
    7e25:	48 89 c6             	mov    rsi,rax
    7e28:	e8 56 ff ff ff       	call   7d83 <tv_push_tail>
    7e2d:	eb 1b                	jmp    7e4a <tv_push_tail+0xc7>
    7e2f:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    7e33:	48 8d 48 fb          	lea    rcx,[rax-0x5]
    7e37:	48 8b 55 b8          	mov    rdx,QWORD PTR [rbp-0x48]
    7e3b:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    7e3f:	48 89 c6             	mov    rsi,rax
    7e42:	48 89 cf             	mov    rdi,rcx
    7e45:	e8 d8 fe ff ff       	call   7d22 <new_path_edit>
    7e4a:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    7e4e:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    7e52:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    7e56:	8b 4d e4             	mov    ecx,DWORD PTR [rbp-0x1c]
    7e59:	48 63 c9             	movsxd rcx,ecx
    7e5c:	48 83 c1 02          	add    rcx,0x2
    7e60:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    7e65:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    7e69:	c9                   	leave
    7e6a:	c3                   	ret

0000000000007e6b <tv_do_assoc>:
    7e6b:	f3 0f 1e fa          	endbr64
    7e6f:	55                   	push   rbp
    7e70:	48 89 e5             	mov    rbp,rsp
    7e73:	48 83 ec 40          	sub    rsp,0x40
    7e77:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    7e7b:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    7e7f:	48 89 55 d8          	mov    QWORD PTR [rbp-0x28],rdx
    7e83:	48 89 4d d0          	mov    QWORD PTR [rbp-0x30],rcx
    7e87:	4c 89 45 c8          	mov    QWORD PTR [rbp-0x38],r8
    7e8b:	48 8b 55 c8          	mov    rdx,QWORD PTR [rbp-0x38]
    7e8f:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    7e93:	48 89 d6             	mov    rsi,rdx
    7e96:	48 89 c7             	mov    rdi,rax
    7e99:	e8 47 fe ff ff       	call   7ce5 <vnode_editable>
    7e9e:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    7ea2:	48 83 7d e8 00       	cmp    QWORD PTR [rbp-0x18],0x0
    7ea7:	75 1d                	jne    7ec6 <tv_do_assoc+0x5b>
    7ea9:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    7ead:	83 e0 1f             	and    eax,0x1f
    7eb0:	48 89 c2             	mov    rdx,rax
    7eb3:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7eb7:	48 8d 4a 02          	lea    rcx,[rdx+0x2]
    7ebb:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    7ebf:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    7ec4:	eb 63                	jmp    7f29 <tv_do_assoc+0xbe>
    7ec6:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    7eca:	89 c2                	mov    edx,eax
    7ecc:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    7ed0:	89 d1                	mov    ecx,edx
    7ed2:	48 d3 f8             	sar    rax,cl
    7ed5:	83 e0 1f             	and    eax,0x1f
    7ed8:	89 45 f4             	mov    DWORD PTR [rbp-0xc],eax
    7edb:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7edf:	8b 55 f4             	mov    edx,DWORD PTR [rbp-0xc]
    7ee2:	48 63 d2             	movsxd rdx,edx
    7ee5:	48 83 c2 02          	add    rdx,0x2
    7ee9:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    7eee:	48 89 c6             	mov    rsi,rax
    7ef1:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    7ef5:	48 8d 78 fb          	lea    rdi,[rax-0x5]
    7ef9:	48 8b 4d c8          	mov    rcx,QWORD PTR [rbp-0x38]
    7efd:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    7f01:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    7f05:	49 89 c8             	mov    r8,rcx
    7f08:	48 89 d1             	mov    rcx,rdx
    7f0b:	48 89 c2             	mov    rdx,rax
    7f0e:	e8 58 ff ff ff       	call   7e6b <tv_do_assoc>
    7f13:	48 89 c1             	mov    rcx,rax
    7f16:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7f1a:	8b 55 f4             	mov    edx,DWORD PTR [rbp-0xc]
    7f1d:	48 63 d2             	movsxd rdx,edx
    7f20:	48 83 c2 02          	add    rdx,0x2
    7f24:	48 89 4c d0 08       	mov    QWORD PTR [rax+rdx*8+0x8],rcx
    7f29:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7f2d:	c9                   	leave
    7f2e:	c3                   	ret

0000000000007f2f <cljn_vec_empty>:
    7f2f:	f3 0f 1e fa          	endbr64
    7f33:	55                   	push   rbp
    7f34:	48 89 e5             	mov    rbp,rsp
    7f37:	48 83 ec 10          	sub    rsp,0x10
    7f3b:	e8 e1 ef ff ff       	call   6f21 <maybe_gc>
    7f40:	8b 05 52 c1 00 02    	mov    eax,DWORD PTR [rip+0x200c152]        # 2014098 <gc_disabled>
    7f46:	83 c0 01             	add    eax,0x1
    7f49:	89 05 49 c1 00 02    	mov    DWORD PTR [rip+0x200c149],eax        # 2014098 <gc_disabled>
    7f4f:	be 05 00 00 00       	mov    esi,0x5
    7f54:	bf 38 00 00 00       	mov    edi,0x38
    7f59:	e8 15 f0 ff ff       	call   6f73 <obj_alloc>
    7f5e:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    7f62:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7f66:	48 c7 40 10 00 00 00 	mov    QWORD PTR [rax+0x10],0x0
    7f6d:	00 
    7f6e:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7f72:	48 c7 40 18 05 00 00 	mov    QWORD PTR [rax+0x18],0x5
    7f79:	00 
    7f7a:	e8 3c fc ff ff       	call   7bbb <vnode_new>
    7f7f:	48 89 c2             	mov    rdx,rax
    7f82:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7f86:	48 89 50 20          	mov    QWORD PTR [rax+0x20],rdx
    7f8a:	e8 2c fc ff ff       	call   7bbb <vnode_new>
    7f8f:	48 89 c2             	mov    rdx,rax
    7f92:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7f96:	48 89 50 28          	mov    QWORD PTR [rax+0x28],rdx
    7f9a:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7f9e:	48 c7 40 30 00 00 00 	mov    QWORD PTR [rax+0x30],0x0
    7fa5:	00 
    7fa6:	8b 05 ec c0 00 02    	mov    eax,DWORD PTR [rip+0x200c0ec]        # 2014098 <gc_disabled>
    7fac:	83 e8 01             	sub    eax,0x1
    7faf:	89 05 e3 c0 00 02    	mov    DWORD PTR [rip+0x200c0e3],eax        # 2014098 <gc_disabled>
    7fb5:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7fb9:	c9                   	leave
    7fba:	c3                   	ret

0000000000007fbb <pv_tailoff>:
    7fbb:	f3 0f 1e fa          	endbr64
    7fbf:	55                   	push   rbp
    7fc0:	48 89 e5             	mov    rbp,rsp
    7fc3:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    7fc7:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7fcb:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    7fcf:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    7fd3:	48 8b 40 30          	mov    rax,QWORD PTR [rax+0x30]
    7fd7:	48 29 c2             	sub    rdx,rax
    7fda:	48 89 d0             	mov    rax,rdx
    7fdd:	5d                   	pop    rbp
    7fde:	c3                   	ret

0000000000007fdf <pv_nth>:
    7fdf:	f3 0f 1e fa          	endbr64
    7fe3:	55                   	push   rbp
    7fe4:	48 89 e5             	mov    rbp,rsp
    7fe7:	53                   	push   rbx
    7fe8:	48 83 ec 20          	sub    rsp,0x20
    7fec:	48 89 7d e0          	mov    QWORD PTR [rbp-0x20],rdi
    7ff0:	48 89 75 d8          	mov    QWORD PTR [rbp-0x28],rsi
    7ff4:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    7ff8:	48 89 c7             	mov    rdi,rax
    7ffb:	e8 bb ff ff ff       	call   7fbb <pv_tailoff>
    8000:	48 39 45 d8          	cmp    QWORD PTR [rbp-0x28],rax
    8004:	7c 29                	jl     802f <pv_nth+0x50>
    8006:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    800a:	48 8b 40 28          	mov    rax,QWORD PTR [rax+0x28]
    800e:	48 89 c3             	mov    rbx,rax
    8011:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    8015:	48 89 c7             	mov    rdi,rax
    8018:	e8 9e ff ff ff       	call   7fbb <pv_tailoff>
    801d:	48 8b 55 d8          	mov    rdx,QWORD PTR [rbp-0x28]
    8021:	48 29 c2             	sub    rdx,rax
    8024:	48 8d 42 02          	lea    rax,[rdx+0x2]
    8028:	48 8b 44 c3 08       	mov    rax,QWORD PTR [rbx+rax*8+0x8]
    802d:	eb 63                	jmp    8092 <pv_nth+0xb3>
    802f:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    8033:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    8037:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    803b:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    803f:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    8043:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    8047:	eb 2b                	jmp    8074 <pv_nth+0x95>
    8049:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    804d:	89 c2                	mov    edx,eax
    804f:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    8053:	89 d1                	mov    ecx,edx
    8055:	48 d3 f8             	sar    rax,cl
    8058:	83 e0 1f             	and    eax,0x1f
    805b:	48 89 c2             	mov    rdx,rax
    805e:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    8062:	48 83 c2 02          	add    rdx,0x2
    8066:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    806b:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    806f:	48 83 6d f0 05       	sub    QWORD PTR [rbp-0x10],0x5
    8074:	48 83 7d f0 00       	cmp    QWORD PTR [rbp-0x10],0x0
    8079:	7f ce                	jg     8049 <pv_nth+0x6a>
    807b:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    807f:	83 e0 1f             	and    eax,0x1f
    8082:	48 89 c2             	mov    rdx,rax
    8085:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    8089:	48 83 c2 02          	add    rdx,0x2
    808d:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    8092:	48 8b 5d f8          	mov    rbx,QWORD PTR [rbp-0x8]
    8096:	c9                   	leave
    8097:	c3                   	ret

0000000000008098 <new_path>:
    8098:	f3 0f 1e fa          	endbr64
    809c:	55                   	push   rbp
    809d:	48 89 e5             	mov    rbp,rsp
    80a0:	48 83 ec 20          	sub    rsp,0x20
    80a4:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    80a8:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    80ac:	48 83 7d e8 00       	cmp    QWORD PTR [rbp-0x18],0x0
    80b1:	75 06                	jne    80b9 <new_path+0x21>
    80b3:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    80b7:	eb 2f                	jmp    80e8 <new_path+0x50>
    80b9:	e8 fd fa ff ff       	call   7bbb <vnode_new>
    80be:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    80c2:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    80c6:	48 8d 50 fb          	lea    rdx,[rax-0x5]
    80ca:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    80ce:	48 89 c6             	mov    rsi,rax
    80d1:	48 89 d7             	mov    rdi,rdx
    80d4:	e8 bf ff ff ff       	call   8098 <new_path>
    80d9:	48 89 c2             	mov    rdx,rax
    80dc:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    80e0:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    80e4:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    80e8:	c9                   	leave
    80e9:	c3                   	ret

00000000000080ea <push_tail>:
    80ea:	f3 0f 1e fa          	endbr64
    80ee:	55                   	push   rbp
    80ef:	48 89 e5             	mov    rbp,rsp
    80f2:	48 83 ec 40          	sub    rsp,0x40
    80f6:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    80fa:	48 89 75 d0          	mov    QWORD PTR [rbp-0x30],rsi
    80fe:	48 89 55 c8          	mov    QWORD PTR [rbp-0x38],rdx
    8102:	48 89 4d c0          	mov    QWORD PTR [rbp-0x40],rcx
    8106:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    810a:	48 83 e8 01          	sub    rax,0x1
    810e:	48 8b 55 d8          	mov    rdx,QWORD PTR [rbp-0x28]
    8112:	89 d1                	mov    ecx,edx
    8114:	48 d3 f8             	sar    rax,cl
    8117:	83 e0 1f             	and    eax,0x1f
    811a:	89 45 e4             	mov    DWORD PTR [rbp-0x1c],eax
    811d:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    8121:	48 89 c7             	mov    rdi,rax
    8124:	e8 ed fa ff ff       	call   7c16 <vnode_copy>
    8129:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    812d:	48 83 7d d8 05       	cmp    QWORD PTR [rbp-0x28],0x5
    8132:	75 0a                	jne    813e <push_tail+0x54>
    8134:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    8138:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    813c:	eb 61                	jmp    819f <push_tail+0xb5>
    813e:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    8142:	8b 55 e4             	mov    edx,DWORD PTR [rbp-0x1c]
    8145:	48 63 d2             	movsxd rdx,edx
    8148:	48 83 c2 02          	add    rdx,0x2
    814c:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    8151:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    8155:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8159:	48 89 c7             	mov    rdi,rax
    815c:	e8 69 ec ff ff       	call   6dca <obj_type>
    8161:	83 f8 09             	cmp    eax,0x9
    8164:	75 1e                	jne    8184 <push_tail+0x9a>
    8166:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    816a:	48 8b 55 d8          	mov    rdx,QWORD PTR [rbp-0x28]
    816e:	48 8d 7a fb          	lea    rdi,[rdx-0x5]
    8172:	48 8b 4d c0          	mov    rcx,QWORD PTR [rbp-0x40]
    8176:	48 8b 55 c8          	mov    rdx,QWORD PTR [rbp-0x38]
    817a:	48 89 c6             	mov    rsi,rax
    817d:	e8 68 ff ff ff       	call   80ea <push_tail>
    8182:	eb 17                	jmp    819b <push_tail+0xb1>
    8184:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    8188:	48 8d 50 fb          	lea    rdx,[rax-0x5]
    818c:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    8190:	48 89 c6             	mov    rsi,rax
    8193:	48 89 d7             	mov    rdi,rdx
    8196:	e8 fd fe ff ff       	call   8098 <new_path>
    819b:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    819f:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    81a3:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    81a7:	8b 4d e4             	mov    ecx,DWORD PTR [rbp-0x1c]
    81aa:	48 63 c9             	movsxd rcx,ecx
    81ad:	48 83 c1 02          	add    rcx,0x2
    81b1:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    81b6:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    81ba:	c9                   	leave
    81bb:	c3                   	ret

00000000000081bc <cljn_vec_conj>:
    81bc:	f3 0f 1e fa          	endbr64
    81c0:	55                   	push   rbp
    81c1:	48 89 e5             	mov    rbp,rsp
    81c4:	48 83 ec 50          	sub    rsp,0x50
    81c8:	48 89 7d b8          	mov    QWORD PTR [rbp-0x48],rdi
    81cc:	48 89 75 b0          	mov    QWORD PTR [rbp-0x50],rsi
    81d0:	e8 4c ed ff ff       	call   6f21 <maybe_gc>
    81d5:	8b 05 bd be 00 02    	mov    eax,DWORD PTR [rip+0x200bebd]        # 2014098 <gc_disabled>
    81db:	83 c0 01             	add    eax,0x1
    81de:	89 05 b4 be 00 02    	mov    DWORD PTR [rip+0x200beb4],eax        # 2014098 <gc_disabled>
    81e4:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    81e8:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    81ec:	be 05 00 00 00       	mov    esi,0x5
    81f1:	bf 38 00 00 00       	mov    edi,0x38
    81f6:	e8 78 ed ff ff       	call   6f73 <obj_alloc>
    81fb:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    81ff:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    8203:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    8207:	48 8d 50 01          	lea    rdx,[rax+0x1]
    820b:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    820f:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    8213:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    8217:	48 8b 50 18          	mov    rdx,QWORD PTR [rax+0x18]
    821b:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    821f:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    8223:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    8227:	48 8b 50 20          	mov    rdx,QWORD PTR [rax+0x20]
    822b:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    822f:	48 89 50 20          	mov    QWORD PTR [rax+0x20],rdx
    8233:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    8237:	48 8b 40 30          	mov    rax,QWORD PTR [rax+0x30]
    823b:	48 83 f8 1f          	cmp    rax,0x1f
    823f:	7f 52                	jg     8293 <cljn_vec_conj+0xd7>
    8241:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    8245:	48 8b 40 28          	mov    rax,QWORD PTR [rax+0x28]
    8249:	48 89 c7             	mov    rdi,rax
    824c:	e8 c5 f9 ff ff       	call   7c16 <vnode_copy>
    8251:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    8255:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    8259:	48 8b 50 30          	mov    rdx,QWORD PTR [rax+0x30]
    825d:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8261:	48 8d 4a 02          	lea    rcx,[rdx+0x2]
    8265:	48 8b 55 b0          	mov    rdx,QWORD PTR [rbp-0x50]
    8269:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    826e:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    8272:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    8276:	48 89 50 28          	mov    QWORD PTR [rax+0x28],rdx
    827a:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    827e:	48 8b 40 30          	mov    rax,QWORD PTR [rax+0x30]
    8282:	48 8d 50 01          	lea    rdx,[rax+0x1]
    8286:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    828a:	48 89 50 30          	mov    QWORD PTR [rax+0x30],rdx
    828e:	e9 f3 00 00 00       	jmp    8386 <cljn_vec_conj+0x1ca>
    8293:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    8297:	48 8b 40 28          	mov    rax,QWORD PTR [rax+0x28]
    829b:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    829f:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    82a3:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    82a7:	48 89 45 d0          	mov    QWORD PTR [rbp-0x30],rax
    82ab:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    82af:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    82b3:	48 c1 f8 05          	sar    rax,0x5
    82b7:	48 89 c2             	mov    rdx,rax
    82ba:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    82be:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    82c2:	be 01 00 00 00       	mov    esi,0x1
    82c7:	89 c1                	mov    ecx,eax
    82c9:	48 d3 e6             	shl    rsi,cl
    82cc:	48 89 f0             	mov    rax,rsi
    82cf:	48 39 c2             	cmp    rdx,rax
    82d2:	7e 42                	jle    8316 <cljn_vec_conj+0x15a>
    82d4:	e8 e2 f8 ff ff       	call   7bbb <vnode_new>
    82d9:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
    82dd:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    82e1:	48 8b 50 20          	mov    rdx,QWORD PTR [rax+0x20]
    82e5:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    82e9:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    82ed:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    82f1:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    82f5:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    82f9:	48 89 d6             	mov    rsi,rdx
    82fc:	48 89 c7             	mov    rdi,rax
    82ff:	e8 94 fd ff ff       	call   8098 <new_path>
    8304:	48 89 c2             	mov    rdx,rax
    8307:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    830b:	48 89 50 20          	mov    QWORD PTR [rax+0x20],rdx
    830f:	48 83 45 d0 05       	add    QWORD PTR [rbp-0x30],0x5
    8314:	eb 2b                	jmp    8341 <cljn_vec_conj+0x185>
    8316:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    831a:	48 8b 48 10          	mov    rcx,QWORD PTR [rax+0x10]
    831e:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    8322:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    8326:	48 89 c6             	mov    rsi,rax
    8329:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    832d:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    8331:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    8335:	48 89 c7             	mov    rdi,rax
    8338:	e8 ad fd ff ff       	call   80ea <push_tail>
    833d:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
    8341:	e8 75 f8 ff ff       	call   7bbb <vnode_new>
    8346:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    834a:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    834e:	48 8b 55 b0          	mov    rdx,QWORD PTR [rbp-0x50]
    8352:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    8356:	48 8b 55 c8          	mov    rdx,QWORD PTR [rbp-0x38]
    835a:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    835e:	48 89 50 20          	mov    QWORD PTR [rax+0x20],rdx
    8362:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    8366:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    836a:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    836e:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    8372:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    8376:	48 89 50 28          	mov    QWORD PTR [rax+0x28],rdx
    837a:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    837e:	48 c7 40 30 01 00 00 	mov    QWORD PTR [rax+0x30],0x1
    8385:	00 
    8386:	8b 05 0c bd 00 02    	mov    eax,DWORD PTR [rip+0x200bd0c]        # 2014098 <gc_disabled>
    838c:	83 e8 01             	sub    eax,0x1
    838f:	89 05 03 bd 00 02    	mov    DWORD PTR [rip+0x200bd03],eax        # 2014098 <gc_disabled>
    8395:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    8399:	c9                   	leave
    839a:	c3                   	ret

000000000000839b <do_assoc>:
    839b:	f3 0f 1e fa          	endbr64
    839f:	55                   	push   rbp
    83a0:	48 89 e5             	mov    rbp,rsp
    83a3:	48 83 ec 30          	sub    rsp,0x30
    83a7:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    83ab:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    83af:	48 89 55 d8          	mov    QWORD PTR [rbp-0x28],rdx
    83b3:	48 89 4d d0          	mov    QWORD PTR [rbp-0x30],rcx
    83b7:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    83bb:	48 89 c7             	mov    rdi,rax
    83be:	e8 53 f8 ff ff       	call   7c16 <vnode_copy>
    83c3:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    83c7:	48 83 7d e8 00       	cmp    QWORD PTR [rbp-0x18],0x0
    83cc:	75 1d                	jne    83eb <do_assoc+0x50>
    83ce:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    83d2:	83 e0 1f             	and    eax,0x1f
    83d5:	48 89 c2             	mov    rdx,rax
    83d8:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    83dc:	48 8d 4a 02          	lea    rcx,[rdx+0x2]
    83e0:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    83e4:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    83e9:	eb 5c                	jmp    8447 <do_assoc+0xac>
    83eb:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    83ef:	89 c2                	mov    edx,eax
    83f1:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    83f5:	89 d1                	mov    ecx,edx
    83f7:	48 d3 f8             	sar    rax,cl
    83fa:	83 e0 1f             	and    eax,0x1f
    83fd:	89 45 f4             	mov    DWORD PTR [rbp-0xc],eax
    8400:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    8404:	8b 55 f4             	mov    edx,DWORD PTR [rbp-0xc]
    8407:	48 63 d2             	movsxd rdx,edx
    840a:	48 83 c2 02          	add    rdx,0x2
    840e:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    8413:	48 89 c6             	mov    rsi,rax
    8416:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    841a:	48 8d 78 fb          	lea    rdi,[rax-0x5]
    841e:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    8422:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    8426:	48 89 d1             	mov    rcx,rdx
    8429:	48 89 c2             	mov    rdx,rax
    842c:	e8 6a ff ff ff       	call   839b <do_assoc>
    8431:	48 89 c1             	mov    rcx,rax
    8434:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8438:	8b 55 f4             	mov    edx,DWORD PTR [rbp-0xc]
    843b:	48 63 d2             	movsxd rdx,edx
    843e:	48 83 c2 02          	add    rdx,0x2
    8442:	48 89 4c d0 08       	mov    QWORD PTR [rax+rdx*8+0x8],rcx
    8447:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    844b:	c9                   	leave
    844c:	c3                   	ret

000000000000844d <cljn_vec_assoc>:
    844d:	f3 0f 1e fa          	endbr64
    8451:	55                   	push   rbp
    8452:	48 89 e5             	mov    rbp,rsp
    8455:	48 83 ec 40          	sub    rsp,0x40
    8459:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    845d:	48 89 75 d0          	mov    QWORD PTR [rbp-0x30],rsi
    8461:	48 89 55 c8          	mov    QWORD PTR [rbp-0x38],rdx
    8465:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    8469:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    846d:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    8471:	83 e0 01             	and    eax,0x1
    8474:	48 85 c0             	test   rax,rax
    8477:	74 09                	je     8482 <cljn_vec_assoc+0x35>
    8479:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    847d:	48 d1 f8             	sar    rax,1
    8480:	eb 07                	jmp    8489 <cljn_vec_assoc+0x3c>
    8482:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
    8489:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    848d:	48 83 7d e8 00       	cmp    QWORD PTR [rbp-0x18],0x0
    8492:	78 0e                	js     84a2 <cljn_vec_assoc+0x55>
    8494:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    8498:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    849c:	48 39 45 e8          	cmp    QWORD PTR [rbp-0x18],rax
    84a0:	7e 0f                	jle    84b1 <cljn_vec_assoc+0x64>
    84a2:	48 8d 05 5f 7c 00 00 	lea    rax,[rip+0x7c5f]        # 10108 <_IO_stdin_used+0x108>
    84a9:	48 89 c7             	mov    rdi,rax
    84ac:	e8 dd e8 ff ff       	call   6d8e <die>
    84b1:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    84b5:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    84b9:	48 39 45 e8          	cmp    QWORD PTR [rbp-0x18],rax
    84bd:	75 18                	jne    84d7 <cljn_vec_assoc+0x8a>
    84bf:	48 8b 55 c8          	mov    rdx,QWORD PTR [rbp-0x38]
    84c3:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    84c7:	48 89 d6             	mov    rsi,rdx
    84ca:	48 89 c7             	mov    rdi,rax
    84cd:	e8 ea fc ff ff       	call   81bc <cljn_vec_conj>
    84d2:	e9 10 01 00 00       	jmp    85e7 <cljn_vec_assoc+0x19a>
    84d7:	e8 45 ea ff ff       	call   6f21 <maybe_gc>
    84dc:	8b 05 b6 bb 00 02    	mov    eax,DWORD PTR [rip+0x200bbb6]        # 2014098 <gc_disabled>
    84e2:	83 c0 01             	add    eax,0x1
    84e5:	89 05 ad bb 00 02    	mov    DWORD PTR [rip+0x200bbad],eax        # 2014098 <gc_disabled>
    84eb:	be 05 00 00 00       	mov    esi,0x5
    84f0:	bf 38 00 00 00       	mov    edi,0x38
    84f5:	e8 79 ea ff ff       	call   6f73 <obj_alloc>
    84fa:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    84fe:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    8502:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    8506:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    850a:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    850e:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    8512:	48 8b 50 18          	mov    rdx,QWORD PTR [rax+0x18]
    8516:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    851a:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    851e:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    8522:	48 8b 50 20          	mov    rdx,QWORD PTR [rax+0x20]
    8526:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    852a:	48 89 50 20          	mov    QWORD PTR [rax+0x20],rdx
    852e:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    8532:	48 8b 50 28          	mov    rdx,QWORD PTR [rax+0x28]
    8536:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    853a:	48 89 50 28          	mov    QWORD PTR [rax+0x28],rdx
    853e:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    8542:	48 8b 50 30          	mov    rdx,QWORD PTR [rax+0x30]
    8546:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    854a:	48 89 50 30          	mov    QWORD PTR [rax+0x30],rdx
    854e:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    8552:	48 89 c7             	mov    rdi,rax
    8555:	e8 61 fa ff ff       	call   7fbb <pv_tailoff>
    855a:	48 39 45 e8          	cmp    QWORD PTR [rbp-0x18],rax
    855e:	7c 46                	jl     85a6 <cljn_vec_assoc+0x159>
    8560:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    8564:	48 8b 40 28          	mov    rax,QWORD PTR [rax+0x28]
    8568:	48 89 c7             	mov    rdi,rax
    856b:	e8 a6 f6 ff ff       	call   7c16 <vnode_copy>
    8570:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    8574:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    8578:	48 89 c7             	mov    rdi,rax
    857b:	e8 3b fa ff ff       	call   7fbb <pv_tailoff>
    8580:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    8584:	48 29 c2             	sub    rdx,rax
    8587:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    858b:	48 8d 4a 02          	lea    rcx,[rdx+0x2]
    858f:	48 8b 55 c8          	mov    rdx,QWORD PTR [rbp-0x38]
    8593:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    8598:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    859c:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    85a0:	48 89 50 28          	mov    QWORD PTR [rax+0x28],rdx
    85a4:	eb 2e                	jmp    85d4 <cljn_vec_assoc+0x187>
    85a6:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    85aa:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    85ae:	48 89 c6             	mov    rsi,rax
    85b1:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    85b5:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    85b9:	48 8b 4d c8          	mov    rcx,QWORD PTR [rbp-0x38]
    85bd:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    85c1:	48 89 c7             	mov    rdi,rax
    85c4:	e8 d2 fd ff ff       	call   839b <do_assoc>
    85c9:	48 89 c2             	mov    rdx,rax
    85cc:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    85d0:	48 89 50 20          	mov    QWORD PTR [rax+0x20],rdx
    85d4:	8b 05 be ba 00 02    	mov    eax,DWORD PTR [rip+0x200babe]        # 2014098 <gc_disabled>
    85da:	83 e8 01             	sub    eax,0x1
    85dd:	89 05 b5 ba 00 02    	mov    DWORD PTR [rip+0x200bab5],eax        # 2014098 <gc_disabled>
    85e3:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    85e7:	c9                   	leave
    85e8:	c3                   	ret

00000000000085e9 <cljn_vec_count_raw>:
    85e9:	f3 0f 1e fa          	endbr64
    85ed:	55                   	push   rbp
    85ee:	48 89 e5             	mov    rbp,rsp
    85f1:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    85f5:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    85f9:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    85fd:	5d                   	pop    rbp
    85fe:	c3                   	ret

00000000000085ff <cljn_set_alloc>:
    85ff:	f3 0f 1e fa          	endbr64
    8603:	55                   	push   rbp
    8604:	48 89 e5             	mov    rbp,rsp
    8607:	48 83 ec 30          	sub    rsp,0x30
    860b:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    860f:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    8613:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    8617:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    861b:	48 83 c0 03          	add    rax,0x3
    861f:	48 c1 e0 03          	shl    rax,0x3
    8623:	be 07 00 00 00       	mov    esi,0x7
    8628:	48 89 c7             	mov    rdi,rax
    862b:	e8 43 e9 ff ff       	call   6f73 <obj_alloc>
    8630:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    8634:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8638:	48 c7 40 10 00 00 00 	mov    QWORD PTR [rax+0x10],0x0
    863f:	00 
    8640:	48 c7 45 e8 00 00 00 	mov    QWORD PTR [rbp-0x18],0x0
    8647:	00 
    8648:	eb 1a                	jmp    8664 <cljn_set_alloc+0x65>
    864a:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    864e:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    8652:	48 83 c2 02          	add    rdx,0x2
    8656:	48 c7 44 d0 08 02 00 	mov    QWORD PTR [rax+rdx*8+0x8],0x2
    865d:	00 00 
    865f:	48 83 45 e8 01       	add    QWORD PTR [rbp-0x18],0x1
    8664:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    8668:	48 3b 45 f0          	cmp    rax,QWORD PTR [rbp-0x10]
    866c:	7c dc                	jl     864a <cljn_set_alloc+0x4b>
    866e:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8672:	c9                   	leave
    8673:	c3                   	ret

0000000000008674 <set_member>:
    8674:	f3 0f 1e fa          	endbr64
    8678:	55                   	push   rbp
    8679:	48 89 e5             	mov    rbp,rsp
    867c:	48 83 ec 20          	sub    rsp,0x20
    8680:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    8684:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    8688:	48 c7 45 f8 00 00 00 	mov    QWORD PTR [rbp-0x8],0x0
    868f:	00 
    8690:	eb 30                	jmp    86c2 <set_member+0x4e>
    8692:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    8696:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    869a:	48 83 c2 02          	add    rdx,0x2
    869e:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    86a3:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    86a7:	48 89 d6             	mov    rsi,rdx
    86aa:	48 89 c7             	mov    rdi,rax
    86ad:	e8 52 4f 00 00       	call   d604 <cljn_equal_raw>
    86b2:	85 c0                	test   eax,eax
    86b4:	74 07                	je     86bd <set_member+0x49>
    86b6:	b8 01 00 00 00       	mov    eax,0x1
    86bb:	eb 18                	jmp    86d5 <set_member+0x61>
    86bd:	48 83 45 f8 01       	add    QWORD PTR [rbp-0x8],0x1
    86c2:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    86c6:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    86ca:	48 39 45 f8          	cmp    QWORD PTR [rbp-0x8],rax
    86ce:	7c c2                	jl     8692 <set_member+0x1e>
    86d0:	b8 00 00 00 00       	mov    eax,0x0
    86d5:	c9                   	leave
    86d6:	c3                   	ret

00000000000086d7 <cljn_set_add>:
    86d7:	f3 0f 1e fa          	endbr64
    86db:	55                   	push   rbp
    86dc:	48 89 e5             	mov    rbp,rsp
    86df:	48 83 ec 20          	sub    rsp,0x20
    86e3:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    86e7:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    86eb:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    86ef:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    86f3:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    86f7:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    86fb:	48 89 d6             	mov    rsi,rdx
    86fe:	48 89 c7             	mov    rdi,rax
    8701:	e8 6e ff ff ff       	call   8674 <set_member>
    8706:	85 c0                	test   eax,eax
    8708:	75 25                	jne    872f <cljn_set_add+0x58>
    870a:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    870e:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    8712:	48 8d 48 01          	lea    rcx,[rax+0x1]
    8716:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    871a:	48 89 4a 10          	mov    QWORD PTR [rdx+0x10],rcx
    871e:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    8722:	48 8d 48 02          	lea    rcx,[rax+0x2]
    8726:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    872a:	48 89 44 ca 08       	mov    QWORD PTR [rdx+rcx*8+0x8],rax
    872f:	90                   	nop
    8730:	c9                   	leave
    8731:	c3                   	ret

0000000000008732 <hset_node_assoc>:
    8732:	f3 0f 1e fa          	endbr64
    8736:	55                   	push   rbp
    8737:	48 89 e5             	mov    rbp,rsp
    873a:	48 83 ec 40          	sub    rsp,0x40
    873e:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    8742:	48 89 75 d0          	mov    QWORD PTR [rbp-0x30],rsi
    8746:	48 89 55 c8          	mov    QWORD PTR [rbp-0x38],rdx
    874a:	48 89 4d c0          	mov    QWORD PTR [rbp-0x40],rcx
    874e:	64 48 8b 04 25 28 00 	mov    rax,QWORD PTR fs:0x28
    8755:	00 00 
    8757:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    875b:	31 c0                	xor    eax,eax
    875d:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    8761:	48 89 c7             	mov    rdi,rax
    8764:	e8 fb 04 00 00       	call   8c64 <cljn_hash>
    8769:	89 c6                	mov    esi,eax
    876b:	48 8d 7d ec          	lea    rdi,[rbp-0x14]
    876f:	48 8b 4d c8          	mov    rcx,QWORD PTR [rbp-0x38]
    8773:	48 8b 55 c8          	mov    rdx,QWORD PTR [rbp-0x38]
    8777:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    877b:	49 89 f9             	mov    r9,rdi
    877e:	49 89 c8             	mov    r8,rcx
    8781:	48 89 d1             	mov    rcx,rdx
    8784:	89 f2                	mov    edx,esi
    8786:	be 00 00 00 00       	mov    esi,0x0
    878b:	48 89 c7             	mov    rdi,rax
    878e:	e8 6b 09 00 00       	call   90fe <node_assoc>
    8793:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    8797:	8b 45 ec             	mov    eax,DWORD PTR [rbp-0x14]
    879a:	48 63 d0             	movsxd rdx,eax
    879d:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    87a1:	48 01 c2             	add    rdx,rax
    87a4:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    87a8:	48 89 10             	mov    QWORD PTR [rax],rdx
    87ab:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    87af:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    87b3:	64 48 2b 14 25 28 00 	sub    rdx,QWORD PTR fs:0x28
    87ba:	00 00 
    87bc:	74 05                	je     87c3 <hset_node_assoc+0x91>
    87be:	e8 9d 88 ff ff       	call   1060 <__stack_chk_fail@plt>
    87c3:	c9                   	leave
    87c4:	c3                   	ret

00000000000087c5 <cljn_set_conj>:
    87c5:	f3 0f 1e fa          	endbr64
    87c9:	55                   	push   rbp
    87ca:	48 89 e5             	mov    rbp,rsp
    87cd:	48 81 ec b0 00 00 00 	sub    rsp,0xb0
    87d4:	48 89 bd 58 ff ff ff 	mov    QWORD PTR [rbp-0xa8],rdi
    87db:	48 89 b5 50 ff ff ff 	mov    QWORD PTR [rbp-0xb0],rsi
    87e2:	64 48 8b 04 25 28 00 	mov    rax,QWORD PTR fs:0x28
    87e9:	00 00 
    87eb:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    87ef:	31 c0                	xor    eax,eax
    87f1:	e8 2b e7 ff ff       	call   6f21 <maybe_gc>
    87f6:	8b 05 9c b8 00 02    	mov    eax,DWORD PTR [rip+0x200b89c]        # 2014098 <gc_disabled>
    87fc:	83 c0 01             	add    eax,0x1
    87ff:	89 05 93 b8 00 02    	mov    DWORD PTR [rip+0x200b893],eax        # 2014098 <gc_disabled>
    8805:	48 8b 85 58 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa8]
    880c:	48 89 c7             	mov    rdi,rax
    880f:	e8 b6 e5 ff ff       	call   6dca <obj_type>
    8814:	83 f8 0d             	cmp    eax,0xd
    8817:	75 73                	jne    888c <cljn_set_conj+0xc7>
    8819:	48 8b 85 58 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa8]
    8820:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    8824:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    8828:	48 8b 70 10          	mov    rsi,QWORD PTR [rax+0x10]
    882c:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    8830:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    8834:	48 8d 8d 68 ff ff ff 	lea    rcx,[rbp-0x98]
    883b:	48 8b 95 50 ff ff ff 	mov    rdx,QWORD PTR [rbp-0xb0]
    8842:	48 89 c7             	mov    rdi,rax
    8845:	e8 e8 fe ff ff       	call   8732 <hset_node_assoc>
    884a:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    884e:	be 0d 00 00 00       	mov    esi,0xd
    8853:	bf 20 00 00 00       	mov    edi,0x20
    8858:	e8 16 e7 ff ff       	call   6f73 <obj_alloc>
    885d:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    8861:	48 8b 95 68 ff ff ff 	mov    rdx,QWORD PTR [rbp-0x98]
    8868:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    886c:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    8870:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    8874:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    8878:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    887c:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    8880:	48 89 85 70 ff ff ff 	mov    QWORD PTR [rbp-0x90],rax
    8887:	e9 d7 02 00 00       	jmp    8b63 <cljn_set_conj+0x39e>
    888c:	48 8b 85 58 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa8]
    8893:	48 89 45 88          	mov    QWORD PTR [rbp-0x78],rax
    8897:	48 8b 95 50 ff ff ff 	mov    rdx,QWORD PTR [rbp-0xb0]
    889e:	48 8b 45 88          	mov    rax,QWORD PTR [rbp-0x78]
    88a2:	48 89 d6             	mov    rsi,rdx
    88a5:	48 89 c7             	mov    rdi,rax
    88a8:	e8 c7 fd ff ff       	call   8674 <set_member>
    88ad:	85 c0                	test   eax,eax
    88af:	74 13                	je     88c4 <cljn_set_conj+0xff>
    88b1:	48 8b 85 58 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa8]
    88b8:	48 89 85 70 ff ff ff 	mov    QWORD PTR [rbp-0x90],rax
    88bf:	e9 9f 02 00 00       	jmp    8b63 <cljn_set_conj+0x39e>
    88c4:	48 8b 45 88          	mov    rax,QWORD PTR [rbp-0x78]
    88c8:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    88cc:	48 83 f8 07          	cmp    rax,0x7
    88d0:	0f 8e f6 01 00 00    	jle    8acc <cljn_set_conj+0x307>
    88d6:	bf 00 00 00 00       	mov    edi,0x0
    88db:	e8 a8 04 00 00       	call   8d88 <mnode_alloc>
    88e0:	48 89 45 a0          	mov    QWORD PTR [rbp-0x60],rax
    88e4:	48 8b 45 a0          	mov    rax,QWORD PTR [rbp-0x60]
    88e8:	c7 40 10 00 00 00 00 	mov    DWORD PTR [rax+0x10],0x0
    88ef:	be 0d 00 00 00       	mov    esi,0xd
    88f4:	bf 20 00 00 00       	mov    edi,0x20
    88f9:	e8 75 e6 ff ff       	call   6f73 <obj_alloc>
    88fe:	48 89 45 a8          	mov    QWORD PTR [rbp-0x58],rax
    8902:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    8906:	48 c7 40 10 00 00 00 	mov    QWORD PTR [rax+0x10],0x0
    890d:	00 
    890e:	48 8b 55 a0          	mov    rdx,QWORD PTR [rbp-0x60]
    8912:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    8916:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    891a:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    891e:	48 89 c7             	mov    rdi,rax
    8921:	e8 3e e3 ff ff       	call   6c64 <cljn_gc_push>
    8926:	48 c7 85 78 ff ff ff 	mov    QWORD PTR [rbp-0x88],0x0
    892d:	00 00 00 00 
    8931:	e9 b1 00 00 00       	jmp    89e7 <cljn_set_conj+0x222>
    8936:	48 8b 05 43 b7 00 02 	mov    rax,QWORD PTR [rip+0x200b743]        # 2014080 <gc_sp>
    893d:	48 83 e8 01          	sub    rax,0x1
    8941:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    8948:	00 
    8949:	48 8d 05 30 b7 00 00 	lea    rax,[rip+0xb730]        # 14080 <gc_stack>
    8950:	48 8b 04 02          	mov    rax,QWORD PTR [rdx+rax*1]
    8954:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
    8958:	48 8b 45 88          	mov    rax,QWORD PTR [rbp-0x78]
    895c:	48 8b 95 78 ff ff ff 	mov    rdx,QWORD PTR [rbp-0x88]
    8963:	48 83 c2 02          	add    rdx,0x2
    8967:	48 8b 54 d0 08       	mov    rdx,QWORD PTR [rax+rdx*8+0x8]
    896c:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    8970:	48 8b 70 10          	mov    rsi,QWORD PTR [rax+0x10]
    8974:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    8978:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    897c:	48 8d 8d 68 ff ff ff 	lea    rcx,[rbp-0x98]
    8983:	48 89 c7             	mov    rdi,rax
    8986:	e8 a7 fd ff ff       	call   8732 <hset_node_assoc>
    898b:	48 89 45 d0          	mov    QWORD PTR [rbp-0x30],rax
    898f:	be 0d 00 00 00       	mov    esi,0xd
    8994:	bf 20 00 00 00       	mov    edi,0x20
    8999:	e8 d5 e5 ff ff       	call   6f73 <obj_alloc>
    899e:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    89a2:	48 8b 95 68 ff ff ff 	mov    rdx,QWORD PTR [rbp-0x98]
    89a9:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    89ad:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    89b1:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    89b5:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    89b9:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    89bd:	48 8b 05 bc b6 00 02 	mov    rax,QWORD PTR [rip+0x200b6bc]        # 2014080 <gc_sp>
    89c4:	48 8d 50 ff          	lea    rdx,[rax-0x1]
    89c8:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    89cc:	48 8d 0c d5 00 00 00 	lea    rcx,[rdx*8+0x0]
    89d3:	00 
    89d4:	48 8d 15 a5 b6 00 00 	lea    rdx,[rip+0xb6a5]        # 14080 <gc_stack>
    89db:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    89df:	48 83 85 78 ff ff ff 	add    QWORD PTR [rbp-0x88],0x1
    89e6:	01 
    89e7:	48 8b 45 88          	mov    rax,QWORD PTR [rbp-0x78]
    89eb:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    89ef:	48 39 85 78 ff ff ff 	cmp    QWORD PTR [rbp-0x88],rax
    89f6:	0f 8c 3a ff ff ff    	jl     8936 <cljn_set_conj+0x171>
    89fc:	48 8b 05 7d b6 00 02 	mov    rax,QWORD PTR [rip+0x200b67d]        # 2014080 <gc_sp>
    8a03:	48 83 e8 01          	sub    rax,0x1
    8a07:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    8a0e:	00 
    8a0f:	48 8d 05 6a b6 00 00 	lea    rax,[rip+0xb66a]        # 14080 <gc_stack>
    8a16:	48 8b 04 02          	mov    rax,QWORD PTR [rdx+rax*1]
    8a1a:	48 89 45 b0          	mov    QWORD PTR [rbp-0x50],rax
    8a1e:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    8a22:	48 8b 70 10          	mov    rsi,QWORD PTR [rax+0x10]
    8a26:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    8a2a:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    8a2e:	48 8d 8d 68 ff ff ff 	lea    rcx,[rbp-0x98]
    8a35:	48 8b 95 50 ff ff ff 	mov    rdx,QWORD PTR [rbp-0xb0]
    8a3c:	48 89 c7             	mov    rdi,rax
    8a3f:	e8 ee fc ff ff       	call   8732 <hset_node_assoc>
    8a44:	48 89 45 b8          	mov    QWORD PTR [rbp-0x48],rax
    8a48:	be 0d 00 00 00       	mov    esi,0xd
    8a4d:	bf 20 00 00 00       	mov    edi,0x20
    8a52:	e8 1c e5 ff ff       	call   6f73 <obj_alloc>
    8a57:	48 89 45 c0          	mov    QWORD PTR [rbp-0x40],rax
    8a5b:	48 8b 95 68 ff ff ff 	mov    rdx,QWORD PTR [rbp-0x98]
    8a62:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    8a66:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    8a6a:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    8a6e:	48 8b 55 b8          	mov    rdx,QWORD PTR [rbp-0x48]
    8a72:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    8a76:	48 8b 05 03 b6 00 02 	mov    rax,QWORD PTR [rip+0x200b603]        # 2014080 <gc_sp>
    8a7d:	48 8d 50 ff          	lea    rdx,[rax-0x1]
    8a81:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    8a85:	48 8d 0c d5 00 00 00 	lea    rcx,[rdx*8+0x0]
    8a8c:	00 
    8a8d:	48 8d 15 ec b5 00 00 	lea    rdx,[rip+0xb5ec]        # 14080 <gc_stack>
    8a94:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    8a98:	48 8b 05 e1 b5 00 02 	mov    rax,QWORD PTR [rip+0x200b5e1]        # 2014080 <gc_sp>
    8a9f:	48 83 e8 01          	sub    rax,0x1
    8aa3:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    8aaa:	00 
    8aab:	48 8d 05 ce b5 00 00 	lea    rax,[rip+0xb5ce]        # 14080 <gc_stack>
    8ab2:	48 8b 04 02          	mov    rax,QWORD PTR [rdx+rax*1]
    8ab6:	48 89 85 70 ff ff ff 	mov    QWORD PTR [rbp-0x90],rax
    8abd:	bf 01 00 00 00       	mov    edi,0x1
    8ac2:	e8 15 e2 ff ff       	call   6cdc <cljn_gc_popn>
    8ac7:	e9 97 00 00 00       	jmp    8b63 <cljn_set_conj+0x39e>
    8acc:	48 8b 45 88          	mov    rax,QWORD PTR [rbp-0x78]
    8ad0:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    8ad4:	48 89 45 90          	mov    QWORD PTR [rbp-0x70],rax
    8ad8:	48 8b 45 90          	mov    rax,QWORD PTR [rbp-0x70]
    8adc:	48 83 c0 04          	add    rax,0x4
    8ae0:	48 c1 e0 03          	shl    rax,0x3
    8ae4:	be 07 00 00 00       	mov    esi,0x7
    8ae9:	48 89 c7             	mov    rdi,rax
    8aec:	e8 82 e4 ff ff       	call   6f73 <obj_alloc>
    8af1:	48 89 45 98          	mov    QWORD PTR [rbp-0x68],rax
    8af5:	48 8b 45 90          	mov    rax,QWORD PTR [rbp-0x70]
    8af9:	48 8d 50 01          	lea    rdx,[rax+0x1]
    8afd:	48 8b 45 98          	mov    rax,QWORD PTR [rbp-0x68]
    8b01:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    8b05:	48 c7 45 80 00 00 00 	mov    QWORD PTR [rbp-0x80],0x0
    8b0c:	00 
    8b0d:	eb 27                	jmp    8b36 <cljn_set_conj+0x371>
    8b0f:	48 8b 45 88          	mov    rax,QWORD PTR [rbp-0x78]
    8b13:	48 8b 55 80          	mov    rdx,QWORD PTR [rbp-0x80]
    8b17:	48 83 c2 02          	add    rdx,0x2
    8b1b:	48 8b 54 d0 08       	mov    rdx,QWORD PTR [rax+rdx*8+0x8]
    8b20:	48 8b 45 98          	mov    rax,QWORD PTR [rbp-0x68]
    8b24:	48 8b 4d 80          	mov    rcx,QWORD PTR [rbp-0x80]
    8b28:	48 83 c1 02          	add    rcx,0x2
    8b2c:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    8b31:	48 83 45 80 01       	add    QWORD PTR [rbp-0x80],0x1
    8b36:	48 8b 45 80          	mov    rax,QWORD PTR [rbp-0x80]
    8b3a:	48 3b 45 90          	cmp    rax,QWORD PTR [rbp-0x70]
    8b3e:	7c cf                	jl     8b0f <cljn_set_conj+0x34a>
    8b40:	48 8b 45 98          	mov    rax,QWORD PTR [rbp-0x68]
    8b44:	48 8b 55 90          	mov    rdx,QWORD PTR [rbp-0x70]
    8b48:	48 8d 4a 02          	lea    rcx,[rdx+0x2]
    8b4c:	48 8b 95 50 ff ff ff 	mov    rdx,QWORD PTR [rbp-0xb0]
    8b53:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    8b58:	48 8b 45 98          	mov    rax,QWORD PTR [rbp-0x68]
    8b5c:	48 89 85 70 ff ff ff 	mov    QWORD PTR [rbp-0x90],rax
    8b63:	8b 05 2f b5 00 02    	mov    eax,DWORD PTR [rip+0x200b52f]        # 2014098 <gc_disabled>
    8b69:	83 e8 01             	sub    eax,0x1
    8b6c:	89 05 26 b5 00 02    	mov    DWORD PTR [rip+0x200b526],eax        # 2014098 <gc_disabled>
    8b72:	48 8b 85 70 ff ff ff 	mov    rax,QWORD PTR [rbp-0x90]
    8b79:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    8b7d:	64 48 2b 14 25 28 00 	sub    rdx,QWORD PTR fs:0x28
    8b84:	00 00 
    8b86:	74 05                	je     8b8d <cljn_set_conj+0x3c8>
    8b88:	e8 d3 84 ff ff       	call   1060 <__stack_chk_fail@plt>
    8b8d:	c9                   	leave
    8b8e:	c3                   	ret

0000000000008b8f <cljn_set_contains>:
    8b8f:	f3 0f 1e fa          	endbr64
    8b93:	55                   	push   rbp
    8b94:	48 89 e5             	mov    rbp,rsp
    8b97:	48 83 ec 10          	sub    rsp,0x10
    8b9b:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    8b9f:	48 89 75 f0          	mov    QWORD PTR [rbp-0x10],rsi
    8ba3:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8ba7:	48 89 c7             	mov    rdi,rax
    8baa:	e8 1b e2 ff ff       	call   6dca <obj_type>
    8baf:	83 f8 0d             	cmp    eax,0xd
    8bb2:	75 3f                	jne    8bf3 <cljn_set_contains+0x64>
    8bb4:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    8bb8:	48 89 c7             	mov    rdi,rax
    8bbb:	e8 a4 00 00 00       	call   8c64 <cljn_hash>
    8bc0:	89 c6                	mov    esi,eax
    8bc2:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8bc6:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    8bca:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    8bce:	48 89 d1             	mov    rcx,rdx
    8bd1:	89 f2                	mov    edx,esi
    8bd3:	be 00 00 00 00       	mov    esi,0x0
    8bd8:	48 89 c7             	mov    rdi,rax
    8bdb:	e8 d3 01 00 00       	call   8db3 <node_get>
    8be0:	48 83 f8 2a          	cmp    rax,0x2a
    8be4:	0f 95 c0             	setne  al
    8be7:	0f b6 c0             	movzx  eax,al
    8bea:	89 c7                	mov    edi,eax
    8bec:	e8 a7 47 00 00       	call   d398 <b2v>
    8bf1:	eb 1a                	jmp    8c0d <cljn_set_contains+0x7e>
    8bf3:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8bf7:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    8bfb:	48 89 d6             	mov    rsi,rdx
    8bfe:	48 89 c7             	mov    rdi,rax
    8c01:	e8 6e fa ff ff       	call   8674 <set_member>
    8c06:	89 c7                	mov    edi,eax
    8c08:	e8 8b 47 00 00       	call   d398 <b2v>
    8c0d:	c9                   	leave
    8c0e:	c3                   	ret

0000000000008c0f <hash_bytes>:
    8c0f:	f3 0f 1e fa          	endbr64
    8c13:	55                   	push   rbp
    8c14:	48 89 e5             	mov    rbp,rsp
    8c17:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    8c1b:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    8c1f:	c7 45 f4 c5 9d 1c 81 	mov    DWORD PTR [rbp-0xc],0x811c9dc5
    8c26:	48 c7 45 f8 00 00 00 	mov    QWORD PTR [rbp-0x8],0x0
    8c2d:	00 
    8c2e:	eb 25                	jmp    8c55 <hash_bytes+0x46>
    8c30:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    8c34:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8c38:	48 01 d0             	add    rax,rdx
    8c3b:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    8c3e:	0f b6 c0             	movzx  eax,al
    8c41:	31 45 f4             	xor    DWORD PTR [rbp-0xc],eax
    8c44:	8b 45 f4             	mov    eax,DWORD PTR [rbp-0xc]
    8c47:	69 c0 93 01 00 01    	imul   eax,eax,0x1000193
    8c4d:	89 45 f4             	mov    DWORD PTR [rbp-0xc],eax
    8c50:	48 83 45 f8 01       	add    QWORD PTR [rbp-0x8],0x1
    8c55:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8c59:	48 3b 45 e0          	cmp    rax,QWORD PTR [rbp-0x20]
    8c5d:	72 d1                	jb     8c30 <hash_bytes+0x21>
    8c5f:	8b 45 f4             	mov    eax,DWORD PTR [rbp-0xc]
    8c62:	5d                   	pop    rbp
    8c63:	c3                   	ret

0000000000008c64 <cljn_hash>:
    8c64:	f3 0f 1e fa          	endbr64
    8c68:	55                   	push   rbp
    8c69:	48 89 e5             	mov    rbp,rsp
    8c6c:	48 83 ec 28          	sub    rsp,0x28
    8c70:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    8c74:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    8c78:	83 e0 01             	and    eax,0x1
    8c7b:	48 85 c0             	test   rax,rax
    8c7e:	74 5c                	je     8cdc <cljn_hash+0x78>
    8c80:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    8c84:	48 d1 f8             	sar    rax,1
    8c87:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    8c8b:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8c8f:	48 c1 e8 1e          	shr    rax,0x1e
    8c93:	48 33 45 f8          	xor    rax,QWORD PTR [rbp-0x8]
    8c97:	48 ba b9 e5 e4 1c 6d 	movabs rdx,0xbf58476d1ce4e5b9
    8c9e:	47 58 bf 
    8ca1:	48 0f af c2          	imul   rax,rdx
    8ca5:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    8ca9:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8cad:	48 c1 e8 1b          	shr    rax,0x1b
    8cb1:	48 33 45 f8          	xor    rax,QWORD PTR [rbp-0x8]
    8cb5:	48 ba eb 11 31 13 bb 	movabs rdx,0x94d049bb133111eb
    8cbc:	49 d0 94 
    8cbf:	48 0f af c2          	imul   rax,rdx
    8cc3:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    8cc7:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8ccb:	89 c2                	mov    edx,eax
    8ccd:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8cd1:	48 c1 e8 1f          	shr    rax,0x1f
    8cd5:	31 d0                	xor    eax,edx
    8cd7:	e9 aa 00 00 00       	jmp    8d86 <cljn_hash+0x122>
    8cdc:	48 83 7d d8 02       	cmp    QWORD PTR [rbp-0x28],0x2
    8ce1:	75 0a                	jne    8ced <cljn_hash+0x89>
    8ce3:	b8 00 00 00 00       	mov    eax,0x0
    8ce8:	e9 99 00 00 00       	jmp    8d86 <cljn_hash+0x122>
    8ced:	48 83 7d d8 0a       	cmp    QWORD PTR [rbp-0x28],0xa
    8cf2:	75 0a                	jne    8cfe <cljn_hash+0x9a>
    8cf4:	b8 01 00 00 00       	mov    eax,0x1
    8cf9:	e9 88 00 00 00       	jmp    8d86 <cljn_hash+0x122>
    8cfe:	48 83 7d d8 06       	cmp    QWORD PTR [rbp-0x28],0x6
    8d03:	75 07                	jne    8d0c <cljn_hash+0xa8>
    8d05:	b8 02 00 00 00       	mov    eax,0x2
    8d0a:	eb 7a                	jmp    8d86 <cljn_hash+0x122>
    8d0c:	48 83 7d d8 12       	cmp    QWORD PTR [rbp-0x28],0x12
    8d11:	75 07                	jne    8d1a <cljn_hash+0xb6>
    8d13:	b8 03 00 00 00       	mov    eax,0x3
    8d18:	eb 6c                	jmp    8d86 <cljn_hash+0x122>
    8d1a:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    8d1e:	48 89 c7             	mov    rdi,rax
    8d21:	e8 a4 e0 ff ff       	call   6dca <obj_type>
    8d26:	83 f8 01             	cmp    eax,0x1
    8d29:	74 07                	je     8d32 <cljn_hash+0xce>
    8d2b:	83 f8 04             	cmp    eax,0x4
    8d2e:	74 27                	je     8d57 <cljn_hash+0xf3>
    8d30:	eb 4f                	jmp    8d81 <cljn_hash+0x11d>
    8d32:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    8d36:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    8d3a:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    8d3e:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    8d42:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    8d46:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    8d4a:	48 89 d6             	mov    rsi,rdx
    8d4d:	48 89 c7             	mov    rdi,rax
    8d50:	e8 ba fe ff ff       	call   8c0f <hash_bytes>
    8d55:	eb 2f                	jmp    8d86 <cljn_hash+0x122>
    8d57:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    8d5b:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    8d5f:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    8d63:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    8d67:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    8d6b:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    8d6f:	48 89 d6             	mov    rsi,rdx
    8d72:	48 89 c7             	mov    rdi,rax
    8d75:	e8 95 fe ff ff       	call   8c0f <hash_bytes>
    8d7a:	35 b9 79 37 9e       	xor    eax,0x9e3779b9
    8d7f:	eb 05                	jmp    8d86 <cljn_hash+0x122>
    8d81:	b8 07 00 00 00       	mov    eax,0x7
    8d86:	c9                   	leave
    8d87:	c3                   	ret

0000000000008d88 <mnode_alloc>:
    8d88:	f3 0f 1e fa          	endbr64
    8d8c:	55                   	push   rbp
    8d8d:	48 89 e5             	mov    rbp,rsp
    8d90:	48 83 ec 10          	sub    rsp,0x10
    8d94:	89 7d fc             	mov    DWORD PTR [rbp-0x4],edi
    8d97:	8b 45 fc             	mov    eax,DWORD PTR [rbp-0x4]
    8d9a:	48 98                	cdqe
    8d9c:	48 83 c0 03          	add    rax,0x3
    8da0:	48 c1 e0 03          	shl    rax,0x3
    8da4:	be 0b 00 00 00       	mov    esi,0xb
    8da9:	48 89 c7             	mov    rdi,rax
    8dac:	e8 c2 e1 ff ff       	call   6f73 <obj_alloc>
    8db1:	c9                   	leave
    8db2:	c3                   	ret

0000000000008db3 <node_get>:
    8db3:	f3 0f 1e fa          	endbr64
    8db7:	55                   	push   rbp
    8db8:	48 89 e5             	mov    rbp,rsp
    8dbb:	48 83 ec 50          	sub    rsp,0x50
    8dbf:	48 89 7d c8          	mov    QWORD PTR [rbp-0x38],rdi
    8dc3:	89 75 c4             	mov    DWORD PTR [rbp-0x3c],esi
    8dc6:	89 55 c0             	mov    DWORD PTR [rbp-0x40],edx
    8dc9:	48 89 4d b8          	mov    QWORD PTR [rbp-0x48],rcx
    8dcd:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    8dd1:	48 89 c7             	mov    rdi,rax
    8dd4:	e8 f1 df ff ff       	call   6dca <obj_type>
    8dd9:	83 f8 0c             	cmp    eax,0xc
    8ddc:	75 72                	jne    8e50 <node_get+0x9d>
    8dde:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    8de2:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    8de6:	48 c7 45 e0 00 00 00 	mov    QWORD PTR [rbp-0x20],0x0
    8ded:	00 
    8dee:	eb 48                	jmp    8e38 <node_get+0x85>
    8df0:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    8df4:	48 8d 14 00          	lea    rdx,[rax+rax*1]
    8df8:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8dfc:	48 83 c2 04          	add    rdx,0x4
    8e00:	48 8b 04 d0          	mov    rax,QWORD PTR [rax+rdx*8]
    8e04:	48 8b 55 b8          	mov    rdx,QWORD PTR [rbp-0x48]
    8e08:	48 89 d6             	mov    rsi,rdx
    8e0b:	48 89 c7             	mov    rdi,rax
    8e0e:	e8 f1 47 00 00       	call   d604 <cljn_equal_raw>
    8e13:	85 c0                	test   eax,eax
    8e15:	74 1c                	je     8e33 <node_get+0x80>
    8e17:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    8e1b:	48 01 c0             	add    rax,rax
    8e1e:	48 8d 50 01          	lea    rdx,[rax+0x1]
    8e22:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8e26:	48 83 c2 04          	add    rdx,0x4
    8e2a:	48 8b 04 d0          	mov    rax,QWORD PTR [rax+rdx*8]
    8e2e:	e9 fc 00 00 00       	jmp    8f2f <node_get+0x17c>
    8e33:	48 83 45 e0 01       	add    QWORD PTR [rbp-0x20],0x1
    8e38:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8e3c:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    8e40:	48 39 45 e0          	cmp    QWORD PTR [rbp-0x20],rax
    8e44:	7c aa                	jl     8df0 <node_get+0x3d>
    8e46:	b8 2a 00 00 00       	mov    eax,0x2a
    8e4b:	e9 df 00 00 00       	jmp    8f2f <node_get+0x17c>
    8e50:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    8e54:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    8e58:	8b 45 c4             	mov    eax,DWORD PTR [rbp-0x3c]
    8e5b:	8b 55 c0             	mov    edx,DWORD PTR [rbp-0x40]
    8e5e:	89 c1                	mov    ecx,eax
    8e60:	d3 ea                	shr    edx,cl
    8e62:	89 d0                	mov    eax,edx
    8e64:	83 e0 1f             	and    eax,0x1f
    8e67:	ba 01 00 00 00       	mov    edx,0x1
    8e6c:	89 c1                	mov    ecx,eax
    8e6e:	d3 e2                	shl    edx,cl
    8e70:	89 d0                	mov    eax,edx
    8e72:	89 45 d8             	mov    DWORD PTR [rbp-0x28],eax
    8e75:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    8e79:	8b 40 10             	mov    eax,DWORD PTR [rax+0x10]
    8e7c:	23 45 d8             	and    eax,DWORD PTR [rbp-0x28]
    8e7f:	85 c0                	test   eax,eax
    8e81:	75 0a                	jne    8e8d <node_get+0xda>
    8e83:	b8 2a 00 00 00       	mov    eax,0x2a
    8e88:	e9 a2 00 00 00       	jmp    8f2f <node_get+0x17c>
    8e8d:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    8e91:	8b 40 10             	mov    eax,DWORD PTR [rax+0x10]
    8e94:	8b 55 d8             	mov    edx,DWORD PTR [rbp-0x28]
    8e97:	83 ea 01             	sub    edx,0x1
    8e9a:	21 d0                	and    eax,edx
    8e9c:	89 c0                	mov    eax,eax
    8e9e:	48 89 c7             	mov    rdi,rax
    8ea1:	e8 0a 6c 00 00       	call   fab0 <__popcountdi2>
    8ea6:	89 45 dc             	mov    DWORD PTR [rbp-0x24],eax
    8ea9:	8b 45 dc             	mov    eax,DWORD PTR [rbp-0x24]
    8eac:	8d 14 00             	lea    edx,[rax+rax*1]
    8eaf:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    8eb3:	48 63 d2             	movsxd rdx,edx
    8eb6:	48 83 c2 02          	add    rdx,0x2
    8eba:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    8ebf:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    8ec3:	48 83 7d f0 1a       	cmp    QWORD PTR [rbp-0x10],0x1a
    8ec8:	75 2f                	jne    8ef9 <node_get+0x146>
    8eca:	8b 45 c4             	mov    eax,DWORD PTR [rbp-0x3c]
    8ecd:	8d 70 05             	lea    esi,[rax+0x5]
    8ed0:	8b 45 dc             	mov    eax,DWORD PTR [rbp-0x24]
    8ed3:	01 c0                	add    eax,eax
    8ed5:	8d 50 01             	lea    edx,[rax+0x1]
    8ed8:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    8edc:	48 63 d2             	movsxd rdx,edx
    8edf:	48 83 c2 02          	add    rdx,0x2
    8ee3:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    8ee8:	48 8b 4d b8          	mov    rcx,QWORD PTR [rbp-0x48]
    8eec:	8b 55 c0             	mov    edx,DWORD PTR [rbp-0x40]
    8eef:	48 89 c7             	mov    rdi,rax
    8ef2:	e8 bc fe ff ff       	call   8db3 <node_get>
    8ef7:	eb 36                	jmp    8f2f <node_get+0x17c>
    8ef9:	48 8b 55 b8          	mov    rdx,QWORD PTR [rbp-0x48]
    8efd:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    8f01:	48 89 d6             	mov    rsi,rdx
    8f04:	48 89 c7             	mov    rdi,rax
    8f07:	e8 f8 46 00 00       	call   d604 <cljn_equal_raw>
    8f0c:	85 c0                	test   eax,eax
    8f0e:	74 1a                	je     8f2a <node_get+0x177>
    8f10:	8b 45 dc             	mov    eax,DWORD PTR [rbp-0x24]
    8f13:	01 c0                	add    eax,eax
    8f15:	8d 50 01             	lea    edx,[rax+0x1]
    8f18:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    8f1c:	48 63 d2             	movsxd rdx,edx
    8f1f:	48 83 c2 02          	add    rdx,0x2
    8f23:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    8f28:	eb 05                	jmp    8f2f <node_get+0x17c>
    8f2a:	b8 2a 00 00 00       	mov    eax,0x2a
    8f2f:	c9                   	leave
    8f30:	c3                   	ret

0000000000008f31 <merge_two>:
    8f31:	f3 0f 1e fa          	endbr64
    8f35:	55                   	push   rbp
    8f36:	48 89 e5             	mov    rbp,rsp
    8f39:	48 83 ec 60          	sub    rsp,0x60
    8f3d:	89 7d cc             	mov    DWORD PTR [rbp-0x34],edi
    8f40:	89 75 c8             	mov    DWORD PTR [rbp-0x38],esi
    8f43:	48 89 55 c0          	mov    QWORD PTR [rbp-0x40],rdx
    8f47:	48 89 4d b8          	mov    QWORD PTR [rbp-0x48],rcx
    8f4b:	44 89 45 b4          	mov    DWORD PTR [rbp-0x4c],r8d
    8f4f:	4c 89 4d a8          	mov    QWORD PTR [rbp-0x58],r9
    8f53:	83 7d cc 1f          	cmp    DWORD PTR [rbp-0x34],0x1f
    8f57:	76 62                	jbe    8fbb <merge_two+0x8a>
    8f59:	be 0c 00 00 00       	mov    esi,0xc
    8f5e:	bf 40 00 00 00       	mov    edi,0x40
    8f63:	e8 0b e0 ff ff       	call   6f73 <obj_alloc>
    8f68:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    8f6c:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8f70:	8b 55 c8             	mov    edx,DWORD PTR [rbp-0x38]
    8f73:	89 50 10             	mov    DWORD PTR [rax+0x10],edx
    8f76:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8f7a:	48 c7 40 18 02 00 00 	mov    QWORD PTR [rax+0x18],0x2
    8f81:	00 
    8f82:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8f86:	48 8b 55 c0          	mov    rdx,QWORD PTR [rbp-0x40]
    8f8a:	48 89 50 20          	mov    QWORD PTR [rax+0x20],rdx
    8f8e:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8f92:	48 8b 55 b8          	mov    rdx,QWORD PTR [rbp-0x48]
    8f96:	48 89 50 28          	mov    QWORD PTR [rax+0x28],rdx
    8f9a:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8f9e:	48 8b 55 a8          	mov    rdx,QWORD PTR [rbp-0x58]
    8fa2:	48 89 50 30          	mov    QWORD PTR [rax+0x30],rdx
    8fa6:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8faa:	48 8b 55 10          	mov    rdx,QWORD PTR [rbp+0x10]
    8fae:	48 89 50 38          	mov    QWORD PTR [rax+0x38],rdx
    8fb2:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    8fb6:	e9 41 01 00 00       	jmp    90fc <merge_two+0x1cb>
    8fbb:	8b 45 cc             	mov    eax,DWORD PTR [rbp-0x34]
    8fbe:	8b 55 c8             	mov    edx,DWORD PTR [rbp-0x38]
    8fc1:	89 c1                	mov    ecx,eax
    8fc3:	d3 ea                	shr    edx,cl
    8fc5:	89 d0                	mov    eax,edx
    8fc7:	83 e0 1f             	and    eax,0x1f
    8fca:	89 45 d8             	mov    DWORD PTR [rbp-0x28],eax
    8fcd:	8b 45 cc             	mov    eax,DWORD PTR [rbp-0x34]
    8fd0:	8b 55 b4             	mov    edx,DWORD PTR [rbp-0x4c]
    8fd3:	89 c1                	mov    ecx,eax
    8fd5:	d3 ea                	shr    edx,cl
    8fd7:	89 d0                	mov    eax,edx
    8fd9:	83 e0 1f             	and    eax,0x1f
    8fdc:	89 45 dc             	mov    DWORD PTR [rbp-0x24],eax
    8fdf:	8b 45 d8             	mov    eax,DWORD PTR [rbp-0x28]
    8fe2:	3b 45 dc             	cmp    eax,DWORD PTR [rbp-0x24]
    8fe5:	75 76                	jne    905d <merge_two+0x12c>
    8fe7:	8b 45 cc             	mov    eax,DWORD PTR [rbp-0x34]
    8fea:	8d 78 05             	lea    edi,[rax+0x5]
    8fed:	4c 8b 45 a8          	mov    r8,QWORD PTR [rbp-0x58]
    8ff1:	8b 75 b4             	mov    esi,DWORD PTR [rbp-0x4c]
    8ff4:	48 8b 4d b8          	mov    rcx,QWORD PTR [rbp-0x48]
    8ff8:	48 8b 55 c0          	mov    rdx,QWORD PTR [rbp-0x40]
    8ffc:	8b 45 c8             	mov    eax,DWORD PTR [rbp-0x38]
    8fff:	48 83 ec 08          	sub    rsp,0x8
    9003:	ff 75 10             	push   QWORD PTR [rbp+0x10]
    9006:	4d 89 c1             	mov    r9,r8
    9009:	41 89 f0             	mov    r8d,esi
    900c:	89 c6                	mov    esi,eax
    900e:	e8 1e ff ff ff       	call   8f31 <merge_two>
    9013:	48 83 c4 10          	add    rsp,0x10
    9017:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    901b:	bf 02 00 00 00       	mov    edi,0x2
    9020:	e8 63 fd ff ff       	call   8d88 <mnode_alloc>
    9025:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    9029:	8b 45 d8             	mov    eax,DWORD PTR [rbp-0x28]
    902c:	ba 01 00 00 00       	mov    edx,0x1
    9031:	89 c1                	mov    ecx,eax
    9033:	d3 e2                	shl    edx,cl
    9035:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    9039:	89 50 10             	mov    DWORD PTR [rax+0x10],edx
    903c:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    9040:	48 c7 40 18 1a 00 00 	mov    QWORD PTR [rax+0x18],0x1a
    9047:	00 
    9048:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    904c:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    9050:	48 89 50 20          	mov    QWORD PTR [rax+0x20],rdx
    9054:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    9058:	e9 9f 00 00 00       	jmp    90fc <merge_two+0x1cb>
    905d:	bf 04 00 00 00       	mov    edi,0x4
    9062:	e8 21 fd ff ff       	call   8d88 <mnode_alloc>
    9067:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    906b:	8b 45 d8             	mov    eax,DWORD PTR [rbp-0x28]
    906e:	ba 01 00 00 00       	mov    edx,0x1
    9073:	89 c1                	mov    ecx,eax
    9075:	d3 e2                	shl    edx,cl
    9077:	8b 45 dc             	mov    eax,DWORD PTR [rbp-0x24]
    907a:	be 01 00 00 00       	mov    esi,0x1
    907f:	89 c1                	mov    ecx,eax
    9081:	d3 e6                	shl    esi,cl
    9083:	89 f0                	mov    eax,esi
    9085:	09 c2                	or     edx,eax
    9087:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    908b:	89 50 10             	mov    DWORD PTR [rax+0x10],edx
    908e:	8b 45 d8             	mov    eax,DWORD PTR [rbp-0x28]
    9091:	3b 45 dc             	cmp    eax,DWORD PTR [rbp-0x24]
    9094:	7d 32                	jge    90c8 <merge_two+0x197>
    9096:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    909a:	48 8b 55 c0          	mov    rdx,QWORD PTR [rbp-0x40]
    909e:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    90a2:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    90a6:	48 8b 55 b8          	mov    rdx,QWORD PTR [rbp-0x48]
    90aa:	48 89 50 20          	mov    QWORD PTR [rax+0x20],rdx
    90ae:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    90b2:	48 8b 55 a8          	mov    rdx,QWORD PTR [rbp-0x58]
    90b6:	48 89 50 28          	mov    QWORD PTR [rax+0x28],rdx
    90ba:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    90be:	48 8b 55 10          	mov    rdx,QWORD PTR [rbp+0x10]
    90c2:	48 89 50 30          	mov    QWORD PTR [rax+0x30],rdx
    90c6:	eb 30                	jmp    90f8 <merge_two+0x1c7>
    90c8:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    90cc:	48 8b 55 a8          	mov    rdx,QWORD PTR [rbp-0x58]
    90d0:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    90d4:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    90d8:	48 8b 55 10          	mov    rdx,QWORD PTR [rbp+0x10]
    90dc:	48 89 50 20          	mov    QWORD PTR [rax+0x20],rdx
    90e0:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    90e4:	48 8b 55 c0          	mov    rdx,QWORD PTR [rbp-0x40]
    90e8:	48 89 50 28          	mov    QWORD PTR [rax+0x28],rdx
    90ec:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    90f0:	48 8b 55 b8          	mov    rdx,QWORD PTR [rbp-0x48]
    90f4:	48 89 50 30          	mov    QWORD PTR [rax+0x30],rdx
    90f8:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    90fc:	c9                   	leave
    90fd:	c3                   	ret

00000000000090fe <node_assoc>:
    90fe:	f3 0f 1e fa          	endbr64
    9102:	55                   	push   rbp
    9103:	48 89 e5             	mov    rbp,rsp
    9106:	53                   	push   rbx
    9107:	48 81 ec a8 00 00 00 	sub    rsp,0xa8
    910e:	48 89 bd 78 ff ff ff 	mov    QWORD PTR [rbp-0x88],rdi
    9115:	89 b5 74 ff ff ff    	mov    DWORD PTR [rbp-0x8c],esi
    911b:	89 95 70 ff ff ff    	mov    DWORD PTR [rbp-0x90],edx
    9121:	48 89 8d 68 ff ff ff 	mov    QWORD PTR [rbp-0x98],rcx
    9128:	4c 89 85 60 ff ff ff 	mov    QWORD PTR [rbp-0xa0],r8
    912f:	4c 89 8d 58 ff ff ff 	mov    QWORD PTR [rbp-0xa8],r9
    9136:	48 8b 85 78 ff ff ff 	mov    rax,QWORD PTR [rbp-0x88]
    913d:	48 89 c7             	mov    rdi,rax
    9140:	e8 85 dc ff ff       	call   6dca <obj_type>
    9145:	83 f8 0c             	cmp    eax,0xc
    9148:	0f 85 ea 01 00 00    	jne    9338 <node_assoc+0x23a>
    914e:	48 8b 85 78 ff ff ff 	mov    rax,QWORD PTR [rbp-0x88]
    9155:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    9159:	48 c7 45 98 00 00 00 	mov    QWORD PTR [rbp-0x68],0x0
    9160:	00 
    9161:	e9 e6 00 00 00       	jmp    924c <node_assoc+0x14e>
    9166:	48 8b 45 98          	mov    rax,QWORD PTR [rbp-0x68]
    916a:	48 8d 14 00          	lea    rdx,[rax+rax*1]
    916e:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    9172:	48 83 c2 04          	add    rdx,0x4
    9176:	48 8b 04 d0          	mov    rax,QWORD PTR [rax+rdx*8]
    917a:	48 8b 95 68 ff ff ff 	mov    rdx,QWORD PTR [rbp-0x98]
    9181:	48 89 d6             	mov    rsi,rdx
    9184:	48 89 c7             	mov    rdi,rax
    9187:	e8 78 44 00 00       	call   d604 <cljn_equal_raw>
    918c:	85 c0                	test   eax,eax
    918e:	0f 84 b3 00 00 00    	je     9247 <node_assoc+0x149>
    9194:	48 8b 85 58 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa8]
    919b:	c7 00 00 00 00 00    	mov    DWORD PTR [rax],0x0
    91a1:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    91a5:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    91a9:	48 83 c0 02          	add    rax,0x2
    91ad:	48 c1 e0 04          	shl    rax,0x4
    91b1:	be 0c 00 00 00       	mov    esi,0xc
    91b6:	48 89 c7             	mov    rdi,rax
    91b9:	e8 b5 dd ff ff       	call   6f73 <obj_alloc>
    91be:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    91c2:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    91c6:	8b 50 10             	mov    edx,DWORD PTR [rax+0x10]
    91c9:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    91cd:	89 50 10             	mov    DWORD PTR [rax+0x10],edx
    91d0:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    91d4:	48 8b 50 18          	mov    rdx,QWORD PTR [rax+0x18]
    91d8:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    91dc:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    91e0:	48 c7 45 a0 00 00 00 	mov    QWORD PTR [rbp-0x60],0x0
    91e7:	00 
    91e8:	eb 25                	jmp    920f <node_assoc+0x111>
    91ea:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    91ee:	48 8b 55 a0          	mov    rdx,QWORD PTR [rbp-0x60]
    91f2:	48 83 c2 04          	add    rdx,0x4
    91f6:	48 8b 14 d0          	mov    rdx,QWORD PTR [rax+rdx*8]
    91fa:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    91fe:	48 8b 4d a0          	mov    rcx,QWORD PTR [rbp-0x60]
    9202:	48 83 c1 04          	add    rcx,0x4
    9206:	48 89 14 c8          	mov    QWORD PTR [rax+rcx*8],rdx
    920a:	48 83 45 a0 01       	add    QWORD PTR [rbp-0x60],0x1
    920f:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    9213:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    9217:	48 01 c0             	add    rax,rax
    921a:	48 39 45 a0          	cmp    QWORD PTR [rbp-0x60],rax
    921e:	7c ca                	jl     91ea <node_assoc+0xec>
    9220:	48 8b 45 98          	mov    rax,QWORD PTR [rbp-0x68]
    9224:	48 01 c0             	add    rax,rax
    9227:	48 8d 50 01          	lea    rdx,[rax+0x1]
    922b:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    922f:	48 8d 4a 04          	lea    rcx,[rdx+0x4]
    9233:	48 8b 95 60 ff ff ff 	mov    rdx,QWORD PTR [rbp-0xa0]
    923a:	48 89 14 c8          	mov    QWORD PTR [rax+rcx*8],rdx
    923e:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    9242:	e9 2f 04 00 00       	jmp    9676 <node_assoc+0x578>
    9247:	48 83 45 98 01       	add    QWORD PTR [rbp-0x68],0x1
    924c:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    9250:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    9254:	48 39 45 98          	cmp    QWORD PTR [rbp-0x68],rax
    9258:	0f 8c 08 ff ff ff    	jl     9166 <node_assoc+0x68>
    925e:	48 8b 85 58 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa8]
    9265:	c7 00 01 00 00 00    	mov    DWORD PTR [rax],0x1
    926b:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    926f:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    9273:	48 83 c0 03          	add    rax,0x3
    9277:	48 c1 e0 04          	shl    rax,0x4
    927b:	be 0c 00 00 00       	mov    esi,0xc
    9280:	48 89 c7             	mov    rdi,rax
    9283:	e8 eb dc ff ff       	call   6f73 <obj_alloc>
    9288:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    928c:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    9290:	8b 50 10             	mov    edx,DWORD PTR [rax+0x10]
    9293:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    9297:	89 50 10             	mov    DWORD PTR [rax+0x10],edx
    929a:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    929e:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    92a2:	48 8d 50 01          	lea    rdx,[rax+0x1]
    92a6:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    92aa:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    92ae:	48 c7 45 a8 00 00 00 	mov    QWORD PTR [rbp-0x58],0x0
    92b5:	00 
    92b6:	eb 25                	jmp    92dd <node_assoc+0x1df>
    92b8:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    92bc:	48 8b 55 a8          	mov    rdx,QWORD PTR [rbp-0x58]
    92c0:	48 83 c2 04          	add    rdx,0x4
    92c4:	48 8b 14 d0          	mov    rdx,QWORD PTR [rax+rdx*8]
    92c8:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    92cc:	48 8b 4d a8          	mov    rcx,QWORD PTR [rbp-0x58]
    92d0:	48 83 c1 04          	add    rcx,0x4
    92d4:	48 89 14 c8          	mov    QWORD PTR [rax+rcx*8],rdx
    92d8:	48 83 45 a8 01       	add    QWORD PTR [rbp-0x58],0x1
    92dd:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    92e1:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    92e5:	48 01 c0             	add    rax,rax
    92e8:	48 39 45 a8          	cmp    QWORD PTR [rbp-0x58],rax
    92ec:	7c ca                	jl     92b8 <node_assoc+0x1ba>
    92ee:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    92f2:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    92f6:	48 8d 14 00          	lea    rdx,[rax+rax*1]
    92fa:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    92fe:	48 8d 4a 04          	lea    rcx,[rdx+0x4]
    9302:	48 8b 95 68 ff ff ff 	mov    rdx,QWORD PTR [rbp-0x98]
    9309:	48 89 14 c8          	mov    QWORD PTR [rax+rcx*8],rdx
    930d:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    9311:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    9315:	48 01 c0             	add    rax,rax
    9318:	48 8d 50 01          	lea    rdx,[rax+0x1]
    931c:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    9320:	48 8d 4a 04          	lea    rcx,[rdx+0x4]
    9324:	48 8b 95 60 ff ff ff 	mov    rdx,QWORD PTR [rbp-0xa0]
    932b:	48 89 14 c8          	mov    QWORD PTR [rax+rcx*8],rdx
    932f:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    9333:	e9 3e 03 00 00       	jmp    9676 <node_assoc+0x578>
    9338:	48 8b 85 78 ff ff ff 	mov    rax,QWORD PTR [rbp-0x88]
    933f:	48 89 45 b0          	mov    QWORD PTR [rbp-0x50],rax
    9343:	8b 85 74 ff ff ff    	mov    eax,DWORD PTR [rbp-0x8c]
    9349:	8b 95 70 ff ff ff    	mov    edx,DWORD PTR [rbp-0x90]
    934f:	89 c1                	mov    ecx,eax
    9351:	d3 ea                	shr    edx,cl
    9353:	89 d0                	mov    eax,edx
    9355:	83 e0 1f             	and    eax,0x1f
    9358:	ba 01 00 00 00       	mov    edx,0x1
    935d:	89 c1                	mov    ecx,eax
    935f:	d3 e2                	shl    edx,cl
    9361:	89 d0                	mov    eax,edx
    9363:	89 45 8c             	mov    DWORD PTR [rbp-0x74],eax
    9366:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    936a:	8b 40 10             	mov    eax,DWORD PTR [rax+0x10]
    936d:	8b 55 8c             	mov    edx,DWORD PTR [rbp-0x74]
    9370:	83 ea 01             	sub    edx,0x1
    9373:	21 d0                	and    eax,edx
    9375:	89 c0                	mov    eax,eax
    9377:	48 89 c7             	mov    rdi,rax
    937a:	e8 31 67 00 00       	call   fab0 <__popcountdi2>
    937f:	89 45 90             	mov    DWORD PTR [rbp-0x70],eax
    9382:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    9386:	8b 40 10             	mov    eax,DWORD PTR [rax+0x10]
    9389:	89 c0                	mov    eax,eax
    938b:	48 89 c7             	mov    rdi,rax
    938e:	e8 1d 67 00 00       	call   fab0 <__popcountdi2>
    9393:	89 45 94             	mov    DWORD PTR [rbp-0x6c],eax
    9396:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    939a:	8b 40 10             	mov    eax,DWORD PTR [rax+0x10]
    939d:	23 45 8c             	and    eax,DWORD PTR [rbp-0x74]
    93a0:	85 c0                	test   eax,eax
    93a2:	0f 84 dd 01 00 00    	je     9585 <node_assoc+0x487>
    93a8:	8b 45 90             	mov    eax,DWORD PTR [rbp-0x70]
    93ab:	8d 14 00             	lea    edx,[rax+rax*1]
    93ae:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    93b2:	48 63 d2             	movsxd rdx,edx
    93b5:	48 83 c2 02          	add    rdx,0x2
    93b9:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    93be:	48 89 45 c0          	mov    QWORD PTR [rbp-0x40],rax
    93c2:	8b 45 94             	mov    eax,DWORD PTR [rbp-0x6c]
    93c5:	01 c0                	add    eax,eax
    93c7:	89 c7                	mov    edi,eax
    93c9:	e8 ba f9 ff ff       	call   8d88 <mnode_alloc>
    93ce:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
    93d2:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    93d6:	8b 50 10             	mov    edx,DWORD PTR [rax+0x10]
    93d9:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    93dd:	89 50 10             	mov    DWORD PTR [rax+0x10],edx
    93e0:	c7 45 80 00 00 00 00 	mov    DWORD PTR [rbp-0x80],0x0
    93e7:	eb 2a                	jmp    9413 <node_assoc+0x315>
    93e9:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    93ed:	8b 55 80             	mov    edx,DWORD PTR [rbp-0x80]
    93f0:	48 63 d2             	movsxd rdx,edx
    93f3:	48 83 c2 02          	add    rdx,0x2
    93f7:	48 8b 54 d0 08       	mov    rdx,QWORD PTR [rax+rdx*8+0x8]
    93fc:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    9400:	8b 4d 80             	mov    ecx,DWORD PTR [rbp-0x80]
    9403:	48 63 c9             	movsxd rcx,ecx
    9406:	48 83 c1 02          	add    rcx,0x2
    940a:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    940f:	83 45 80 01          	add    DWORD PTR [rbp-0x80],0x1
    9413:	8b 45 94             	mov    eax,DWORD PTR [rbp-0x6c]
    9416:	01 c0                	add    eax,eax
    9418:	39 45 80             	cmp    DWORD PTR [rbp-0x80],eax
    941b:	7c cc                	jl     93e9 <node_assoc+0x2eb>
    941d:	48 83 7d c0 1a       	cmp    QWORD PTR [rbp-0x40],0x1a
    9422:	75 67                	jne    948b <node_assoc+0x38d>
    9424:	8b 85 74 ff ff ff    	mov    eax,DWORD PTR [rbp-0x8c]
    942a:	8d 70 05             	lea    esi,[rax+0x5]
    942d:	8b 45 90             	mov    eax,DWORD PTR [rbp-0x70]
    9430:	01 c0                	add    eax,eax
    9432:	8d 50 01             	lea    edx,[rax+0x1]
    9435:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    9439:	48 63 d2             	movsxd rdx,edx
    943c:	48 83 c2 02          	add    rdx,0x2
    9440:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    9445:	8b 55 90             	mov    edx,DWORD PTR [rbp-0x70]
    9448:	01 d2                	add    edx,edx
    944a:	8d 5a 01             	lea    ebx,[rdx+0x1]
    944d:	4c 8b 85 58 ff ff ff 	mov    r8,QWORD PTR [rbp-0xa8]
    9454:	48 8b bd 60 ff ff ff 	mov    rdi,QWORD PTR [rbp-0xa0]
    945b:	48 8b 8d 68 ff ff ff 	mov    rcx,QWORD PTR [rbp-0x98]
    9462:	8b 95 70 ff ff ff    	mov    edx,DWORD PTR [rbp-0x90]
    9468:	4d 89 c1             	mov    r9,r8
    946b:	49 89 f8             	mov    r8,rdi
    946e:	48 89 c7             	mov    rdi,rax
    9471:	e8 88 fc ff ff       	call   90fe <node_assoc>
    9476:	48 8b 55 c8          	mov    rdx,QWORD PTR [rbp-0x38]
    947a:	48 63 cb             	movsxd rcx,ebx
    947d:	48 83 c1 02          	add    rcx,0x2
    9481:	48 89 44 ca 08       	mov    QWORD PTR [rdx+rcx*8+0x8],rax
    9486:	e9 f1 00 00 00       	jmp    957c <node_assoc+0x47e>
    948b:	48 8b 95 68 ff ff ff 	mov    rdx,QWORD PTR [rbp-0x98]
    9492:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    9496:	48 89 d6             	mov    rsi,rdx
    9499:	48 89 c7             	mov    rdi,rax
    949c:	e8 63 41 00 00       	call   d604 <cljn_equal_raw>
    94a1:	85 c0                	test   eax,eax
    94a3:	74 31                	je     94d6 <node_assoc+0x3d8>
    94a5:	48 8b 85 58 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa8]
    94ac:	c7 00 00 00 00 00    	mov    DWORD PTR [rax],0x0
    94b2:	8b 45 90             	mov    eax,DWORD PTR [rbp-0x70]
    94b5:	01 c0                	add    eax,eax
    94b7:	8d 50 01             	lea    edx,[rax+0x1]
    94ba:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    94be:	48 63 d2             	movsxd rdx,edx
    94c1:	48 8d 4a 02          	lea    rcx,[rdx+0x2]
    94c5:	48 8b 95 60 ff ff ff 	mov    rdx,QWORD PTR [rbp-0xa0]
    94cc:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    94d1:	e9 a6 00 00 00       	jmp    957c <node_assoc+0x47e>
    94d6:	48 8b 85 58 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa8]
    94dd:	c7 00 01 00 00 00    	mov    DWORD PTR [rax],0x1
    94e3:	8b 45 90             	mov    eax,DWORD PTR [rbp-0x70]
    94e6:	01 c0                	add    eax,eax
    94e8:	8d 50 01             	lea    edx,[rax+0x1]
    94eb:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    94ef:	48 63 d2             	movsxd rdx,edx
    94f2:	48 83 c2 02          	add    rdx,0x2
    94f6:	48 8b 5c d0 08       	mov    rbx,QWORD PTR [rax+rdx*8+0x8]
    94fb:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    94ff:	48 89 c7             	mov    rdi,rax
    9502:	e8 5d f7 ff ff       	call   8c64 <cljn_hash>
    9507:	89 c6                	mov    esi,eax
    9509:	8b 85 74 ff ff ff    	mov    eax,DWORD PTR [rbp-0x8c]
    950f:	8d 78 05             	lea    edi,[rax+0x5]
    9512:	48 8b 8d 68 ff ff ff 	mov    rcx,QWORD PTR [rbp-0x98]
    9519:	8b 95 70 ff ff ff    	mov    edx,DWORD PTR [rbp-0x90]
    951f:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    9523:	48 83 ec 08          	sub    rsp,0x8
    9527:	ff b5 60 ff ff ff    	push   QWORD PTR [rbp-0xa0]
    952d:	49 89 c9             	mov    r9,rcx
    9530:	41 89 d0             	mov    r8d,edx
    9533:	48 89 d9             	mov    rcx,rbx
    9536:	48 89 c2             	mov    rdx,rax
    9539:	e8 f3 f9 ff ff       	call   8f31 <merge_two>
    953e:	48 83 c4 10          	add    rsp,0x10
    9542:	48 89 45 d0          	mov    QWORD PTR [rbp-0x30],rax
    9546:	8b 45 90             	mov    eax,DWORD PTR [rbp-0x70]
    9549:	8d 14 00             	lea    edx,[rax+rax*1]
    954c:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    9550:	48 63 d2             	movsxd rdx,edx
    9553:	48 83 c2 02          	add    rdx,0x2
    9557:	48 c7 44 d0 08 1a 00 	mov    QWORD PTR [rax+rdx*8+0x8],0x1a
    955e:	00 00 
    9560:	8b 45 90             	mov    eax,DWORD PTR [rbp-0x70]
    9563:	01 c0                	add    eax,eax
    9565:	8d 50 01             	lea    edx,[rax+0x1]
    9568:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    956c:	48 63 d2             	movsxd rdx,edx
    956f:	48 8d 4a 02          	lea    rcx,[rdx+0x2]
    9573:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    9577:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    957c:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    9580:	e9 f1 00 00 00       	jmp    9676 <node_assoc+0x578>
    9585:	48 8b 85 58 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa8]
    958c:	c7 00 01 00 00 00    	mov    DWORD PTR [rax],0x1
    9592:	8b 45 94             	mov    eax,DWORD PTR [rbp-0x6c]
    9595:	83 c0 01             	add    eax,0x1
    9598:	01 c0                	add    eax,eax
    959a:	89 c7                	mov    edi,eax
    959c:	e8 e7 f7 ff ff       	call   8d88 <mnode_alloc>
    95a1:	48 89 45 b8          	mov    QWORD PTR [rbp-0x48],rax
    95a5:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    95a9:	8b 40 10             	mov    eax,DWORD PTR [rax+0x10]
    95ac:	0b 45 8c             	or     eax,DWORD PTR [rbp-0x74]
    95af:	89 c2                	mov    edx,eax
    95b1:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    95b5:	89 50 10             	mov    DWORD PTR [rax+0x10],edx
    95b8:	c7 45 84 00 00 00 00 	mov    DWORD PTR [rbp-0x7c],0x0
    95bf:	eb 2a                	jmp    95eb <node_assoc+0x4ed>
    95c1:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    95c5:	8b 55 84             	mov    edx,DWORD PTR [rbp-0x7c]
    95c8:	48 63 d2             	movsxd rdx,edx
    95cb:	48 83 c2 02          	add    rdx,0x2
    95cf:	48 8b 54 d0 08       	mov    rdx,QWORD PTR [rax+rdx*8+0x8]
    95d4:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    95d8:	8b 4d 84             	mov    ecx,DWORD PTR [rbp-0x7c]
    95db:	48 63 c9             	movsxd rcx,ecx
    95de:	48 83 c1 02          	add    rcx,0x2
    95e2:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    95e7:	83 45 84 01          	add    DWORD PTR [rbp-0x7c],0x1
    95eb:	8b 45 90             	mov    eax,DWORD PTR [rbp-0x70]
    95ee:	01 c0                	add    eax,eax
    95f0:	39 45 84             	cmp    DWORD PTR [rbp-0x7c],eax
    95f3:	7c cc                	jl     95c1 <node_assoc+0x4c3>
    95f5:	8b 45 90             	mov    eax,DWORD PTR [rbp-0x70]
    95f8:	8d 14 00             	lea    edx,[rax+rax*1]
    95fb:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    95ff:	48 63 d2             	movsxd rdx,edx
    9602:	48 8d 4a 02          	lea    rcx,[rdx+0x2]
    9606:	48 8b 95 68 ff ff ff 	mov    rdx,QWORD PTR [rbp-0x98]
    960d:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    9612:	8b 45 90             	mov    eax,DWORD PTR [rbp-0x70]
    9615:	01 c0                	add    eax,eax
    9617:	8d 50 01             	lea    edx,[rax+0x1]
    961a:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    961e:	48 63 d2             	movsxd rdx,edx
    9621:	48 8d 4a 02          	lea    rcx,[rdx+0x2]
    9625:	48 8b 95 60 ff ff ff 	mov    rdx,QWORD PTR [rbp-0xa0]
    962c:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    9631:	8b 45 90             	mov    eax,DWORD PTR [rbp-0x70]
    9634:	01 c0                	add    eax,eax
    9636:	89 45 88             	mov    DWORD PTR [rbp-0x78],eax
    9639:	eb 2d                	jmp    9668 <node_assoc+0x56a>
    963b:	8b 45 88             	mov    eax,DWORD PTR [rbp-0x78]
    963e:	8d 48 02             	lea    ecx,[rax+0x2]
    9641:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    9645:	8b 55 88             	mov    edx,DWORD PTR [rbp-0x78]
    9648:	48 63 d2             	movsxd rdx,edx
    964b:	48 83 c2 02          	add    rdx,0x2
    964f:	48 8b 54 d0 08       	mov    rdx,QWORD PTR [rax+rdx*8+0x8]
    9654:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    9658:	48 63 c9             	movsxd rcx,ecx
    965b:	48 83 c1 02          	add    rcx,0x2
    965f:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    9664:	83 45 88 01          	add    DWORD PTR [rbp-0x78],0x1
    9668:	8b 45 94             	mov    eax,DWORD PTR [rbp-0x6c]
    966b:	01 c0                	add    eax,eax
    966d:	39 45 88             	cmp    DWORD PTR [rbp-0x78],eax
    9670:	7c c9                	jl     963b <node_assoc+0x53d>
    9672:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    9676:	48 8b 5d f8          	mov    rbx,QWORD PTR [rbp-0x8]
    967a:	c9                   	leave
    967b:	c3                   	ret

000000000000967c <hmap_cons_walk>:
    967c:	f3 0f 1e fa          	endbr64
    9680:	55                   	push   rbp
    9681:	48 89 e5             	mov    rbp,rsp
    9684:	48 83 ec 50          	sub    rsp,0x50
    9688:	48 89 7d b8          	mov    QWORD PTR [rbp-0x48],rdi
    968c:	89 75 b4             	mov    DWORD PTR [rbp-0x4c],esi
    968f:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    9693:	48 89 c7             	mov    rdi,rax
    9696:	e8 2f d7 ff ff       	call   6dca <obj_type>
    969b:	83 f8 0c             	cmp    eax,0xc
    969e:	0f 85 95 00 00 00    	jne    9739 <hmap_cons_walk+0xbd>
    96a4:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    96a8:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    96ac:	48 c7 45 d0 00 00 00 	mov    QWORD PTR [rbp-0x30],0x0
    96b3:	00 
    96b4:	eb 70                	jmp    9726 <hmap_cons_walk+0xaa>
    96b6:	48 8b 05 c3 a9 00 02 	mov    rax,QWORD PTR [rip+0x200a9c3]        # 2014080 <gc_sp>
    96bd:	48 83 e8 01          	sub    rax,0x1
    96c1:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    96c8:	00 
    96c9:	48 8d 05 b0 a9 00 00 	lea    rax,[rip+0xa9b0]        # 14080 <gc_stack>
    96d0:	48 8b 14 02          	mov    rdx,QWORD PTR [rdx+rax*1]
    96d4:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    96d8:	48 8d 0c 00          	lea    rcx,[rax+rax*1]
    96dc:	8b 45 b4             	mov    eax,DWORD PTR [rbp-0x4c]
    96df:	48 98                	cdqe
    96e1:	48 01 c1             	add    rcx,rax
    96e4:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    96e8:	48 83 c1 04          	add    rcx,0x4
    96ec:	48 8b 04 c8          	mov    rax,QWORD PTR [rax+rcx*8]
    96f0:	48 89 d6             	mov    rsi,rdx
    96f3:	48 89 c7             	mov    rdi,rax
    96f6:	e8 e6 df ff ff       	call   76e1 <cljn_cons>
    96fb:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    96ff:	48 8b 05 7a a9 00 02 	mov    rax,QWORD PTR [rip+0x200a97a]        # 2014080 <gc_sp>
    9706:	48 83 e8 01          	sub    rax,0x1
    970a:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    9711:	00 
    9712:	48 8d 15 67 a9 00 00 	lea    rdx,[rip+0xa967]        # 14080 <gc_stack>
    9719:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    971d:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    9721:	48 83 45 d0 01       	add    QWORD PTR [rbp-0x30],0x1
    9726:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    972a:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    972e:	48 39 45 d0          	cmp    QWORD PTR [rbp-0x30],rax
    9732:	7c 82                	jl     96b6 <hmap_cons_walk+0x3a>
    9734:	e9 f3 00 00 00       	jmp    982c <hmap_cons_walk+0x1b0>
    9739:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    973d:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    9741:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    9745:	8b 40 10             	mov    eax,DWORD PTR [rax+0x10]
    9748:	89 c0                	mov    eax,eax
    974a:	48 89 c7             	mov    rdi,rax
    974d:	e8 5e 63 00 00       	call   fab0 <__popcountdi2>
    9752:	89 45 cc             	mov    DWORD PTR [rbp-0x34],eax
    9755:	c7 45 c8 00 00 00 00 	mov    DWORD PTR [rbp-0x38],0x0
    975c:	e9 bf 00 00 00       	jmp    9820 <hmap_cons_walk+0x1a4>
    9761:	8b 45 c8             	mov    eax,DWORD PTR [rbp-0x38]
    9764:	8d 14 00             	lea    edx,[rax+rax*1]
    9767:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    976b:	48 63 d2             	movsxd rdx,edx
    976e:	48 83 c2 02          	add    rdx,0x2
    9772:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    9777:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    977b:	48 83 7d e0 1a       	cmp    QWORD PTR [rbp-0x20],0x1a
    9780:	75 27                	jne    97a9 <hmap_cons_walk+0x12d>
    9782:	8b 45 c8             	mov    eax,DWORD PTR [rbp-0x38]
    9785:	01 c0                	add    eax,eax
    9787:	8d 50 01             	lea    edx,[rax+0x1]
    978a:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    978e:	48 63 d2             	movsxd rdx,edx
    9791:	48 83 c2 02          	add    rdx,0x2
    9795:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    979a:	8b 55 b4             	mov    edx,DWORD PTR [rbp-0x4c]
    979d:	89 d6                	mov    esi,edx
    979f:	48 89 c7             	mov    rdi,rax
    97a2:	e8 d5 fe ff ff       	call   967c <hmap_cons_walk>
    97a7:	eb 73                	jmp    981c <hmap_cons_walk+0x1a0>
    97a9:	48 8b 05 d0 a8 00 02 	mov    rax,QWORD PTR [rip+0x200a8d0]        # 2014080 <gc_sp>
    97b0:	48 83 e8 01          	sub    rax,0x1
    97b4:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    97bb:	00 
    97bc:	48 8d 05 bd a8 00 00 	lea    rax,[rip+0xa8bd]        # 14080 <gc_stack>
    97c3:	48 8b 14 02          	mov    rdx,QWORD PTR [rdx+rax*1]
    97c7:	83 7d b4 00          	cmp    DWORD PTR [rbp-0x4c],0x0
    97cb:	74 1a                	je     97e7 <hmap_cons_walk+0x16b>
    97cd:	8b 45 c8             	mov    eax,DWORD PTR [rbp-0x38]
    97d0:	01 c0                	add    eax,eax
    97d2:	8d 48 01             	lea    ecx,[rax+0x1]
    97d5:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    97d9:	48 63 c9             	movsxd rcx,ecx
    97dc:	48 83 c1 02          	add    rcx,0x2
    97e0:	48 8b 44 c8 08       	mov    rax,QWORD PTR [rax+rcx*8+0x8]
    97e5:	eb 04                	jmp    97eb <hmap_cons_walk+0x16f>
    97e7:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    97eb:	48 89 d6             	mov    rsi,rdx
    97ee:	48 89 c7             	mov    rdi,rax
    97f1:	e8 eb de ff ff       	call   76e1 <cljn_cons>
    97f6:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    97fa:	48 8b 05 7f a8 00 02 	mov    rax,QWORD PTR [rip+0x200a87f]        # 2014080 <gc_sp>
    9801:	48 83 e8 01          	sub    rax,0x1
    9805:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    980c:	00 
    980d:	48 8d 15 6c a8 00 00 	lea    rdx,[rip+0xa86c]        # 14080 <gc_stack>
    9814:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    9818:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    981c:	83 45 c8 01          	add    DWORD PTR [rbp-0x38],0x1
    9820:	8b 45 c8             	mov    eax,DWORD PTR [rbp-0x38]
    9823:	3b 45 cc             	cmp    eax,DWORD PTR [rbp-0x34]
    9826:	0f 8c 35 ff ff ff    	jl     9761 <hmap_cons_walk+0xe5>
    982c:	c9                   	leave
    982d:	c3                   	ret

000000000000982e <hnode_all_in>:
    982e:	f3 0f 1e fa          	endbr64
    9832:	55                   	push   rbp
    9833:	48 89 e5             	mov    rbp,rsp
    9836:	48 83 ec 40          	sub    rsp,0x40
    983a:	48 89 7d c8          	mov    QWORD PTR [rbp-0x38],rdi
    983e:	48 89 75 c0          	mov    QWORD PTR [rbp-0x40],rsi
    9842:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    9846:	48 89 c7             	mov    rdi,rax
    9849:	e8 7c d5 ff ff       	call   6dca <obj_type>
    984e:	83 f8 0c             	cmp    eax,0xc
    9851:	75 68                	jne    98bb <hnode_all_in+0x8d>
    9853:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    9857:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    985b:	48 c7 45 e0 00 00 00 	mov    QWORD PTR [rbp-0x20],0x0
    9862:	00 
    9863:	eb 3e                	jmp    98a3 <hnode_all_in+0x75>
    9865:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    9869:	48 8d 14 00          	lea    rdx,[rax+rax*1]
    986d:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    9871:	48 83 c2 04          	add    rdx,0x4
    9875:	48 8b 14 d0          	mov    rdx,QWORD PTR [rax+rdx*8]
    9879:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    987d:	48 89 d6             	mov    rsi,rdx
    9880:	48 89 c7             	mov    rdi,rax
    9883:	e8 fe 26 00 00       	call   bf86 <cljn_contains>
    9888:	48 89 c7             	mov    rdi,rax
    988b:	e8 ef 43 00 00       	call   dc7f <cljn_truthy>
    9890:	85 c0                	test   eax,eax
    9892:	75 0a                	jne    989e <hnode_all_in+0x70>
    9894:	b8 00 00 00 00       	mov    eax,0x0
    9899:	e9 d0 00 00 00       	jmp    996e <hnode_all_in+0x140>
    989e:	48 83 45 e0 01       	add    QWORD PTR [rbp-0x20],0x1
    98a3:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    98a7:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    98ab:	48 39 45 e0          	cmp    QWORD PTR [rbp-0x20],rax
    98af:	7c b4                	jl     9865 <hnode_all_in+0x37>
    98b1:	b8 01 00 00 00       	mov    eax,0x1
    98b6:	e9 b3 00 00 00       	jmp    996e <hnode_all_in+0x140>
    98bb:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    98bf:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    98c3:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    98c7:	8b 40 10             	mov    eax,DWORD PTR [rax+0x10]
    98ca:	89 c0                	mov    eax,eax
    98cc:	48 89 c7             	mov    rdi,rax
    98cf:	e8 dc 61 00 00       	call   fab0 <__popcountdi2>
    98d4:	89 45 dc             	mov    DWORD PTR [rbp-0x24],eax
    98d7:	c7 45 d8 00 00 00 00 	mov    DWORD PTR [rbp-0x28],0x0
    98de:	eb 7d                	jmp    995d <hnode_all_in+0x12f>
    98e0:	8b 45 d8             	mov    eax,DWORD PTR [rbp-0x28]
    98e3:	8d 14 00             	lea    edx,[rax+rax*1]
    98e6:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    98ea:	48 63 d2             	movsxd rdx,edx
    98ed:	48 83 c2 02          	add    rdx,0x2
    98f1:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    98f6:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    98fa:	48 83 7d f0 1a       	cmp    QWORD PTR [rbp-0x10],0x1a
    98ff:	75 32                	jne    9933 <hnode_all_in+0x105>
    9901:	8b 45 d8             	mov    eax,DWORD PTR [rbp-0x28]
    9904:	01 c0                	add    eax,eax
    9906:	8d 50 01             	lea    edx,[rax+0x1]
    9909:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    990d:	48 63 d2             	movsxd rdx,edx
    9910:	48 83 c2 02          	add    rdx,0x2
    9914:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    9919:	48 8b 55 c0          	mov    rdx,QWORD PTR [rbp-0x40]
    991d:	48 89 d6             	mov    rsi,rdx
    9920:	48 89 c7             	mov    rdi,rax
    9923:	e8 06 ff ff ff       	call   982e <hnode_all_in>
    9928:	85 c0                	test   eax,eax
    992a:	75 2d                	jne    9959 <hnode_all_in+0x12b>
    992c:	b8 00 00 00 00       	mov    eax,0x0
    9931:	eb 3b                	jmp    996e <hnode_all_in+0x140>
    9933:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    9937:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    993b:	48 89 d6             	mov    rsi,rdx
    993e:	48 89 c7             	mov    rdi,rax
    9941:	e8 40 26 00 00       	call   bf86 <cljn_contains>
    9946:	48 89 c7             	mov    rdi,rax
    9949:	e8 31 43 00 00       	call   dc7f <cljn_truthy>
    994e:	85 c0                	test   eax,eax
    9950:	75 07                	jne    9959 <hnode_all_in+0x12b>
    9952:	b8 00 00 00 00       	mov    eax,0x0
    9957:	eb 15                	jmp    996e <hnode_all_in+0x140>
    9959:	83 45 d8 01          	add    DWORD PTR [rbp-0x28],0x1
    995d:	8b 45 d8             	mov    eax,DWORD PTR [rbp-0x28]
    9960:	3b 45 dc             	cmp    eax,DWORD PTR [rbp-0x24]
    9963:	0f 8c 77 ff ff ff    	jl     98e0 <hnode_all_in+0xb2>
    9969:	b8 01 00 00 00       	mov    eax,0x1
    996e:	c9                   	leave
    996f:	c3                   	ret

0000000000009970 <hnode_push_keys>:
    9970:	f3 0f 1e fa          	endbr64
    9974:	55                   	push   rbp
    9975:	48 89 e5             	mov    rbp,rsp
    9978:	48 83 ec 40          	sub    rsp,0x40
    997c:	48 89 7d c8          	mov    QWORD PTR [rbp-0x38],rdi
    9980:	48 c7 45 d8 00 00 00 	mov    QWORD PTR [rbp-0x28],0x0
    9987:	00 
    9988:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    998c:	48 89 c7             	mov    rdi,rax
    998f:	e8 36 d4 ff ff       	call   6dca <obj_type>
    9994:	83 f8 0c             	cmp    eax,0xc
    9997:	75 4f                	jne    99e8 <hnode_push_keys+0x78>
    9999:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    999d:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    99a1:	48 c7 45 e0 00 00 00 	mov    QWORD PTR [rbp-0x20],0x0
    99a8:	00 
    99a9:	eb 26                	jmp    99d1 <hnode_push_keys+0x61>
    99ab:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    99af:	48 8d 14 00          	lea    rdx,[rax+rax*1]
    99b3:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    99b7:	48 83 c2 04          	add    rdx,0x4
    99bb:	48 8b 04 d0          	mov    rax,QWORD PTR [rax+rdx*8]
    99bf:	48 89 c7             	mov    rdi,rax
    99c2:	e8 9d d2 ff ff       	call   6c64 <cljn_gc_push>
    99c7:	48 83 45 d8 01       	add    QWORD PTR [rbp-0x28],0x1
    99cc:	48 83 45 e0 01       	add    QWORD PTR [rbp-0x20],0x1
    99d1:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    99d5:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    99d9:	48 39 45 e0          	cmp    QWORD PTR [rbp-0x20],rax
    99dd:	7c cc                	jl     99ab <hnode_push_keys+0x3b>
    99df:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    99e3:	e9 8d 00 00 00       	jmp    9a75 <hnode_push_keys+0x105>
    99e8:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    99ec:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    99f0:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    99f4:	8b 40 10             	mov    eax,DWORD PTR [rax+0x10]
    99f7:	89 c0                	mov    eax,eax
    99f9:	48 89 c7             	mov    rdi,rax
    99fc:	e8 af 60 00 00       	call   fab0 <__popcountdi2>
    9a01:	89 45 d4             	mov    DWORD PTR [rbp-0x2c],eax
    9a04:	c7 45 d0 00 00 00 00 	mov    DWORD PTR [rbp-0x30],0x0
    9a0b:	eb 5c                	jmp    9a69 <hnode_push_keys+0xf9>
    9a0d:	8b 45 d0             	mov    eax,DWORD PTR [rbp-0x30]
    9a10:	8d 14 00             	lea    edx,[rax+rax*1]
    9a13:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    9a17:	48 63 d2             	movsxd rdx,edx
    9a1a:	48 83 c2 02          	add    rdx,0x2
    9a1e:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    9a23:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    9a27:	48 83 7d f0 1a       	cmp    QWORD PTR [rbp-0x10],0x1a
    9a2c:	75 26                	jne    9a54 <hnode_push_keys+0xe4>
    9a2e:	8b 45 d0             	mov    eax,DWORD PTR [rbp-0x30]
    9a31:	01 c0                	add    eax,eax
    9a33:	8d 50 01             	lea    edx,[rax+0x1]
    9a36:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    9a3a:	48 63 d2             	movsxd rdx,edx
    9a3d:	48 83 c2 02          	add    rdx,0x2
    9a41:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    9a46:	48 89 c7             	mov    rdi,rax
    9a49:	e8 22 ff ff ff       	call   9970 <hnode_push_keys>
    9a4e:	48 01 45 d8          	add    QWORD PTR [rbp-0x28],rax
    9a52:	eb 11                	jmp    9a65 <hnode_push_keys+0xf5>
    9a54:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    9a58:	48 89 c7             	mov    rdi,rax
    9a5b:	e8 04 d2 ff ff       	call   6c64 <cljn_gc_push>
    9a60:	48 83 45 d8 01       	add    QWORD PTR [rbp-0x28],0x1
    9a65:	83 45 d0 01          	add    DWORD PTR [rbp-0x30],0x1
    9a69:	8b 45 d0             	mov    eax,DWORD PTR [rbp-0x30]
    9a6c:	3b 45 d4             	cmp    eax,DWORD PTR [rbp-0x2c]
    9a6f:	7c 9c                	jl     9a0d <hnode_push_keys+0x9d>
    9a71:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    9a75:	c9                   	leave
    9a76:	c3                   	ret

0000000000009a77 <cljn_map_alloc>:
    9a77:	f3 0f 1e fa          	endbr64
    9a7b:	55                   	push   rbp
    9a7c:	48 89 e5             	mov    rbp,rsp
    9a7f:	48 83 ec 30          	sub    rsp,0x30
    9a83:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    9a87:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    9a8b:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    9a8f:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    9a93:	48 c1 e0 04          	shl    rax,0x4
    9a97:	48 83 c0 18          	add    rax,0x18
    9a9b:	be 06 00 00 00       	mov    esi,0x6
    9aa0:	48 89 c7             	mov    rdi,rax
    9aa3:	e8 cb d4 ff ff       	call   6f73 <obj_alloc>
    9aa8:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    9aac:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    9ab0:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    9ab4:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    9ab8:	48 c7 45 e8 00 00 00 	mov    QWORD PTR [rbp-0x18],0x0
    9abf:	00 
    9ac0:	eb 1a                	jmp    9adc <cljn_map_alloc+0x65>
    9ac2:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    9ac6:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    9aca:	48 83 c2 02          	add    rdx,0x2
    9ace:	48 c7 44 d0 08 02 00 	mov    QWORD PTR [rax+rdx*8+0x8],0x2
    9ad5:	00 00 
    9ad7:	48 83 45 e8 01       	add    QWORD PTR [rbp-0x18],0x1
    9adc:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    9ae0:	48 01 c0             	add    rax,rax
    9ae3:	48 39 45 e8          	cmp    QWORD PTR [rbp-0x18],rax
    9ae7:	7c d9                	jl     9ac2 <cljn_map_alloc+0x4b>
    9ae9:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    9aed:	c9                   	leave
    9aee:	c3                   	ret

0000000000009aef <cljn_map_set>:
    9aef:	f3 0f 1e fa          	endbr64
    9af3:	55                   	push   rbp
    9af4:	48 89 e5             	mov    rbp,rsp
    9af7:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    9afb:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    9aff:	48 89 55 d8          	mov    QWORD PTR [rbp-0x28],rdx
    9b03:	48 89 4d d0          	mov    QWORD PTR [rbp-0x30],rcx
    9b07:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    9b0b:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    9b0f:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    9b13:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    9b17:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    9b1b:	48 8d 14 00          	lea    rdx,[rax+rax*1]
    9b1f:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    9b23:	48 8d 4a 02          	lea    rcx,[rdx+0x2]
    9b27:	48 8b 55 d8          	mov    rdx,QWORD PTR [rbp-0x28]
    9b2b:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    9b30:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    9b34:	48 01 c0             	add    rax,rax
    9b37:	48 8d 50 01          	lea    rdx,[rax+0x1]
    9b3b:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    9b3f:	48 8d 4a 02          	lea    rcx,[rdx+0x2]
    9b43:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    9b47:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    9b4c:	90                   	nop
    9b4d:	5d                   	pop    rbp
    9b4e:	c3                   	ret

0000000000009b4f <map_index>:
    9b4f:	f3 0f 1e fa          	endbr64
    9b53:	55                   	push   rbp
    9b54:	48 89 e5             	mov    rbp,rsp
    9b57:	48 83 ec 20          	sub    rsp,0x20
    9b5b:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    9b5f:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    9b63:	48 c7 45 f8 00 00 00 	mov    QWORD PTR [rbp-0x8],0x0
    9b6a:	00 
    9b6b:	eb 33                	jmp    9ba0 <map_index+0x51>
    9b6d:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    9b71:	48 8d 14 00          	lea    rdx,[rax+rax*1]
    9b75:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    9b79:	48 83 c2 02          	add    rdx,0x2
    9b7d:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    9b82:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    9b86:	48 89 d6             	mov    rsi,rdx
    9b89:	48 89 c7             	mov    rdi,rax
    9b8c:	e8 73 3a 00 00       	call   d604 <cljn_equal_raw>
    9b91:	85 c0                	test   eax,eax
    9b93:	74 06                	je     9b9b <map_index+0x4c>
    9b95:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    9b99:	eb 1a                	jmp    9bb5 <map_index+0x66>
    9b9b:	48 83 45 f8 01       	add    QWORD PTR [rbp-0x8],0x1
    9ba0:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    9ba4:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    9ba8:	48 39 45 f8          	cmp    QWORD PTR [rbp-0x8],rax
    9bac:	7c bf                	jl     9b6d <map_index+0x1e>
    9bae:	48 c7 c0 ff ff ff ff 	mov    rax,0xffffffffffffffff
    9bb5:	c9                   	leave
    9bb6:	c3                   	ret

0000000000009bb7 <hmap_from_arraymap>:
    9bb7:	f3 0f 1e fa          	endbr64
    9bbb:	55                   	push   rbp
    9bbc:	48 89 e5             	mov    rbp,rsp
    9bbf:	41 54                	push   r12
    9bc1:	53                   	push   rbx
    9bc2:	48 81 ec 90 00 00 00 	sub    rsp,0x90
    9bc9:	48 89 bd 78 ff ff ff 	mov    QWORD PTR [rbp-0x88],rdi
    9bd0:	48 89 b5 70 ff ff ff 	mov    QWORD PTR [rbp-0x90],rsi
    9bd7:	48 89 95 68 ff ff ff 	mov    QWORD PTR [rbp-0x98],rdx
    9bde:	64 48 8b 04 25 28 00 	mov    rax,QWORD PTR fs:0x28
    9be5:	00 00 
    9be7:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    9beb:	31 c0                	xor    eax,eax
    9bed:	bf 00 00 00 00       	mov    edi,0x0
    9bf2:	e8 91 f1 ff ff       	call   8d88 <mnode_alloc>
    9bf7:	48 89 45 98          	mov    QWORD PTR [rbp-0x68],rax
    9bfb:	48 8b 45 98          	mov    rax,QWORD PTR [rbp-0x68]
    9bff:	c7 40 10 00 00 00 00 	mov    DWORD PTR [rax+0x10],0x0
    9c06:	be 0a 00 00 00       	mov    esi,0xa
    9c0b:	bf 20 00 00 00       	mov    edi,0x20
    9c10:	e8 5e d3 ff ff       	call   6f73 <obj_alloc>
    9c15:	48 89 45 a0          	mov    QWORD PTR [rbp-0x60],rax
    9c19:	48 8b 45 a0          	mov    rax,QWORD PTR [rbp-0x60]
    9c1d:	48 c7 40 10 00 00 00 	mov    QWORD PTR [rax+0x10],0x0
    9c24:	00 
    9c25:	48 8b 55 98          	mov    rdx,QWORD PTR [rbp-0x68]
    9c29:	48 8b 45 a0          	mov    rax,QWORD PTR [rbp-0x60]
    9c2d:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    9c31:	48 8b 45 a0          	mov    rax,QWORD PTR [rbp-0x60]
    9c35:	48 89 45 a8          	mov    QWORD PTR [rbp-0x58],rax
    9c39:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    9c3d:	48 89 c7             	mov    rdi,rax
    9c40:	e8 1f d0 ff ff       	call   6c64 <cljn_gc_push>
    9c45:	48 c7 45 90 00 00 00 	mov    QWORD PTR [rbp-0x70],0x0
    9c4c:	00 
    9c4d:	e9 fb 00 00 00       	jmp    9d4d <hmap_from_arraymap+0x196>
    9c52:	48 8b 05 27 a4 00 02 	mov    rax,QWORD PTR [rip+0x200a427]        # 2014080 <gc_sp>
    9c59:	48 83 e8 01          	sub    rax,0x1
    9c5d:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    9c64:	00 
    9c65:	48 8d 05 14 a4 00 00 	lea    rax,[rip+0xa414]        # 14080 <gc_stack>
    9c6c:	48 8b 04 02          	mov    rax,QWORD PTR [rdx+rax*1]
    9c70:	48 89 45 d0          	mov    QWORD PTR [rbp-0x30],rax
    9c74:	48 8b 45 90          	mov    rax,QWORD PTR [rbp-0x70]
    9c78:	48 01 c0             	add    rax,rax
    9c7b:	48 8d 50 01          	lea    rdx,[rax+0x1]
    9c7f:	48 8b 85 78 ff ff ff 	mov    rax,QWORD PTR [rbp-0x88]
    9c86:	48 83 c2 02          	add    rdx,0x2
    9c8a:	4c 8b 64 d0 08       	mov    r12,QWORD PTR [rax+rdx*8+0x8]
    9c8f:	48 8b 45 90          	mov    rax,QWORD PTR [rbp-0x70]
    9c93:	48 8d 14 00          	lea    rdx,[rax+rax*1]
    9c97:	48 8b 85 78 ff ff ff 	mov    rax,QWORD PTR [rbp-0x88]
    9c9e:	48 83 c2 02          	add    rdx,0x2
    9ca2:	48 8b 5c d0 08       	mov    rbx,QWORD PTR [rax+rdx*8+0x8]
    9ca7:	48 8b 45 90          	mov    rax,QWORD PTR [rbp-0x70]
    9cab:	48 8d 14 00          	lea    rdx,[rax+rax*1]
    9caf:	48 8b 85 78 ff ff ff 	mov    rax,QWORD PTR [rbp-0x88]
    9cb6:	48 83 c2 02          	add    rdx,0x2
    9cba:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    9cbf:	48 89 c7             	mov    rdi,rax
    9cc2:	e8 9d ef ff ff       	call   8c64 <cljn_hash>
    9cc7:	89 c2                	mov    edx,eax
    9cc9:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    9ccd:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    9cd1:	48 8d 4d 8c          	lea    rcx,[rbp-0x74]
    9cd5:	49 89 c9             	mov    r9,rcx
    9cd8:	4d 89 e0             	mov    r8,r12
    9cdb:	48 89 d9             	mov    rcx,rbx
    9cde:	be 00 00 00 00       	mov    esi,0x0
    9ce3:	48 89 c7             	mov    rdi,rax
    9ce6:	e8 13 f4 ff ff       	call   90fe <node_assoc>
    9ceb:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    9cef:	be 0a 00 00 00       	mov    esi,0xa
    9cf4:	bf 20 00 00 00       	mov    edi,0x20
    9cf9:	e8 75 d2 ff ff       	call   6f73 <obj_alloc>
    9cfe:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    9d02:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    9d06:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    9d0a:	8b 45 8c             	mov    eax,DWORD PTR [rbp-0x74]
    9d0d:	48 98                	cdqe
    9d0f:	48 01 c2             	add    rdx,rax
    9d12:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    9d16:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    9d1a:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    9d1e:	48 8b 55 d8          	mov    rdx,QWORD PTR [rbp-0x28]
    9d22:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    9d26:	48 8b 05 53 a3 00 02 	mov    rax,QWORD PTR [rip+0x200a353]        # 2014080 <gc_sp>
    9d2d:	48 8d 50 ff          	lea    rdx,[rax-0x1]
    9d31:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    9d35:	48 8d 0c d5 00 00 00 	lea    rcx,[rdx*8+0x0]
    9d3c:	00 
    9d3d:	48 8d 15 3c a3 00 00 	lea    rdx,[rip+0xa33c]        # 14080 <gc_stack>
    9d44:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    9d48:	48 83 45 90 01       	add    QWORD PTR [rbp-0x70],0x1
    9d4d:	48 8b 85 78 ff ff ff 	mov    rax,QWORD PTR [rbp-0x88]
    9d54:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    9d58:	48 39 45 90          	cmp    QWORD PTR [rbp-0x70],rax
    9d5c:	0f 8c f0 fe ff ff    	jl     9c52 <hmap_from_arraymap+0x9b>
    9d62:	48 8b 05 17 a3 00 02 	mov    rax,QWORD PTR [rip+0x200a317]        # 2014080 <gc_sp>
    9d69:	48 83 e8 01          	sub    rax,0x1
    9d6d:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    9d74:	00 
    9d75:	48 8d 05 04 a3 00 00 	lea    rax,[rip+0xa304]        # 14080 <gc_stack>
    9d7c:	48 8b 04 02          	mov    rax,QWORD PTR [rdx+rax*1]
    9d80:	48 89 45 b0          	mov    QWORD PTR [rbp-0x50],rax
    9d84:	48 8b 85 70 ff ff ff 	mov    rax,QWORD PTR [rbp-0x90]
    9d8b:	48 89 c7             	mov    rdi,rax
    9d8e:	e8 d1 ee ff ff       	call   8c64 <cljn_hash>
    9d93:	89 c6                	mov    esi,eax
    9d95:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    9d99:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    9d9d:	48 8d 7d 8c          	lea    rdi,[rbp-0x74]
    9da1:	48 8b 8d 68 ff ff ff 	mov    rcx,QWORD PTR [rbp-0x98]
    9da8:	48 8b 95 70 ff ff ff 	mov    rdx,QWORD PTR [rbp-0x90]
    9daf:	49 89 f9             	mov    r9,rdi
    9db2:	49 89 c8             	mov    r8,rcx
    9db5:	48 89 d1             	mov    rcx,rdx
    9db8:	89 f2                	mov    edx,esi
    9dba:	be 00 00 00 00       	mov    esi,0x0
    9dbf:	48 89 c7             	mov    rdi,rax
    9dc2:	e8 37 f3 ff ff       	call   90fe <node_assoc>
    9dc7:	48 89 45 b8          	mov    QWORD PTR [rbp-0x48],rax
    9dcb:	be 0a 00 00 00       	mov    esi,0xa
    9dd0:	bf 20 00 00 00       	mov    edi,0x20
    9dd5:	e8 99 d1 ff ff       	call   6f73 <obj_alloc>
    9dda:	48 89 45 c0          	mov    QWORD PTR [rbp-0x40],rax
    9dde:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    9de2:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    9de6:	8b 45 8c             	mov    eax,DWORD PTR [rbp-0x74]
    9de9:	48 98                	cdqe
    9deb:	48 01 c2             	add    rdx,rax
    9dee:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    9df2:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    9df6:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    9dfa:	48 8b 55 b8          	mov    rdx,QWORD PTR [rbp-0x48]
    9dfe:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    9e02:	48 8b 05 77 a2 00 02 	mov    rax,QWORD PTR [rip+0x200a277]        # 2014080 <gc_sp>
    9e09:	48 8d 50 ff          	lea    rdx,[rax-0x1]
    9e0d:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    9e11:	48 8d 0c d5 00 00 00 	lea    rcx,[rdx*8+0x0]
    9e18:	00 
    9e19:	48 8d 15 60 a2 00 00 	lea    rdx,[rip+0xa260]        # 14080 <gc_stack>
    9e20:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    9e24:	48 8b 05 55 a2 00 02 	mov    rax,QWORD PTR [rip+0x200a255]        # 2014080 <gc_sp>
    9e2b:	48 83 e8 01          	sub    rax,0x1
    9e2f:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    9e36:	00 
    9e37:	48 8d 05 42 a2 00 00 	lea    rax,[rip+0xa242]        # 14080 <gc_stack>
    9e3e:	48 8b 04 02          	mov    rax,QWORD PTR [rdx+rax*1]
    9e42:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
    9e46:	bf 01 00 00 00       	mov    edi,0x1
    9e4b:	e8 8c ce ff ff       	call   6cdc <cljn_gc_popn>
    9e50:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    9e54:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    9e58:	64 48 2b 14 25 28 00 	sub    rdx,QWORD PTR fs:0x28
    9e5f:	00 00 
    9e61:	74 05                	je     9e68 <hmap_from_arraymap+0x2b1>
    9e63:	e8 f8 71 ff ff       	call   1060 <__stack_chk_fail@plt>
    9e68:	48 81 c4 90 00 00 00 	add    rsp,0x90
    9e6f:	5b                   	pop    rbx
    9e70:	41 5c                	pop    r12
    9e72:	5d                   	pop    rbp
    9e73:	c3                   	ret

0000000000009e74 <cljn_map_get>:
    9e74:	f3 0f 1e fa          	endbr64
    9e78:	55                   	push   rbp
    9e79:	48 89 e5             	mov    rbp,rsp
    9e7c:	48 83 ec 30          	sub    rsp,0x30
    9e80:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    9e84:	48 89 75 d0          	mov    QWORD PTR [rbp-0x30],rsi
    9e88:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    9e8c:	48 89 c7             	mov    rdi,rax
    9e8f:	e8 36 cf ff ff       	call   6dca <obj_type>
    9e94:	83 f8 0a             	cmp    eax,0xa
    9e97:	75 4a                	jne    9ee3 <cljn_map_get+0x6f>
    9e99:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    9e9d:	48 89 c7             	mov    rdi,rax
    9ea0:	e8 bf ed ff ff       	call   8c64 <cljn_hash>
    9ea5:	89 c6                	mov    esi,eax
    9ea7:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    9eab:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    9eaf:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    9eb3:	48 89 d1             	mov    rcx,rdx
    9eb6:	89 f2                	mov    edx,esi
    9eb8:	be 00 00 00 00       	mov    esi,0x0
    9ebd:	48 89 c7             	mov    rdi,rax
    9ec0:	e8 ee ee ff ff       	call   8db3 <node_get>
    9ec5:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    9ec9:	48 83 7d f8 2a       	cmp    QWORD PTR [rbp-0x8],0x2a
    9ece:	74 09                	je     9ed9 <cljn_map_get+0x65>
    9ed0:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    9ed4:	e9 8d 00 00 00       	jmp    9f66 <cljn_map_get+0xf2>
    9ed9:	b8 02 00 00 00       	mov    eax,0x2
    9ede:	e9 83 00 00 00       	jmp    9f66 <cljn_map_get+0xf2>
    9ee3:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    9ee7:	48 89 c7             	mov    rdi,rax
    9eea:	e8 db ce ff ff       	call   6dca <obj_type>
    9eef:	83 f8 0f             	cmp    eax,0xf
    9ef2:	75 15                	jne    9f09 <cljn_map_get+0x95>
    9ef4:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    9ef8:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    9efc:	48 89 d6             	mov    rsi,rdx
    9eff:	48 89 c7             	mov    rdi,rax
    9f02:	e8 a9 16 00 00       	call   b5b0 <cljn_sorted_get>
    9f07:	eb 5d                	jmp    9f66 <cljn_map_get+0xf2>
    9f09:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    9f0d:	48 89 c7             	mov    rdi,rax
    9f10:	e8 b5 ce ff ff       	call   6dca <obj_type>
    9f15:	83 f8 06             	cmp    eax,0x6
    9f18:	74 07                	je     9f21 <cljn_map_get+0xad>
    9f1a:	b8 02 00 00 00       	mov    eax,0x2
    9f1f:	eb 45                	jmp    9f66 <cljn_map_get+0xf2>
    9f21:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    9f25:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    9f29:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    9f2d:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    9f31:	48 89 d6             	mov    rsi,rdx
    9f34:	48 89 c7             	mov    rdi,rax
    9f37:	e8 13 fc ff ff       	call   9b4f <map_index>
    9f3c:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    9f40:	48 83 7d f0 00       	cmp    QWORD PTR [rbp-0x10],0x0
    9f45:	78 1a                	js     9f61 <cljn_map_get+0xed>
    9f47:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    9f4b:	48 01 c0             	add    rax,rax
    9f4e:	48 8d 50 01          	lea    rdx,[rax+0x1]
    9f52:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    9f56:	48 83 c2 02          	add    rdx,0x2
    9f5a:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    9f5f:	eb 05                	jmp    9f66 <cljn_map_get+0xf2>
    9f61:	b8 02 00 00 00       	mov    eax,0x2
    9f66:	c9                   	leave
    9f67:	c3                   	ret

0000000000009f68 <cljn_map_contains>:
    9f68:	f3 0f 1e fa          	endbr64
    9f6c:	55                   	push   rbp
    9f6d:	48 89 e5             	mov    rbp,rsp
    9f70:	48 83 ec 10          	sub    rsp,0x10
    9f74:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    9f78:	48 89 75 f0          	mov    QWORD PTR [rbp-0x10],rsi
    9f7c:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    9f80:	48 89 c7             	mov    rdi,rax
    9f83:	e8 42 ce ff ff       	call   6dca <obj_type>
    9f88:	83 f8 0a             	cmp    eax,0xa
    9f8b:	75 3f                	jne    9fcc <cljn_map_contains+0x64>
    9f8d:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    9f91:	48 89 c7             	mov    rdi,rax
    9f94:	e8 cb ec ff ff       	call   8c64 <cljn_hash>
    9f99:	89 c6                	mov    esi,eax
    9f9b:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    9f9f:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    9fa3:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    9fa7:	48 89 d1             	mov    rcx,rdx
    9faa:	89 f2                	mov    edx,esi
    9fac:	be 00 00 00 00       	mov    esi,0x0
    9fb1:	48 89 c7             	mov    rdi,rax
    9fb4:	e8 fa ed ff ff       	call   8db3 <node_get>
    9fb9:	48 83 f8 2a          	cmp    rax,0x2a
    9fbd:	0f 95 c0             	setne  al
    9fc0:	0f b6 c0             	movzx  eax,al
    9fc3:	89 c7                	mov    edi,eax
    9fc5:	e8 ce 33 00 00       	call   d398 <b2v>
    9fca:	eb 62                	jmp    a02e <cljn_map_contains+0xc6>
    9fcc:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    9fd0:	48 89 c7             	mov    rdi,rax
    9fd3:	e8 f2 cd ff ff       	call   6dca <obj_type>
    9fd8:	83 f8 0f             	cmp    eax,0xf
    9fdb:	75 15                	jne    9ff2 <cljn_map_contains+0x8a>
    9fdd:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    9fe1:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    9fe5:	48 89 d6             	mov    rsi,rdx
    9fe8:	48 89 c7             	mov    rdi,rax
    9feb:	e8 03 16 00 00       	call   b5f3 <cljn_sorted_contains>
    9ff0:	eb 3c                	jmp    a02e <cljn_map_contains+0xc6>
    9ff2:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    9ff6:	48 89 c7             	mov    rdi,rax
    9ff9:	e8 cc cd ff ff       	call   6dca <obj_type>
    9ffe:	83 f8 06             	cmp    eax,0x6
    a001:	75 1f                	jne    a022 <cljn_map_contains+0xba>
    a003:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    a007:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    a00b:	48 89 d6             	mov    rsi,rdx
    a00e:	48 89 c7             	mov    rdi,rax
    a011:	e8 39 fb ff ff       	call   9b4f <map_index>
    a016:	48 85 c0             	test   rax,rax
    a019:	78 07                	js     a022 <cljn_map_contains+0xba>
    a01b:	b8 01 00 00 00       	mov    eax,0x1
    a020:	eb 05                	jmp    a027 <cljn_map_contains+0xbf>
    a022:	b8 00 00 00 00       	mov    eax,0x0
    a027:	89 c7                	mov    edi,eax
    a029:	e8 6a 33 00 00       	call   d398 <b2v>
    a02e:	c9                   	leave
    a02f:	c3                   	ret

000000000000a030 <cljn_map_assoc>:
    a030:	f3 0f 1e fa          	endbr64
    a034:	55                   	push   rbp
    a035:	48 89 e5             	mov    rbp,rsp
    a038:	48 83 c4 80          	add    rsp,0xffffffffffffff80
    a03c:	48 89 7d 98          	mov    QWORD PTR [rbp-0x68],rdi
    a040:	48 89 75 90          	mov    QWORD PTR [rbp-0x70],rsi
    a044:	48 89 55 88          	mov    QWORD PTR [rbp-0x78],rdx
    a048:	64 48 8b 04 25 28 00 	mov    rax,QWORD PTR fs:0x28
    a04f:	00 00 
    a051:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    a055:	31 c0                	xor    eax,eax
    a057:	e8 c5 ce ff ff       	call   6f21 <maybe_gc>
    a05c:	8b 05 36 a0 00 02    	mov    eax,DWORD PTR [rip+0x200a036]        # 2014098 <gc_disabled>
    a062:	83 c0 01             	add    eax,0x1
    a065:	89 05 2d a0 00 02    	mov    DWORD PTR [rip+0x200a02d],eax        # 2014098 <gc_disabled>
    a06b:	48 8b 45 98          	mov    rax,QWORD PTR [rbp-0x68]
    a06f:	48 89 c7             	mov    rdi,rax
    a072:	e8 53 cd ff ff       	call   6dca <obj_type>
    a077:	83 f8 0a             	cmp    eax,0xa
    a07a:	0f 85 8a 00 00 00    	jne    a10a <cljn_map_assoc+0xda>
    a080:	48 8b 45 98          	mov    rax,QWORD PTR [rbp-0x68]
    a084:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    a088:	48 8b 45 90          	mov    rax,QWORD PTR [rbp-0x70]
    a08c:	48 89 c7             	mov    rdi,rax
    a08f:	e8 d0 eb ff ff       	call   8c64 <cljn_hash>
    a094:	89 c6                	mov    esi,eax
    a096:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    a09a:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    a09e:	48 8d 7d a4          	lea    rdi,[rbp-0x5c]
    a0a2:	48 8b 4d 88          	mov    rcx,QWORD PTR [rbp-0x78]
    a0a6:	48 8b 55 90          	mov    rdx,QWORD PTR [rbp-0x70]
    a0aa:	49 89 f9             	mov    r9,rdi
    a0ad:	49 89 c8             	mov    r8,rcx
    a0b0:	48 89 d1             	mov    rcx,rdx
    a0b3:	89 f2                	mov    edx,esi
    a0b5:	be 00 00 00 00       	mov    esi,0x0
    a0ba:	48 89 c7             	mov    rdi,rax
    a0bd:	e8 3c f0 ff ff       	call   90fe <node_assoc>
    a0c2:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    a0c6:	be 0a 00 00 00       	mov    esi,0xa
    a0cb:	bf 20 00 00 00       	mov    edi,0x20
    a0d0:	e8 9e ce ff ff       	call   6f73 <obj_alloc>
    a0d5:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    a0d9:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    a0dd:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    a0e1:	8b 45 a4             	mov    eax,DWORD PTR [rbp-0x5c]
    a0e4:	48 98                	cdqe
    a0e6:	48 01 c2             	add    rdx,rax
    a0e9:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    a0ed:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    a0f1:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    a0f5:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    a0f9:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    a0fd:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    a101:	48 89 45 a8          	mov    QWORD PTR [rbp-0x58],rax
    a105:	e9 70 01 00 00       	jmp    a27a <cljn_map_assoc+0x24a>
    a10a:	48 8b 45 98          	mov    rax,QWORD PTR [rbp-0x68]
    a10e:	48 89 45 b8          	mov    QWORD PTR [rbp-0x48],rax
    a112:	48 8b 55 90          	mov    rdx,QWORD PTR [rbp-0x70]
    a116:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    a11a:	48 89 d6             	mov    rsi,rdx
    a11d:	48 89 c7             	mov    rdi,rax
    a120:	e8 2a fa ff ff       	call   9b4f <map_index>
    a125:	48 89 45 c0          	mov    QWORD PTR [rbp-0x40],rax
    a129:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    a12d:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    a131:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
    a135:	48 83 7d c0 00       	cmp    QWORD PTR [rbp-0x40],0x0
    a13a:	79 27                	jns    a163 <cljn_map_assoc+0x133>
    a13c:	48 83 7d c8 07       	cmp    QWORD PTR [rbp-0x38],0x7
    a141:	7e 20                	jle    a163 <cljn_map_assoc+0x133>
    a143:	48 8b 55 88          	mov    rdx,QWORD PTR [rbp-0x78]
    a147:	48 8b 4d 90          	mov    rcx,QWORD PTR [rbp-0x70]
    a14b:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    a14f:	48 89 ce             	mov    rsi,rcx
    a152:	48 89 c7             	mov    rdi,rax
    a155:	e8 5d fa ff ff       	call   9bb7 <hmap_from_arraymap>
    a15a:	48 89 45 a8          	mov    QWORD PTR [rbp-0x58],rax
    a15e:	e9 17 01 00 00       	jmp    a27a <cljn_map_assoc+0x24a>
    a163:	48 83 7d c0 00       	cmp    QWORD PTR [rbp-0x40],0x0
    a168:	79 0a                	jns    a174 <cljn_map_assoc+0x144>
    a16a:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    a16e:	48 83 c0 01          	add    rax,0x1
    a172:	eb 04                	jmp    a178 <cljn_map_assoc+0x148>
    a174:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    a178:	48 89 45 d0          	mov    QWORD PTR [rbp-0x30],rax
    a17c:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    a180:	48 c1 e0 04          	shl    rax,0x4
    a184:	48 83 c0 18          	add    rax,0x18
    a188:	be 06 00 00 00       	mov    esi,0x6
    a18d:	48 89 c7             	mov    rdi,rax
    a190:	e8 de cd ff ff       	call   6f73 <obj_alloc>
    a195:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    a199:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a19d:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    a1a1:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    a1a5:	48 c7 45 b0 00 00 00 	mov    QWORD PTR [rbp-0x50],0x0
    a1ac:	00 
    a1ad:	eb 5f                	jmp    a20e <cljn_map_assoc+0x1de>
    a1af:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    a1b3:	48 8d 14 00          	lea    rdx,[rax+rax*1]
    a1b7:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    a1bb:	48 8d 0c 00          	lea    rcx,[rax+rax*1]
    a1bf:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    a1c3:	48 83 c2 02          	add    rdx,0x2
    a1c7:	48 8b 54 d0 08       	mov    rdx,QWORD PTR [rax+rdx*8+0x8]
    a1cc:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a1d0:	48 83 c1 02          	add    rcx,0x2
    a1d4:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    a1d9:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    a1dd:	48 01 c0             	add    rax,rax
    a1e0:	48 8d 50 01          	lea    rdx,[rax+0x1]
    a1e4:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    a1e8:	48 01 c0             	add    rax,rax
    a1eb:	48 8d 48 01          	lea    rcx,[rax+0x1]
    a1ef:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    a1f3:	48 83 c2 02          	add    rdx,0x2
    a1f7:	48 8b 54 d0 08       	mov    rdx,QWORD PTR [rax+rdx*8+0x8]
    a1fc:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a200:	48 83 c1 02          	add    rcx,0x2
    a204:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    a209:	48 83 45 b0 01       	add    QWORD PTR [rbp-0x50],0x1
    a20e:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    a212:	48 3b 45 c8          	cmp    rax,QWORD PTR [rbp-0x38]
    a216:	7c 97                	jl     a1af <cljn_map_assoc+0x17f>
    a218:	48 83 7d c0 00       	cmp    QWORD PTR [rbp-0x40],0x0
    a21d:	78 1e                	js     a23d <cljn_map_assoc+0x20d>
    a21f:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    a223:	48 01 c0             	add    rax,rax
    a226:	48 8d 50 01          	lea    rdx,[rax+0x1]
    a22a:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a22e:	48 8d 4a 02          	lea    rcx,[rdx+0x2]
    a232:	48 8b 55 88          	mov    rdx,QWORD PTR [rbp-0x78]
    a236:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    a23b:	eb 35                	jmp    a272 <cljn_map_assoc+0x242>
    a23d:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    a241:	48 8d 14 00          	lea    rdx,[rax+rax*1]
    a245:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a249:	48 8d 4a 02          	lea    rcx,[rdx+0x2]
    a24d:	48 8b 55 90          	mov    rdx,QWORD PTR [rbp-0x70]
    a251:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    a256:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    a25a:	48 01 c0             	add    rax,rax
    a25d:	48 8d 50 01          	lea    rdx,[rax+0x1]
    a261:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a265:	48 8d 4a 02          	lea    rcx,[rdx+0x2]
    a269:	48 8b 55 88          	mov    rdx,QWORD PTR [rbp-0x78]
    a26d:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    a272:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a276:	48 89 45 a8          	mov    QWORD PTR [rbp-0x58],rax
    a27a:	8b 05 18 9e 00 02    	mov    eax,DWORD PTR [rip+0x2009e18]        # 2014098 <gc_disabled>
    a280:	83 e8 01             	sub    eax,0x1
    a283:	89 05 0f 9e 00 02    	mov    DWORD PTR [rip+0x2009e0f],eax        # 2014098 <gc_disabled>
    a289:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    a28d:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    a291:	64 48 2b 14 25 28 00 	sub    rdx,QWORD PTR fs:0x28
    a298:	00 00 
    a29a:	74 05                	je     a2a1 <cljn_map_assoc+0x271>
    a29c:	e8 bf 6d ff ff       	call   1060 <__stack_chk_fail@plt>
    a2a1:	c9                   	leave
    a2a2:	c3                   	ret

000000000000a2a3 <cljn_map_dissoc>:
    a2a3:	f3 0f 1e fa          	endbr64
    a2a7:	55                   	push   rbp
    a2a8:	48 89 e5             	mov    rbp,rsp
    a2ab:	48 83 ec 60          	sub    rsp,0x60
    a2af:	48 89 7d a8          	mov    QWORD PTR [rbp-0x58],rdi
    a2b3:	48 89 75 a0          	mov    QWORD PTR [rbp-0x60],rsi
    a2b7:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    a2bb:	48 89 c7             	mov    rdi,rax
    a2be:	e8 07 cb ff ff       	call   6dca <obj_type>
    a2c3:	83 f8 0f             	cmp    eax,0xf
    a2c6:	75 18                	jne    a2e0 <cljn_map_dissoc+0x3d>
    a2c8:	48 8b 55 a0          	mov    rdx,QWORD PTR [rbp-0x60]
    a2cc:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    a2d0:	48 89 d6             	mov    rsi,rdx
    a2d3:	48 89 c7             	mov    rdi,rax
    a2d6:	e8 07 17 00 00       	call   b9e2 <cljn_sorted_dissoc>
    a2db:	e9 c8 01 00 00       	jmp    a4a8 <cljn_map_dissoc+0x205>
    a2e0:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    a2e4:	48 89 c7             	mov    rdi,rax
    a2e7:	e8 de ca ff ff       	call   6dca <obj_type>
    a2ec:	83 f8 0a             	cmp    eax,0xa
    a2ef:	0f 85 ae 00 00 00    	jne    a3a3 <cljn_map_dissoc+0x100>
    a2f5:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    a2f9:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    a2fd:	48 8b 45 a0          	mov    rax,QWORD PTR [rbp-0x60]
    a301:	48 89 c7             	mov    rdi,rax
    a304:	e8 5b e9 ff ff       	call   8c64 <cljn_hash>
    a309:	89 c6                	mov    esi,eax
    a30b:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    a30f:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    a313:	48 8b 55 a0          	mov    rdx,QWORD PTR [rbp-0x60]
    a317:	48 89 d1             	mov    rcx,rdx
    a31a:	89 f2                	mov    edx,esi
    a31c:	be 00 00 00 00       	mov    esi,0x0
    a321:	48 89 c7             	mov    rdi,rax
    a324:	e8 8a ea ff ff       	call   8db3 <node_get>
    a329:	48 83 f8 2a          	cmp    rax,0x2a
    a32d:	75 09                	jne    a338 <cljn_map_dissoc+0x95>
    a32f:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    a333:	e9 70 01 00 00       	jmp    a4a8 <cljn_map_dissoc+0x205>
    a338:	e8 e4 cb ff ff       	call   6f21 <maybe_gc>
    a33d:	bf 00 00 00 00       	mov    edi,0x0
    a342:	e8 30 f7 ff ff       	call   9a77 <cljn_map_alloc>
    a347:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    a34b:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    a34f:	48 89 c7             	mov    rdi,rax
    a352:	e8 0d c9 ff ff       	call   6c64 <cljn_gc_push>
    a357:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    a35b:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    a35f:	48 8b 55 a0          	mov    rdx,QWORD PTR [rbp-0x60]
    a363:	48 89 d6             	mov    rsi,rdx
    a366:	48 89 c7             	mov    rdi,rax
    a369:	e8 3c 01 00 00       	call   a4aa <hmap_dissoc_walk>
    a36e:	48 8b 05 0b 9d 00 02 	mov    rax,QWORD PTR [rip+0x2009d0b]        # 2014080 <gc_sp>
    a375:	48 83 e8 01          	sub    rax,0x1
    a379:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    a380:	00 
    a381:	48 8d 05 f8 9c 00 00 	lea    rax,[rip+0x9cf8]        # 14080 <gc_stack>
    a388:	48 8b 04 02          	mov    rax,QWORD PTR [rdx+rax*1]
    a38c:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    a390:	bf 01 00 00 00       	mov    edi,0x1
    a395:	e8 42 c9 ff ff       	call   6cdc <cljn_gc_popn>
    a39a:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    a39e:	e9 05 01 00 00       	jmp    a4a8 <cljn_map_dissoc+0x205>
    a3a3:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    a3a7:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
    a3ab:	48 8b 55 a0          	mov    rdx,QWORD PTR [rbp-0x60]
    a3af:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    a3b3:	48 89 d6             	mov    rsi,rdx
    a3b6:	48 89 c7             	mov    rdi,rax
    a3b9:	e8 91 f7 ff ff       	call   9b4f <map_index>
    a3be:	48 89 45 d0          	mov    QWORD PTR [rbp-0x30],rax
    a3c2:	48 83 7d d0 00       	cmp    QWORD PTR [rbp-0x30],0x0
    a3c7:	79 09                	jns    a3d2 <cljn_map_dissoc+0x12f>
    a3c9:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    a3cd:	e9 d6 00 00 00       	jmp    a4a8 <cljn_map_dissoc+0x205>
    a3d2:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    a3d6:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    a3da:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    a3de:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a3e2:	48 83 e8 01          	sub    rax,0x1
    a3e6:	48 c1 e0 04          	shl    rax,0x4
    a3ea:	48 83 c0 18          	add    rax,0x18
    a3ee:	be 06 00 00 00       	mov    esi,0x6
    a3f3:	48 89 c7             	mov    rdi,rax
    a3f6:	e8 78 cb ff ff       	call   6f73 <obj_alloc>
    a3fb:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    a3ff:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    a403:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
    a407:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a40b:	48 8d 50 ff          	lea    rdx,[rax-0x1]
    a40f:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    a413:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    a417:	48 c7 45 b8 00 00 00 	mov    QWORD PTR [rbp-0x48],0x0
    a41e:	00 
    a41f:	48 c7 45 c0 00 00 00 	mov    QWORD PTR [rbp-0x40],0x0
    a426:	00 
    a427:	eb 71                	jmp    a49a <cljn_map_dissoc+0x1f7>
    a429:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    a42d:	48 3b 45 d0          	cmp    rax,QWORD PTR [rbp-0x30]
    a431:	74 61                	je     a494 <cljn_map_dissoc+0x1f1>
    a433:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    a437:	48 8d 14 00          	lea    rdx,[rax+rax*1]
    a43b:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    a43f:	48 8d 0c 00          	lea    rcx,[rax+rax*1]
    a443:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    a447:	48 83 c2 02          	add    rdx,0x2
    a44b:	48 8b 54 d0 08       	mov    rdx,QWORD PTR [rax+rdx*8+0x8]
    a450:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    a454:	48 83 c1 02          	add    rcx,0x2
    a458:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    a45d:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    a461:	48 01 c0             	add    rax,rax
    a464:	48 8d 50 01          	lea    rdx,[rax+0x1]
    a468:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    a46c:	48 01 c0             	add    rax,rax
    a46f:	48 8d 48 01          	lea    rcx,[rax+0x1]
    a473:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    a477:	48 83 c2 02          	add    rdx,0x2
    a47b:	48 8b 54 d0 08       	mov    rdx,QWORD PTR [rax+rdx*8+0x8]
    a480:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    a484:	48 83 c1 02          	add    rcx,0x2
    a488:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    a48d:	48 83 45 b8 01       	add    QWORD PTR [rbp-0x48],0x1
    a492:	eb 01                	jmp    a495 <cljn_map_dissoc+0x1f2>
    a494:	90                   	nop
    a495:	48 83 45 c0 01       	add    QWORD PTR [rbp-0x40],0x1
    a49a:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    a49e:	48 3b 45 d8          	cmp    rax,QWORD PTR [rbp-0x28]
    a4a2:	7c 85                	jl     a429 <cljn_map_dissoc+0x186>
    a4a4:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    a4a8:	c9                   	leave
    a4a9:	c3                   	ret

000000000000a4aa <hmap_dissoc_walk>:
    a4aa:	f3 0f 1e fa          	endbr64
    a4ae:	55                   	push   rbp
    a4af:	48 89 e5             	mov    rbp,rsp
    a4b2:	48 83 ec 50          	sub    rsp,0x50
    a4b6:	48 89 7d b8          	mov    QWORD PTR [rbp-0x48],rdi
    a4ba:	48 89 75 b0          	mov    QWORD PTR [rbp-0x50],rsi
    a4be:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    a4c2:	48 89 c7             	mov    rdi,rax
    a4c5:	e8 00 c9 ff ff       	call   6dca <obj_type>
    a4ca:	83 f8 0c             	cmp    eax,0xc
    a4cd:	0f 85 d2 00 00 00    	jne    a5a5 <hmap_dissoc_walk+0xfb>
    a4d3:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    a4d7:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    a4db:	48 c7 45 d0 00 00 00 	mov    QWORD PTR [rbp-0x30],0x0
    a4e2:	00 
    a4e3:	e9 a6 00 00 00       	jmp    a58e <hmap_dissoc_walk+0xe4>
    a4e8:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    a4ec:	48 8d 14 00          	lea    rdx,[rax+rax*1]
    a4f0:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    a4f4:	48 83 c2 04          	add    rdx,0x4
    a4f8:	48 8b 04 d0          	mov    rax,QWORD PTR [rax+rdx*8]
    a4fc:	48 8b 55 b0          	mov    rdx,QWORD PTR [rbp-0x50]
    a500:	48 89 d6             	mov    rsi,rdx
    a503:	48 89 c7             	mov    rdi,rax
    a506:	e8 f9 30 00 00       	call   d604 <cljn_equal_raw>
    a50b:	85 c0                	test   eax,eax
    a50d:	75 7a                	jne    a589 <hmap_dissoc_walk+0xdf>
    a50f:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    a513:	48 01 c0             	add    rax,rax
    a516:	48 8d 50 01          	lea    rdx,[rax+0x1]
    a51a:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    a51e:	48 83 c2 04          	add    rdx,0x4
    a522:	48 8b 14 d0          	mov    rdx,QWORD PTR [rax+rdx*8]
    a526:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    a52a:	48 8d 0c 00          	lea    rcx,[rax+rax*1]
    a52e:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    a532:	48 83 c1 04          	add    rcx,0x4
    a536:	48 8b 0c c8          	mov    rcx,QWORD PTR [rax+rcx*8]
    a53a:	48 8b 05 3f 9b 00 02 	mov    rax,QWORD PTR [rip+0x2009b3f]        # 2014080 <gc_sp>
    a541:	48 83 e8 01          	sub    rax,0x1
    a545:	48 8d 34 c5 00 00 00 	lea    rsi,[rax*8+0x0]
    a54c:	00 
    a54d:	48 8d 05 2c 9b 00 00 	lea    rax,[rip+0x9b2c]        # 14080 <gc_stack>
    a554:	48 8b 04 06          	mov    rax,QWORD PTR [rsi+rax*1]
    a558:	48 89 ce             	mov    rsi,rcx
    a55b:	48 89 c7             	mov    rdi,rax
    a55e:	e8 cd fa ff ff       	call   a030 <cljn_map_assoc>
    a563:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    a567:	48 8b 05 12 9b 00 02 	mov    rax,QWORD PTR [rip+0x2009b12]        # 2014080 <gc_sp>
    a56e:	48 83 e8 01          	sub    rax,0x1
    a572:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    a579:	00 
    a57a:	48 8d 15 ff 9a 00 00 	lea    rdx,[rip+0x9aff]        # 14080 <gc_stack>
    a581:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    a585:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    a589:	48 83 45 d0 01       	add    QWORD PTR [rbp-0x30],0x1
    a58e:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    a592:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    a596:	48 39 45 d0          	cmp    QWORD PTR [rbp-0x30],rax
    a59a:	0f 8c 48 ff ff ff    	jl     a4e8 <hmap_dissoc_walk+0x3e>
    a5a0:	e9 07 01 00 00       	jmp    a6ac <hmap_dissoc_walk+0x202>
    a5a5:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    a5a9:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    a5ad:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a5b1:	8b 40 10             	mov    eax,DWORD PTR [rax+0x10]
    a5b4:	89 c0                	mov    eax,eax
    a5b6:	48 89 c7             	mov    rdi,rax
    a5b9:	e8 f2 54 00 00       	call   fab0 <__popcountdi2>
    a5be:	89 45 cc             	mov    DWORD PTR [rbp-0x34],eax
    a5c1:	c7 45 c8 00 00 00 00 	mov    DWORD PTR [rbp-0x38],0x0
    a5c8:	e9 d3 00 00 00       	jmp    a6a0 <hmap_dissoc_walk+0x1f6>
    a5cd:	8b 45 c8             	mov    eax,DWORD PTR [rbp-0x38]
    a5d0:	8d 14 00             	lea    edx,[rax+rax*1]
    a5d3:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a5d7:	48 63 d2             	movsxd rdx,edx
    a5da:	48 83 c2 02          	add    rdx,0x2
    a5de:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    a5e3:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    a5e7:	48 83 7d e0 1a       	cmp    QWORD PTR [rbp-0x20],0x1a
    a5ec:	75 2c                	jne    a61a <hmap_dissoc_walk+0x170>
    a5ee:	8b 45 c8             	mov    eax,DWORD PTR [rbp-0x38]
    a5f1:	01 c0                	add    eax,eax
    a5f3:	8d 50 01             	lea    edx,[rax+0x1]
    a5f6:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a5fa:	48 63 d2             	movsxd rdx,edx
    a5fd:	48 83 c2 02          	add    rdx,0x2
    a601:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    a606:	48 8b 55 b0          	mov    rdx,QWORD PTR [rbp-0x50]
    a60a:	48 89 d6             	mov    rsi,rdx
    a60d:	48 89 c7             	mov    rdi,rax
    a610:	e8 95 fe ff ff       	call   a4aa <hmap_dissoc_walk>
    a615:	e9 82 00 00 00       	jmp    a69c <hmap_dissoc_walk+0x1f2>
    a61a:	48 8b 55 b0          	mov    rdx,QWORD PTR [rbp-0x50]
    a61e:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    a622:	48 89 d6             	mov    rsi,rdx
    a625:	48 89 c7             	mov    rdi,rax
    a628:	e8 d7 2f 00 00       	call   d604 <cljn_equal_raw>
    a62d:	85 c0                	test   eax,eax
    a62f:	75 6b                	jne    a69c <hmap_dissoc_walk+0x1f2>
    a631:	8b 45 c8             	mov    eax,DWORD PTR [rbp-0x38]
    a634:	01 c0                	add    eax,eax
    a636:	8d 50 01             	lea    edx,[rax+0x1]
    a639:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a63d:	48 63 d2             	movsxd rdx,edx
    a640:	48 83 c2 02          	add    rdx,0x2
    a644:	48 8b 54 d0 08       	mov    rdx,QWORD PTR [rax+rdx*8+0x8]
    a649:	48 8b 05 30 9a 00 02 	mov    rax,QWORD PTR [rip+0x2009a30]        # 2014080 <gc_sp>
    a650:	48 83 e8 01          	sub    rax,0x1
    a654:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    a65b:	00 
    a65c:	48 8d 05 1d 9a 00 00 	lea    rax,[rip+0x9a1d]        # 14080 <gc_stack>
    a663:	48 8b 04 01          	mov    rax,QWORD PTR [rcx+rax*1]
    a667:	48 8b 4d e0          	mov    rcx,QWORD PTR [rbp-0x20]
    a66b:	48 89 ce             	mov    rsi,rcx
    a66e:	48 89 c7             	mov    rdi,rax
    a671:	e8 ba f9 ff ff       	call   a030 <cljn_map_assoc>
    a676:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    a67a:	48 8b 05 ff 99 00 02 	mov    rax,QWORD PTR [rip+0x20099ff]        # 2014080 <gc_sp>
    a681:	48 83 e8 01          	sub    rax,0x1
    a685:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    a68c:	00 
    a68d:	48 8d 15 ec 99 00 00 	lea    rdx,[rip+0x99ec]        # 14080 <gc_stack>
    a694:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    a698:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    a69c:	83 45 c8 01          	add    DWORD PTR [rbp-0x38],0x1
    a6a0:	8b 45 c8             	mov    eax,DWORD PTR [rbp-0x38]
    a6a3:	3b 45 cc             	cmp    eax,DWORD PTR [rbp-0x34]
    a6a6:	0f 8c 21 ff ff ff    	jl     a5cd <hmap_dissoc_walk+0x123>
    a6ac:	c9                   	leave
    a6ad:	c3                   	ret

000000000000a6ae <cljn_map_keys>:
    a6ae:	f3 0f 1e fa          	endbr64
    a6b2:	55                   	push   rbp
    a6b3:	48 89 e5             	mov    rbp,rsp
    a6b6:	48 83 ec 30          	sub    rsp,0x30
    a6ba:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    a6be:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a6c2:	48 89 c7             	mov    rdi,rax
    a6c5:	e8 00 c7 ff ff       	call   6dca <obj_type>
    a6ca:	83 f8 08             	cmp    eax,0x8
    a6cd:	75 0c                	jne    a6db <cljn_map_keys+0x2d>
    a6cf:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a6d3:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    a6d7:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    a6db:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a6df:	48 89 c7             	mov    rdi,rax
    a6e2:	e8 e3 c6 ff ff       	call   6dca <obj_type>
    a6e7:	83 f8 0f             	cmp    eax,0xf
    a6ea:	75 16                	jne    a702 <cljn_map_keys+0x54>
    a6ec:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a6f0:	be 00 00 00 00       	mov    esi,0x0
    a6f5:	48 89 c7             	mov    rdi,rax
    a6f8:	e8 ba 0f 00 00       	call   b6b7 <sorted_seq>
    a6fd:	e9 f9 00 00 00       	jmp    a7fb <cljn_map_keys+0x14d>
    a702:	48 c7 45 e8 12 00 00 	mov    QWORD PTR [rbp-0x18],0x12
    a709:	00 
    a70a:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    a70e:	48 89 c7             	mov    rdi,rax
    a711:	e8 4e c5 ff ff       	call   6c64 <cljn_gc_push>
    a716:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a71a:	48 89 c7             	mov    rdi,rax
    a71d:	e8 a8 c6 ff ff       	call   6dca <obj_type>
    a722:	83 f8 0a             	cmp    eax,0xa
    a725:	75 1a                	jne    a741 <cljn_map_keys+0x93>
    a727:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a72b:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    a72f:	be 00 00 00 00       	mov    esi,0x0
    a734:	48 89 c7             	mov    rdi,rax
    a737:	e8 40 ef ff ff       	call   967c <hmap_cons_walk>
    a73c:	e9 8a 00 00 00       	jmp    a7cb <cljn_map_keys+0x11d>
    a741:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a745:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    a749:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    a74d:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    a751:	48 83 e8 01          	sub    rax,0x1
    a755:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    a759:	eb 69                	jmp    a7c4 <cljn_map_keys+0x116>
    a75b:	48 8b 05 1e 99 00 02 	mov    rax,QWORD PTR [rip+0x200991e]        # 2014080 <gc_sp>
    a762:	48 83 e8 01          	sub    rax,0x1
    a766:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    a76d:	00 
    a76e:	48 8d 05 0b 99 00 00 	lea    rax,[rip+0x990b]        # 14080 <gc_stack>
    a775:	48 8b 14 02          	mov    rdx,QWORD PTR [rdx+rax*1]
    a779:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    a77d:	48 8d 0c 00          	lea    rcx,[rax+rax*1]
    a781:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    a785:	48 83 c1 02          	add    rcx,0x2
    a789:	48 8b 44 c8 08       	mov    rax,QWORD PTR [rax+rcx*8+0x8]
    a78e:	48 89 d6             	mov    rsi,rdx
    a791:	48 89 c7             	mov    rdi,rax
    a794:	e8 48 cf ff ff       	call   76e1 <cljn_cons>
    a799:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    a79d:	48 8b 05 dc 98 00 02 	mov    rax,QWORD PTR [rip+0x20098dc]        # 2014080 <gc_sp>
    a7a4:	48 83 e8 01          	sub    rax,0x1
    a7a8:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    a7af:	00 
    a7b0:	48 8d 15 c9 98 00 00 	lea    rdx,[rip+0x98c9]        # 14080 <gc_stack>
    a7b7:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    a7bb:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    a7bf:	48 83 6d e0 01       	sub    QWORD PTR [rbp-0x20],0x1
    a7c4:	48 83 7d e0 00       	cmp    QWORD PTR [rbp-0x20],0x0
    a7c9:	79 90                	jns    a75b <cljn_map_keys+0xad>
    a7cb:	48 8b 05 ae 98 00 02 	mov    rax,QWORD PTR [rip+0x20098ae]        # 2014080 <gc_sp>
    a7d2:	48 83 e8 01          	sub    rax,0x1
    a7d6:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    a7dd:	00 
    a7de:	48 8d 05 9b 98 00 00 	lea    rax,[rip+0x989b]        # 14080 <gc_stack>
    a7e5:	48 8b 04 02          	mov    rax,QWORD PTR [rdx+rax*1]
    a7e9:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    a7ed:	bf 01 00 00 00       	mov    edi,0x1
    a7f2:	e8 e5 c4 ff ff       	call   6cdc <cljn_gc_popn>
    a7f7:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    a7fb:	c9                   	leave
    a7fc:	c3                   	ret

000000000000a7fd <hmap_node_subset>:
    a7fd:	f3 0f 1e fa          	endbr64
    a801:	55                   	push   rbp
    a802:	48 89 e5             	mov    rbp,rsp
    a805:	48 83 ec 40          	sub    rsp,0x40
    a809:	48 89 7d c8          	mov    QWORD PTR [rbp-0x38],rdi
    a80d:	48 89 75 c0          	mov    QWORD PTR [rbp-0x40],rsi
    a811:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    a815:	48 89 c7             	mov    rdi,rax
    a818:	e8 ad c5 ff ff       	call   6dca <obj_type>
    a81d:	83 f8 0c             	cmp    eax,0xc
    a820:	0f 85 bb 00 00 00    	jne    a8e1 <hmap_node_subset+0xe4>
    a826:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    a82a:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    a82e:	48 c7 45 e0 00 00 00 	mov    QWORD PTR [rbp-0x20],0x0
    a835:	00 
    a836:	e9 8a 00 00 00       	jmp    a8c5 <hmap_node_subset+0xc8>
    a83b:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    a83f:	48 8d 14 00          	lea    rdx,[rax+rax*1]
    a843:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    a847:	48 83 c2 04          	add    rdx,0x4
    a84b:	48 8b 14 d0          	mov    rdx,QWORD PTR [rax+rdx*8]
    a84f:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    a853:	48 89 d6             	mov    rsi,rdx
    a856:	48 89 c7             	mov    rdi,rax
    a859:	e8 0a f7 ff ff       	call   9f68 <cljn_map_contains>
    a85e:	48 89 c7             	mov    rdi,rax
    a861:	e8 19 34 00 00       	call   dc7f <cljn_truthy>
    a866:	85 c0                	test   eax,eax
    a868:	74 4c                	je     a8b6 <hmap_node_subset+0xb9>
    a86a:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    a86e:	48 8d 14 00          	lea    rdx,[rax+rax*1]
    a872:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    a876:	48 83 c2 04          	add    rdx,0x4
    a87a:	48 8b 14 d0          	mov    rdx,QWORD PTR [rax+rdx*8]
    a87e:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    a882:	48 89 d6             	mov    rsi,rdx
    a885:	48 89 c7             	mov    rdi,rax
    a888:	e8 e7 f5 ff ff       	call   9e74 <cljn_map_get>
    a88d:	48 89 c2             	mov    rdx,rax
    a890:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    a894:	48 01 c0             	add    rax,rax
    a897:	48 8d 48 01          	lea    rcx,[rax+0x1]
    a89b:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    a89f:	48 83 c1 04          	add    rcx,0x4
    a8a3:	48 8b 04 c8          	mov    rax,QWORD PTR [rax+rcx*8]
    a8a7:	48 89 d6             	mov    rsi,rdx
    a8aa:	48 89 c7             	mov    rdi,rax
    a8ad:	e8 52 2d 00 00       	call   d604 <cljn_equal_raw>
    a8b2:	85 c0                	test   eax,eax
    a8b4:	75 0a                	jne    a8c0 <hmap_node_subset+0xc3>
    a8b6:	b8 00 00 00 00       	mov    eax,0x0
    a8bb:	e9 14 01 00 00       	jmp    a9d4 <hmap_node_subset+0x1d7>
    a8c0:	48 83 45 e0 01       	add    QWORD PTR [rbp-0x20],0x1
    a8c5:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    a8c9:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    a8cd:	48 39 45 e0          	cmp    QWORD PTR [rbp-0x20],rax
    a8d1:	0f 8c 64 ff ff ff    	jl     a83b <hmap_node_subset+0x3e>
    a8d7:	b8 01 00 00 00       	mov    eax,0x1
    a8dc:	e9 f3 00 00 00       	jmp    a9d4 <hmap_node_subset+0x1d7>
    a8e1:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    a8e5:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    a8e9:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    a8ed:	8b 40 10             	mov    eax,DWORD PTR [rax+0x10]
    a8f0:	89 c0                	mov    eax,eax
    a8f2:	48 89 c7             	mov    rdi,rax
    a8f5:	e8 b6 51 00 00       	call   fab0 <__popcountdi2>
    a8fa:	89 45 dc             	mov    DWORD PTR [rbp-0x24],eax
    a8fd:	c7 45 d8 00 00 00 00 	mov    DWORD PTR [rbp-0x28],0x0
    a904:	e9 ba 00 00 00       	jmp    a9c3 <hmap_node_subset+0x1c6>
    a909:	8b 45 d8             	mov    eax,DWORD PTR [rbp-0x28]
    a90c:	8d 14 00             	lea    edx,[rax+rax*1]
    a90f:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    a913:	48 63 d2             	movsxd rdx,edx
    a916:	48 83 c2 02          	add    rdx,0x2
    a91a:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    a91f:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    a923:	48 83 7d f0 1a       	cmp    QWORD PTR [rbp-0x10],0x1a
    a928:	75 32                	jne    a95c <hmap_node_subset+0x15f>
    a92a:	8b 45 d8             	mov    eax,DWORD PTR [rbp-0x28]
    a92d:	01 c0                	add    eax,eax
    a92f:	8d 50 01             	lea    edx,[rax+0x1]
    a932:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    a936:	48 63 d2             	movsxd rdx,edx
    a939:	48 83 c2 02          	add    rdx,0x2
    a93d:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    a942:	48 8b 55 c0          	mov    rdx,QWORD PTR [rbp-0x40]
    a946:	48 89 d6             	mov    rsi,rdx
    a949:	48 89 c7             	mov    rdi,rax
    a94c:	e8 ac fe ff ff       	call   a7fd <hmap_node_subset>
    a951:	85 c0                	test   eax,eax
    a953:	75 6a                	jne    a9bf <hmap_node_subset+0x1c2>
    a955:	b8 00 00 00 00       	mov    eax,0x0
    a95a:	eb 78                	jmp    a9d4 <hmap_node_subset+0x1d7>
    a95c:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    a960:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    a964:	48 89 d6             	mov    rsi,rdx
    a967:	48 89 c7             	mov    rdi,rax
    a96a:	e8 f9 f5 ff ff       	call   9f68 <cljn_map_contains>
    a96f:	48 89 c7             	mov    rdi,rax
    a972:	e8 08 33 00 00       	call   dc7f <cljn_truthy>
    a977:	85 c0                	test   eax,eax
    a979:	74 3d                	je     a9b8 <hmap_node_subset+0x1bb>
    a97b:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    a97f:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    a983:	48 89 d6             	mov    rsi,rdx
    a986:	48 89 c7             	mov    rdi,rax
    a989:	e8 e6 f4 ff ff       	call   9e74 <cljn_map_get>
    a98e:	48 89 c2             	mov    rdx,rax
    a991:	8b 45 d8             	mov    eax,DWORD PTR [rbp-0x28]
    a994:	01 c0                	add    eax,eax
    a996:	8d 48 01             	lea    ecx,[rax+0x1]
    a999:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    a99d:	48 63 c9             	movsxd rcx,ecx
    a9a0:	48 83 c1 02          	add    rcx,0x2
    a9a4:	48 8b 44 c8 08       	mov    rax,QWORD PTR [rax+rcx*8+0x8]
    a9a9:	48 89 d6             	mov    rsi,rdx
    a9ac:	48 89 c7             	mov    rdi,rax
    a9af:	e8 50 2c 00 00       	call   d604 <cljn_equal_raw>
    a9b4:	85 c0                	test   eax,eax
    a9b6:	75 07                	jne    a9bf <hmap_node_subset+0x1c2>
    a9b8:	b8 00 00 00 00       	mov    eax,0x0
    a9bd:	eb 15                	jmp    a9d4 <hmap_node_subset+0x1d7>
    a9bf:	83 45 d8 01          	add    DWORD PTR [rbp-0x28],0x1
    a9c3:	8b 45 d8             	mov    eax,DWORD PTR [rbp-0x28]
    a9c6:	3b 45 dc             	cmp    eax,DWORD PTR [rbp-0x24]
    a9c9:	0f 8c 3a ff ff ff    	jl     a909 <hmap_node_subset+0x10c>
    a9cf:	b8 01 00 00 00       	mov    eax,0x1
    a9d4:	c9                   	leave
    a9d5:	c3                   	ret

000000000000a9d6 <cljn_map_vals>:
    a9d6:	f3 0f 1e fa          	endbr64
    a9da:	55                   	push   rbp
    a9db:	48 89 e5             	mov    rbp,rsp
    a9de:	48 83 ec 30          	sub    rsp,0x30
    a9e2:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    a9e6:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a9ea:	48 89 c7             	mov    rdi,rax
    a9ed:	e8 d8 c3 ff ff       	call   6dca <obj_type>
    a9f2:	83 f8 08             	cmp    eax,0x8
    a9f5:	75 0c                	jne    aa03 <cljn_map_vals+0x2d>
    a9f7:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    a9fb:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    a9ff:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    aa03:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    aa07:	48 89 c7             	mov    rdi,rax
    aa0a:	e8 bb c3 ff ff       	call   6dca <obj_type>
    aa0f:	83 f8 0f             	cmp    eax,0xf
    aa12:	75 16                	jne    aa2a <cljn_map_vals+0x54>
    aa14:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    aa18:	be 01 00 00 00       	mov    esi,0x1
    aa1d:	48 89 c7             	mov    rdi,rax
    aa20:	e8 92 0c 00 00       	call   b6b7 <sorted_seq>
    aa25:	e9 fc 00 00 00       	jmp    ab26 <cljn_map_vals+0x150>
    aa2a:	48 c7 45 e8 12 00 00 	mov    QWORD PTR [rbp-0x18],0x12
    aa31:	00 
    aa32:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    aa36:	48 89 c7             	mov    rdi,rax
    aa39:	e8 26 c2 ff ff       	call   6c64 <cljn_gc_push>
    aa3e:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    aa42:	48 89 c7             	mov    rdi,rax
    aa45:	e8 80 c3 ff ff       	call   6dca <obj_type>
    aa4a:	83 f8 0a             	cmp    eax,0xa
    aa4d:	75 1a                	jne    aa69 <cljn_map_vals+0x93>
    aa4f:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    aa53:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    aa57:	be 01 00 00 00       	mov    esi,0x1
    aa5c:	48 89 c7             	mov    rdi,rax
    aa5f:	e8 18 ec ff ff       	call   967c <hmap_cons_walk>
    aa64:	e9 8d 00 00 00       	jmp    aaf6 <cljn_map_vals+0x120>
    aa69:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    aa6d:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    aa71:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    aa75:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    aa79:	48 83 e8 01          	sub    rax,0x1
    aa7d:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    aa81:	eb 6c                	jmp    aaef <cljn_map_vals+0x119>
    aa83:	48 8b 05 f6 95 00 02 	mov    rax,QWORD PTR [rip+0x20095f6]        # 2014080 <gc_sp>
    aa8a:	48 83 e8 01          	sub    rax,0x1
    aa8e:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    aa95:	00 
    aa96:	48 8d 05 e3 95 00 00 	lea    rax,[rip+0x95e3]        # 14080 <gc_stack>
    aa9d:	48 8b 14 02          	mov    rdx,QWORD PTR [rdx+rax*1]
    aaa1:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    aaa5:	48 01 c0             	add    rax,rax
    aaa8:	48 8d 48 01          	lea    rcx,[rax+0x1]
    aaac:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    aab0:	48 83 c1 02          	add    rcx,0x2
    aab4:	48 8b 44 c8 08       	mov    rax,QWORD PTR [rax+rcx*8+0x8]
    aab9:	48 89 d6             	mov    rsi,rdx
    aabc:	48 89 c7             	mov    rdi,rax
    aabf:	e8 1d cc ff ff       	call   76e1 <cljn_cons>
    aac4:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    aac8:	48 8b 05 b1 95 00 02 	mov    rax,QWORD PTR [rip+0x20095b1]        # 2014080 <gc_sp>
    aacf:	48 83 e8 01          	sub    rax,0x1
    aad3:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    aada:	00 
    aadb:	48 8d 15 9e 95 00 00 	lea    rdx,[rip+0x959e]        # 14080 <gc_stack>
    aae2:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    aae6:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    aaea:	48 83 6d e0 01       	sub    QWORD PTR [rbp-0x20],0x1
    aaef:	48 83 7d e0 00       	cmp    QWORD PTR [rbp-0x20],0x0
    aaf4:	79 8d                	jns    aa83 <cljn_map_vals+0xad>
    aaf6:	48 8b 05 83 95 00 02 	mov    rax,QWORD PTR [rip+0x2009583]        # 2014080 <gc_sp>
    aafd:	48 83 e8 01          	sub    rax,0x1
    ab01:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    ab08:	00 
    ab09:	48 8d 05 70 95 00 00 	lea    rax,[rip+0x9570]        # 14080 <gc_stack>
    ab10:	48 8b 04 02          	mov    rax,QWORD PTR [rdx+rax*1]
    ab14:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    ab18:	bf 01 00 00 00       	mov    edi,0x1
    ab1d:	e8 ba c1 ff ff       	call   6cdc <cljn_gc_popn>
    ab22:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    ab26:	c9                   	leave
    ab27:	c3                   	ret

000000000000ab28 <cmp_bytes>:
    ab28:	f3 0f 1e fa          	endbr64
    ab2c:	55                   	push   rbp
    ab2d:	48 89 e5             	mov    rbp,rsp
    ab30:	48 83 ec 30          	sub    rsp,0x30
    ab34:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    ab38:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    ab3c:	48 89 55 d8          	mov    QWORD PTR [rbp-0x28],rdx
    ab40:	48 89 4d d0          	mov    QWORD PTR [rbp-0x30],rcx
    ab44:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    ab48:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    ab4c:	48 39 c2             	cmp    rdx,rax
    ab4f:	48 0f 46 c2          	cmovbe rax,rdx
    ab53:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    ab57:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    ab5b:	48 8b 4d d8          	mov    rcx,QWORD PTR [rbp-0x28]
    ab5f:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    ab63:	48 89 ce             	mov    rsi,rcx
    ab66:	48 89 c7             	mov    rdi,rax
    ab69:	e8 22 65 ff ff       	call   1090 <memcmp@plt>
    ab6e:	89 45 f4             	mov    DWORD PTR [rbp-0xc],eax
    ab71:	83 7d f4 00          	cmp    DWORD PTR [rbp-0xc],0x0
    ab75:	74 14                	je     ab8b <cmp_bytes+0x63>
    ab77:	83 7d f4 00          	cmp    DWORD PTR [rbp-0xc],0x0
    ab7b:	79 07                	jns    ab84 <cmp_bytes+0x5c>
    ab7d:	b8 ff ff ff ff       	mov    eax,0xffffffff
    ab82:	eb 2e                	jmp    abb2 <cmp_bytes+0x8a>
    ab84:	b8 01 00 00 00       	mov    eax,0x1
    ab89:	eb 27                	jmp    abb2 <cmp_bytes+0x8a>
    ab8b:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    ab8f:	48 3b 45 d0          	cmp    rax,QWORD PTR [rbp-0x30]
    ab93:	74 18                	je     abad <cmp_bytes+0x85>
    ab95:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    ab99:	48 3b 45 d0          	cmp    rax,QWORD PTR [rbp-0x30]
    ab9d:	73 07                	jae    aba6 <cmp_bytes+0x7e>
    ab9f:	b8 ff ff ff ff       	mov    eax,0xffffffff
    aba4:	eb 0c                	jmp    abb2 <cmp_bytes+0x8a>
    aba6:	b8 01 00 00 00       	mov    eax,0x1
    abab:	eb 05                	jmp    abb2 <cmp_bytes+0x8a>
    abad:	b8 00 00 00 00       	mov    eax,0x0
    abb2:	c9                   	leave
    abb3:	c3                   	ret

000000000000abb4 <compare_raw>:
    abb4:	f3 0f 1e fa          	endbr64
    abb8:	55                   	push   rbp
    abb9:	48 89 e5             	mov    rbp,rsp
    abbc:	48 83 ec 40          	sub    rsp,0x40
    abc0:	48 89 7d c8          	mov    QWORD PTR [rbp-0x38],rdi
    abc4:	48 89 75 c0          	mov    QWORD PTR [rbp-0x40],rsi
    abc8:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    abcc:	83 e0 01             	and    eax,0x1
    abcf:	48 85 c0             	test   rax,rax
    abd2:	74 49                	je     ac1d <compare_raw+0x69>
    abd4:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    abd8:	83 e0 01             	and    eax,0x1
    abdb:	48 85 c0             	test   rax,rax
    abde:	74 3d                	je     ac1d <compare_raw+0x69>
    abe0:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    abe4:	48 d1 f8             	sar    rax,1
    abe7:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    abeb:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    abef:	48 d1 f8             	sar    rax,1
    abf2:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    abf6:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    abfa:	48 3b 45 e8          	cmp    rax,QWORD PTR [rbp-0x18]
    abfe:	7c 13                	jl     ac13 <compare_raw+0x5f>
    ac00:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    ac04:	48 3b 45 e8          	cmp    rax,QWORD PTR [rbp-0x18]
    ac08:	0f 9f c0             	setg   al
    ac0b:	0f b6 c0             	movzx  eax,al
    ac0e:	e9 3b 01 00 00       	jmp    ad4e <compare_raw+0x19a>
    ac13:	b8 ff ff ff ff       	mov    eax,0xffffffff
    ac18:	e9 31 01 00 00       	jmp    ad4e <compare_raw+0x19a>
    ac1d:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    ac21:	83 e0 01             	and    eax,0x1
    ac24:	48 85 c0             	test   rax,rax
    ac27:	75 37                	jne    ac60 <compare_raw+0xac>
    ac29:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    ac2d:	48 89 c7             	mov    rdi,rax
    ac30:	e8 95 c1 ff ff       	call   6dca <obj_type>
    ac35:	83 f8 01             	cmp    eax,0x1
    ac38:	74 1f                	je     ac59 <compare_raw+0xa5>
    ac3a:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    ac3e:	48 89 c7             	mov    rdi,rax
    ac41:	e8 84 c1 ff ff       	call   6dca <obj_type>
    ac46:	83 f8 04             	cmp    eax,0x4
    ac49:	75 07                	jne    ac52 <compare_raw+0x9e>
    ac4b:	b8 02 00 00 00       	mov    eax,0x2
    ac50:	eb 13                	jmp    ac65 <compare_raw+0xb1>
    ac52:	b8 03 00 00 00       	mov    eax,0x3
    ac57:	eb 0c                	jmp    ac65 <compare_raw+0xb1>
    ac59:	b8 01 00 00 00       	mov    eax,0x1
    ac5e:	eb 05                	jmp    ac65 <compare_raw+0xb1>
    ac60:	b8 00 00 00 00       	mov    eax,0x0
    ac65:	89 45 d8             	mov    DWORD PTR [rbp-0x28],eax
    ac68:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    ac6c:	83 e0 01             	and    eax,0x1
    ac6f:	48 85 c0             	test   rax,rax
    ac72:	75 37                	jne    acab <compare_raw+0xf7>
    ac74:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    ac78:	48 89 c7             	mov    rdi,rax
    ac7b:	e8 4a c1 ff ff       	call   6dca <obj_type>
    ac80:	83 f8 01             	cmp    eax,0x1
    ac83:	74 1f                	je     aca4 <compare_raw+0xf0>
    ac85:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    ac89:	48 89 c7             	mov    rdi,rax
    ac8c:	e8 39 c1 ff ff       	call   6dca <obj_type>
    ac91:	83 f8 04             	cmp    eax,0x4
    ac94:	75 07                	jne    ac9d <compare_raw+0xe9>
    ac96:	b8 02 00 00 00       	mov    eax,0x2
    ac9b:	eb 13                	jmp    acb0 <compare_raw+0xfc>
    ac9d:	b8 03 00 00 00       	mov    eax,0x3
    aca2:	eb 0c                	jmp    acb0 <compare_raw+0xfc>
    aca4:	b8 01 00 00 00       	mov    eax,0x1
    aca9:	eb 05                	jmp    acb0 <compare_raw+0xfc>
    acab:	b8 00 00 00 00       	mov    eax,0x0
    acb0:	89 45 dc             	mov    DWORD PTR [rbp-0x24],eax
    acb3:	8b 45 d8             	mov    eax,DWORD PTR [rbp-0x28]
    acb6:	3b 45 dc             	cmp    eax,DWORD PTR [rbp-0x24]
    acb9:	74 19                	je     acd4 <compare_raw+0x120>
    acbb:	8b 45 d8             	mov    eax,DWORD PTR [rbp-0x28]
    acbe:	3b 45 dc             	cmp    eax,DWORD PTR [rbp-0x24]
    acc1:	7d 0a                	jge    accd <compare_raw+0x119>
    acc3:	b8 ff ff ff ff       	mov    eax,0xffffffff
    acc8:	e9 81 00 00 00       	jmp    ad4e <compare_raw+0x19a>
    accd:	b8 01 00 00 00       	mov    eax,0x1
    acd2:	eb 7a                	jmp    ad4e <compare_raw+0x19a>
    acd4:	83 7d d8 01          	cmp    DWORD PTR [rbp-0x28],0x1
    acd8:	74 06                	je     ace0 <compare_raw+0x12c>
    acda:	83 7d d8 02          	cmp    DWORD PTR [rbp-0x28],0x2
    acde:	75 3a                	jne    ad1a <compare_raw+0x166>
    ace0:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    ace4:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    ace8:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    acec:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    acf0:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    acf4:	48 8b 48 10          	mov    rcx,QWORD PTR [rax+0x10]
    acf8:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    acfc:	48 8b 50 18          	mov    rdx,QWORD PTR [rax+0x18]
    ad00:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    ad04:	48 8b 70 10          	mov    rsi,QWORD PTR [rax+0x10]
    ad08:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    ad0c:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    ad10:	48 89 c7             	mov    rdi,rax
    ad13:	e8 10 fe ff ff       	call   ab28 <cmp_bytes>
    ad18:	eb 34                	jmp    ad4e <compare_raw+0x19a>
    ad1a:	48 8b 55 c0          	mov    rdx,QWORD PTR [rbp-0x40]
    ad1e:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    ad22:	48 89 d6             	mov    rsi,rdx
    ad25:	48 89 c7             	mov    rdi,rax
    ad28:	e8 d7 28 00 00       	call   d604 <cljn_equal_raw>
    ad2d:	85 c0                	test   eax,eax
    ad2f:	74 07                	je     ad38 <compare_raw+0x184>
    ad31:	b8 00 00 00 00       	mov    eax,0x0
    ad36:	eb 16                	jmp    ad4e <compare_raw+0x19a>
    ad38:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    ad3c:	48 3b 45 c0          	cmp    rax,QWORD PTR [rbp-0x40]
    ad40:	7d 07                	jge    ad49 <compare_raw+0x195>
    ad42:	b8 ff ff ff ff       	mov    eax,0xffffffff
    ad47:	eb 05                	jmp    ad4e <compare_raw+0x19a>
    ad49:	b8 01 00 00 00       	mov    eax,0x1
    ad4e:	c9                   	leave
    ad4f:	c3                   	ret

000000000000ad50 <cljn_compare>:
    ad50:	f3 0f 1e fa          	endbr64
    ad54:	55                   	push   rbp
    ad55:	48 89 e5             	mov    rbp,rsp
    ad58:	48 83 ec 10          	sub    rsp,0x10
    ad5c:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    ad60:	48 89 75 f0          	mov    QWORD PTR [rbp-0x10],rsi
    ad64:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    ad68:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    ad6c:	48 89 d6             	mov    rsi,rdx
    ad6f:	48 89 c7             	mov    rdi,rax
    ad72:	e8 3d fe ff ff       	call   abb4 <compare_raw>
    ad77:	48 98                	cdqe
    ad79:	48 01 c0             	add    rax,rax
    ad7c:	48 83 c8 01          	or     rax,0x1
    ad80:	c9                   	leave
    ad81:	c3                   	ret

000000000000ad82 <cljn_vec_pair>:
    ad82:	f3 0f 1e fa          	endbr64
    ad86:	55                   	push   rbp
    ad87:	48 89 e5             	mov    rbp,rsp
    ad8a:	48 83 ec 20          	sub    rsp,0x20
    ad8e:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    ad92:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    ad96:	8b 05 fc 92 00 02    	mov    eax,DWORD PTR [rip+0x20092fc]        # 2014098 <gc_disabled>
    ad9c:	83 c0 01             	add    eax,0x1
    ad9f:	89 05 f3 92 00 02    	mov    DWORD PTR [rip+0x20092f3],eax        # 2014098 <gc_disabled>
    ada5:	e8 85 d1 ff ff       	call   7f2f <cljn_vec_empty>
    adaa:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    adae:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    adb2:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    adb6:	48 89 d6             	mov    rsi,rdx
    adb9:	48 89 c7             	mov    rdi,rax
    adbc:	e8 fb d3 ff ff       	call   81bc <cljn_vec_conj>
    adc1:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    adc5:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    adc9:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    adcd:	48 89 d6             	mov    rsi,rdx
    add0:	48 89 c7             	mov    rdi,rax
    add3:	e8 e4 d3 ff ff       	call   81bc <cljn_vec_conj>
    add8:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    addc:	8b 05 b6 92 00 02    	mov    eax,DWORD PTR [rip+0x20092b6]        # 2014098 <gc_disabled>
    ade2:	83 e8 01             	sub    eax,0x1
    ade5:	89 05 ad 92 00 02    	mov    DWORD PTR [rip+0x20092ad],eax        # 2014098 <gc_disabled>
    adeb:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    adef:	c9                   	leave
    adf0:	c3                   	ret

000000000000adf1 <tn_red>:
    adf1:	f3 0f 1e fa          	endbr64
    adf5:	55                   	push   rbp
    adf6:	48 89 e5             	mov    rbp,rsp
    adf9:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    adfd:	48 83 7d f8 02       	cmp    QWORD PTR [rbp-0x8],0x2
    ae02:	74 14                	je     ae18 <tn_red+0x27>
    ae04:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    ae08:	48 8b 40 30          	mov    rax,QWORD PTR [rax+0x30]
    ae0c:	48 85 c0             	test   rax,rax
    ae0f:	74 07                	je     ae18 <tn_red+0x27>
    ae11:	b8 01 00 00 00       	mov    eax,0x1
    ae16:	eb 05                	jmp    ae1d <tn_red+0x2c>
    ae18:	b8 00 00 00 00       	mov    eax,0x0
    ae1d:	5d                   	pop    rbp
    ae1e:	c3                   	ret

000000000000ae1f <tn_alloc>:
    ae1f:	f3 0f 1e fa          	endbr64
    ae23:	55                   	push   rbp
    ae24:	48 89 e5             	mov    rbp,rsp
    ae27:	48 83 ec 40          	sub    rsp,0x40
    ae2b:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    ae2f:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    ae33:	48 89 55 d8          	mov    QWORD PTR [rbp-0x28],rdx
    ae37:	48 89 4d d0          	mov    QWORD PTR [rbp-0x30],rcx
    ae3b:	44 89 45 cc          	mov    DWORD PTR [rbp-0x34],r8d
    ae3f:	be 0e 00 00 00       	mov    esi,0xe
    ae44:	bf 38 00 00 00       	mov    edi,0x38
    ae49:	e8 25 c1 ff ff       	call   6f73 <obj_alloc>
    ae4e:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    ae52:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    ae56:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    ae5a:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    ae5e:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    ae62:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    ae66:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    ae6a:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    ae6e:	48 8b 55 d8          	mov    rdx,QWORD PTR [rbp-0x28]
    ae72:	48 89 50 20          	mov    QWORD PTR [rax+0x20],rdx
    ae76:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    ae7a:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    ae7e:	48 89 50 28          	mov    QWORD PTR [rax+0x28],rdx
    ae82:	8b 45 cc             	mov    eax,DWORD PTR [rbp-0x34]
    ae85:	48 63 d0             	movsxd rdx,eax
    ae88:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    ae8c:	48 89 50 30          	mov    QWORD PTR [rax+0x30],rdx
    ae90:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    ae94:	c9                   	leave
    ae95:	c3                   	ret

000000000000ae96 <tn_copy>:
    ae96:	f3 0f 1e fa          	endbr64
    ae9a:	55                   	push   rbp
    ae9b:	48 89 e5             	mov    rbp,rsp
    ae9e:	48 83 ec 20          	sub    rsp,0x20
    aea2:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    aea6:	48 89 75 f0          	mov    QWORD PTR [rbp-0x10],rsi
    aeaa:	48 89 55 e8          	mov    QWORD PTR [rbp-0x18],rdx
    aeae:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    aeb2:	48 8b 40 30          	mov    rax,QWORD PTR [rax+0x30]
    aeb6:	89 c7                	mov    edi,eax
    aeb8:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    aebc:	48 8b 70 18          	mov    rsi,QWORD PTR [rax+0x18]
    aec0:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    aec4:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    aec8:	48 8b 4d e8          	mov    rcx,QWORD PTR [rbp-0x18]
    aecc:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    aed0:	41 89 f8             	mov    r8d,edi
    aed3:	48 89 c7             	mov    rdi,rax
    aed6:	e8 44 ff ff ff       	call   ae1f <tn_alloc>
    aedb:	c9                   	leave
    aedc:	c3                   	ret

000000000000aedd <tn_rot_left>:
    aedd:	f3 0f 1e fa          	endbr64
    aee1:	55                   	push   rbp
    aee2:	48 89 e5             	mov    rbp,rsp
    aee5:	48 83 ec 20          	sub    rsp,0x20
    aee9:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    aeed:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    aef1:	48 8b 40 28          	mov    rax,QWORD PTR [rax+0x28]
    aef5:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    aef9:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    aefd:	48 8b 48 20          	mov    rcx,QWORD PTR [rax+0x20]
    af01:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    af05:	48 8b 50 20          	mov    rdx,QWORD PTR [rax+0x20]
    af09:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    af0d:	48 8b 70 18          	mov    rsi,QWORD PTR [rax+0x18]
    af11:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    af15:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    af19:	41 b8 01 00 00 00    	mov    r8d,0x1
    af1f:	48 89 c7             	mov    rdi,rax
    af22:	e8 f8 fe ff ff       	call   ae1f <tn_alloc>
    af27:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    af2b:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    af2f:	48 8b 40 30          	mov    rax,QWORD PTR [rax+0x30]
    af33:	89 c7                	mov    edi,eax
    af35:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    af39:	48 8b 48 28          	mov    rcx,QWORD PTR [rax+0x28]
    af3d:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    af41:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    af45:	48 8b 70 18          	mov    rsi,QWORD PTR [rax+0x18]
    af49:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    af4d:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    af51:	41 89 f8             	mov    r8d,edi
    af54:	48 89 c7             	mov    rdi,rax
    af57:	e8 c3 fe ff ff       	call   ae1f <tn_alloc>
    af5c:	c9                   	leave
    af5d:	c3                   	ret

000000000000af5e <tn_rot_right>:
    af5e:	f3 0f 1e fa          	endbr64
    af62:	55                   	push   rbp
    af63:	48 89 e5             	mov    rbp,rsp
    af66:	48 83 ec 20          	sub    rsp,0x20
    af6a:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    af6e:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    af72:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    af76:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    af7a:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    af7e:	48 8b 48 28          	mov    rcx,QWORD PTR [rax+0x28]
    af82:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    af86:	48 8b 50 28          	mov    rdx,QWORD PTR [rax+0x28]
    af8a:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    af8e:	48 8b 70 18          	mov    rsi,QWORD PTR [rax+0x18]
    af92:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    af96:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    af9a:	41 b8 01 00 00 00    	mov    r8d,0x1
    afa0:	48 89 c7             	mov    rdi,rax
    afa3:	e8 77 fe ff ff       	call   ae1f <tn_alloc>
    afa8:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    afac:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    afb0:	48 8b 40 30          	mov    rax,QWORD PTR [rax+0x30]
    afb4:	89 c7                	mov    edi,eax
    afb6:	48 8b 4d f8          	mov    rcx,QWORD PTR [rbp-0x8]
    afba:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    afbe:	48 8b 50 20          	mov    rdx,QWORD PTR [rax+0x20]
    afc2:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    afc6:	48 8b 70 18          	mov    rsi,QWORD PTR [rax+0x18]
    afca:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    afce:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    afd2:	41 89 f8             	mov    r8d,edi
    afd5:	48 89 c7             	mov    rdi,rax
    afd8:	e8 42 fe ff ff       	call   ae1f <tn_alloc>
    afdd:	c9                   	leave
    afde:	c3                   	ret

000000000000afdf <tn_flip>:
    afdf:	f3 0f 1e fa          	endbr64
    afe3:	55                   	push   rbp
    afe4:	48 89 e5             	mov    rbp,rsp
    afe7:	48 83 ec 30          	sub    rsp,0x30
    afeb:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    afef:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    aff3:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    aff7:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    affb:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    afff:	48 8b 40 28          	mov    rax,QWORD PTR [rax+0x28]
    b003:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    b007:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    b00b:	48 8b 40 30          	mov    rax,QWORD PTR [rax+0x30]
    b00f:	48 85 c0             	test   rax,rax
    b012:	0f 94 c0             	sete   al
    b015:	0f b6 f8             	movzx  edi,al
    b018:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    b01c:	48 8b 48 28          	mov    rcx,QWORD PTR [rax+0x28]
    b020:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    b024:	48 8b 50 20          	mov    rdx,QWORD PTR [rax+0x20]
    b028:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    b02c:	48 8b 70 18          	mov    rsi,QWORD PTR [rax+0x18]
    b030:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    b034:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    b038:	41 89 f8             	mov    r8d,edi
    b03b:	48 89 c7             	mov    rdi,rax
    b03e:	e8 dc fd ff ff       	call   ae1f <tn_alloc>
    b043:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    b047:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b04b:	48 8b 40 30          	mov    rax,QWORD PTR [rax+0x30]
    b04f:	48 85 c0             	test   rax,rax
    b052:	0f 94 c0             	sete   al
    b055:	0f b6 f8             	movzx  edi,al
    b058:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b05c:	48 8b 48 28          	mov    rcx,QWORD PTR [rax+0x28]
    b060:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b064:	48 8b 50 20          	mov    rdx,QWORD PTR [rax+0x20]
    b068:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b06c:	48 8b 70 18          	mov    rsi,QWORD PTR [rax+0x18]
    b070:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b074:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    b078:	41 89 f8             	mov    r8d,edi
    b07b:	48 89 c7             	mov    rdi,rax
    b07e:	e8 9c fd ff ff       	call   ae1f <tn_alloc>
    b083:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    b087:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    b08b:	48 8b 40 30          	mov    rax,QWORD PTR [rax+0x30]
    b08f:	48 85 c0             	test   rax,rax
    b092:	0f 94 c0             	sete   al
    b095:	0f b6 f8             	movzx  edi,al
    b098:	48 8b 4d f8          	mov    rcx,QWORD PTR [rbp-0x8]
    b09c:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    b0a0:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    b0a4:	48 8b 70 18          	mov    rsi,QWORD PTR [rax+0x18]
    b0a8:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    b0ac:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    b0b0:	41 89 f8             	mov    r8d,edi
    b0b3:	48 89 c7             	mov    rdi,rax
    b0b6:	e8 64 fd ff ff       	call   ae1f <tn_alloc>
    b0bb:	c9                   	leave
    b0bc:	c3                   	ret

000000000000b0bd <tn_fixup>:
    b0bd:	f3 0f 1e fa          	endbr64
    b0c1:	55                   	push   rbp
    b0c2:	48 89 e5             	mov    rbp,rsp
    b0c5:	48 83 ec 20          	sub    rsp,0x20
    b0c9:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    b0cd:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b0d1:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    b0d5:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b0d9:	48 8b 40 28          	mov    rax,QWORD PTR [rax+0x28]
    b0dd:	48 89 c7             	mov    rdi,rax
    b0e0:	e8 0c fd ff ff       	call   adf1 <tn_red>
    b0e5:	85 c0                	test   eax,eax
    b0e7:	74 2c                	je     b115 <tn_fixup+0x58>
    b0e9:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b0ed:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    b0f1:	48 89 c7             	mov    rdi,rax
    b0f4:	e8 f8 fc ff ff       	call   adf1 <tn_red>
    b0f9:	85 c0                	test   eax,eax
    b0fb:	75 18                	jne    b115 <tn_fixup+0x58>
    b0fd:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b101:	48 89 c7             	mov    rdi,rax
    b104:	e8 d4 fd ff ff       	call   aedd <tn_rot_left>
    b109:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    b10d:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b111:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    b115:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b119:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    b11d:	48 89 c7             	mov    rdi,rax
    b120:	e8 cc fc ff ff       	call   adf1 <tn_red>
    b125:	85 c0                	test   eax,eax
    b127:	74 30                	je     b159 <tn_fixup+0x9c>
    b129:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b12d:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    b131:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    b135:	48 89 c7             	mov    rdi,rax
    b138:	e8 b4 fc ff ff       	call   adf1 <tn_red>
    b13d:	85 c0                	test   eax,eax
    b13f:	74 18                	je     b159 <tn_fixup+0x9c>
    b141:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b145:	48 89 c7             	mov    rdi,rax
    b148:	e8 11 fe ff ff       	call   af5e <tn_rot_right>
    b14d:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    b151:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b155:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    b159:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b15d:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    b161:	48 89 c7             	mov    rdi,rax
    b164:	e8 88 fc ff ff       	call   adf1 <tn_red>
    b169:	85 c0                	test   eax,eax
    b16b:	74 24                	je     b191 <tn_fixup+0xd4>
    b16d:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b171:	48 8b 40 28          	mov    rax,QWORD PTR [rax+0x28]
    b175:	48 89 c7             	mov    rdi,rax
    b178:	e8 74 fc ff ff       	call   adf1 <tn_red>
    b17d:	85 c0                	test   eax,eax
    b17f:	74 10                	je     b191 <tn_fixup+0xd4>
    b181:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b185:	48 89 c7             	mov    rdi,rax
    b188:	e8 52 fe ff ff       	call   afdf <tn_flip>
    b18d:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    b191:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b195:	c9                   	leave
    b196:	c3                   	ret

000000000000b197 <tn_insert>:
    b197:	f3 0f 1e fa          	endbr64
    b19b:	55                   	push   rbp
    b19c:	48 89 e5             	mov    rbp,rsp
    b19f:	53                   	push   rbx
    b1a0:	48 83 ec 38          	sub    rsp,0x38
    b1a4:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    b1a8:	48 89 75 d0          	mov    QWORD PTR [rbp-0x30],rsi
    b1ac:	48 89 55 c8          	mov    QWORD PTR [rbp-0x38],rdx
    b1b0:	48 89 4d c0          	mov    QWORD PTR [rbp-0x40],rcx
    b1b4:	48 83 7d d8 02       	cmp    QWORD PTR [rbp-0x28],0x2
    b1b9:	75 2f                	jne    b1ea <tn_insert+0x53>
    b1bb:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    b1bf:	c7 00 01 00 00 00    	mov    DWORD PTR [rax],0x1
    b1c5:	48 8b 75 c8          	mov    rsi,QWORD PTR [rbp-0x38]
    b1c9:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    b1cd:	41 b8 01 00 00 00    	mov    r8d,0x1
    b1d3:	b9 02 00 00 00       	mov    ecx,0x2
    b1d8:	ba 02 00 00 00       	mov    edx,0x2
    b1dd:	48 89 c7             	mov    rdi,rax
    b1e0:	e8 3a fc ff ff       	call   ae1f <tn_alloc>
    b1e5:	e9 e4 00 00 00       	jmp    b2ce <tn_insert+0x137>
    b1ea:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    b1ee:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    b1f2:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b1f6:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    b1fa:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    b1fe:	48 89 d6             	mov    rsi,rdx
    b201:	48 89 c7             	mov    rdi,rax
    b204:	e8 ab f9 ff ff       	call   abb4 <compare_raw>
    b209:	89 45 e4             	mov    DWORD PTR [rbp-0x1c],eax
    b20c:	83 7d e4 00          	cmp    DWORD PTR [rbp-0x1c],0x0
    b210:	75 31                	jne    b243 <tn_insert+0xac>
    b212:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b216:	48 8b 50 28          	mov    rdx,QWORD PTR [rax+0x28]
    b21a:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b21e:	48 8b 48 20          	mov    rcx,QWORD PTR [rax+0x20]
    b222:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b226:	48 89 ce             	mov    rsi,rcx
    b229:	48 89 c7             	mov    rdi,rax
    b22c:	e8 65 fc ff ff       	call   ae96 <tn_copy>
    b231:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    b235:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    b239:	48 8b 55 c8          	mov    rdx,QWORD PTR [rbp-0x38]
    b23d:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    b241:	eb 7f                	jmp    b2c2 <tn_insert+0x12b>
    b243:	83 7d e4 00          	cmp    DWORD PTR [rbp-0x1c],0x0
    b247:	79 3f                	jns    b288 <tn_insert+0xf1>
    b249:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b24d:	48 8b 58 28          	mov    rbx,QWORD PTR [rax+0x28]
    b251:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b255:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    b259:	48 8b 4d c0          	mov    rcx,QWORD PTR [rbp-0x40]
    b25d:	48 8b 55 c8          	mov    rdx,QWORD PTR [rbp-0x38]
    b261:	48 8b 75 d0          	mov    rsi,QWORD PTR [rbp-0x30]
    b265:	48 89 c7             	mov    rdi,rax
    b268:	e8 2a ff ff ff       	call   b197 <tn_insert>
    b26d:	48 89 c1             	mov    rcx,rax
    b270:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b274:	48 89 da             	mov    rdx,rbx
    b277:	48 89 ce             	mov    rsi,rcx
    b27a:	48 89 c7             	mov    rdi,rax
    b27d:	e8 14 fc ff ff       	call   ae96 <tn_copy>
    b282:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    b286:	eb 3a                	jmp    b2c2 <tn_insert+0x12b>
    b288:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b28c:	48 8b 40 28          	mov    rax,QWORD PTR [rax+0x28]
    b290:	48 8b 4d c0          	mov    rcx,QWORD PTR [rbp-0x40]
    b294:	48 8b 55 c8          	mov    rdx,QWORD PTR [rbp-0x38]
    b298:	48 8b 75 d0          	mov    rsi,QWORD PTR [rbp-0x30]
    b29c:	48 89 c7             	mov    rdi,rax
    b29f:	e8 f3 fe ff ff       	call   b197 <tn_insert>
    b2a4:	48 89 c2             	mov    rdx,rax
    b2a7:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b2ab:	48 8b 48 20          	mov    rcx,QWORD PTR [rax+0x20]
    b2af:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b2b3:	48 89 ce             	mov    rsi,rcx
    b2b6:	48 89 c7             	mov    rdi,rax
    b2b9:	e8 d8 fb ff ff       	call   ae96 <tn_copy>
    b2be:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    b2c2:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    b2c6:	48 89 c7             	mov    rdi,rax
    b2c9:	e8 ef fd ff ff       	call   b0bd <tn_fixup>
    b2ce:	48 8b 5d f8          	mov    rbx,QWORD PTR [rbp-0x8]
    b2d2:	c9                   	leave
    b2d3:	c3                   	ret

000000000000b2d4 <tn_get>:
    b2d4:	f3 0f 1e fa          	endbr64
    b2d8:	55                   	push   rbp
    b2d9:	48 89 e5             	mov    rbp,rsp
    b2dc:	48 83 ec 20          	sub    rsp,0x20
    b2e0:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    b2e4:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    b2e8:	eb 4e                	jmp    b338 <tn_get+0x64>
    b2ea:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b2ee:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    b2f2:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b2f6:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    b2fa:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    b2fe:	48 89 d6             	mov    rsi,rdx
    b301:	48 89 c7             	mov    rdi,rax
    b304:	e8 ab f8 ff ff       	call   abb4 <compare_raw>
    b309:	89 45 f4             	mov    DWORD PTR [rbp-0xc],eax
    b30c:	83 7d f4 00          	cmp    DWORD PTR [rbp-0xc],0x0
    b310:	75 0a                	jne    b31c <tn_get+0x48>
    b312:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b316:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    b31a:	eb 28                	jmp    b344 <tn_get+0x70>
    b31c:	83 7d f4 00          	cmp    DWORD PTR [rbp-0xc],0x0
    b320:	79 0a                	jns    b32c <tn_get+0x58>
    b322:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b326:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    b32a:	eb 08                	jmp    b334 <tn_get+0x60>
    b32c:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b330:	48 8b 40 28          	mov    rax,QWORD PTR [rax+0x28]
    b334:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    b338:	48 83 7d e8 02       	cmp    QWORD PTR [rbp-0x18],0x2
    b33d:	75 ab                	jne    b2ea <tn_get+0x16>
    b33f:	b8 2a 00 00 00       	mov    eax,0x2a
    b344:	c9                   	leave
    b345:	c3                   	ret

000000000000b346 <tn_walk_desc>:
    b346:	f3 0f 1e fa          	endbr64
    b34a:	55                   	push   rbp
    b34b:	48 89 e5             	mov    rbp,rsp
    b34e:	48 83 ec 30          	sub    rsp,0x30
    b352:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    b356:	89 75 d4             	mov    DWORD PTR [rbp-0x2c],esi
    b359:	48 83 7d d8 02       	cmp    QWORD PTR [rbp-0x28],0x2
    b35e:	0f 84 ce 00 00 00    	je     b432 <tn_walk_desc+0xec>
    b364:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    b368:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    b36c:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    b370:	48 8b 40 28          	mov    rax,QWORD PTR [rax+0x28]
    b374:	8b 55 d4             	mov    edx,DWORD PTR [rbp-0x2c]
    b377:	89 d6                	mov    esi,edx
    b379:	48 89 c7             	mov    rdi,rax
    b37c:	e8 c5 ff ff ff       	call   b346 <tn_walk_desc>
    b381:	83 7d d4 00          	cmp    DWORD PTR [rbp-0x2c],0x0
    b385:	75 0e                	jne    b395 <tn_walk_desc+0x4f>
    b387:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    b38b:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    b38f:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    b393:	eb 33                	jmp    b3c8 <tn_walk_desc+0x82>
    b395:	83 7d d4 01          	cmp    DWORD PTR [rbp-0x2c],0x1
    b399:	75 0e                	jne    b3a9 <tn_walk_desc+0x63>
    b39b:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    b39f:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    b3a3:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    b3a7:	eb 1f                	jmp    b3c8 <tn_walk_desc+0x82>
    b3a9:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    b3ad:	48 8b 50 18          	mov    rdx,QWORD PTR [rax+0x18]
    b3b1:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    b3b5:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    b3b9:	48 89 d6             	mov    rsi,rdx
    b3bc:	48 89 c7             	mov    rdi,rax
    b3bf:	e8 be f9 ff ff       	call   ad82 <cljn_vec_pair>
    b3c4:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    b3c8:	48 8b 05 b1 8c 00 02 	mov    rax,QWORD PTR [rip+0x2008cb1]        # 2014080 <gc_sp>
    b3cf:	48 83 e8 01          	sub    rax,0x1
    b3d3:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    b3da:	00 
    b3db:	48 8d 05 9e 8c 00 00 	lea    rax,[rip+0x8c9e]        # 14080 <gc_stack>
    b3e2:	48 8b 14 02          	mov    rdx,QWORD PTR [rdx+rax*1]
    b3e6:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b3ea:	48 89 d6             	mov    rsi,rdx
    b3ed:	48 89 c7             	mov    rdi,rax
    b3f0:	e8 ec c2 ff ff       	call   76e1 <cljn_cons>
    b3f5:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    b3f9:	48 8b 05 80 8c 00 02 	mov    rax,QWORD PTR [rip+0x2008c80]        # 2014080 <gc_sp>
    b400:	48 83 e8 01          	sub    rax,0x1
    b404:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    b40b:	00 
    b40c:	48 8d 15 6d 8c 00 00 	lea    rdx,[rip+0x8c6d]        # 14080 <gc_stack>
    b413:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b417:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    b41b:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    b41f:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    b423:	8b 55 d4             	mov    edx,DWORD PTR [rbp-0x2c]
    b426:	89 d6                	mov    esi,edx
    b428:	48 89 c7             	mov    rdi,rax
    b42b:	e8 16 ff ff ff       	call   b346 <tn_walk_desc>
    b430:	eb 01                	jmp    b433 <tn_walk_desc+0xed>
    b432:	90                   	nop
    b433:	c9                   	leave
    b434:	c3                   	ret

000000000000b435 <sorted_alloc>:
    b435:	f3 0f 1e fa          	endbr64
    b439:	55                   	push   rbp
    b43a:	48 89 e5             	mov    rbp,rsp
    b43d:	48 83 ec 20          	sub    rsp,0x20
    b441:	89 7d ec             	mov    DWORD PTR [rbp-0x14],edi
    b444:	8b 45 ec             	mov    eax,DWORD PTR [rbp-0x14]
    b447:	89 c6                	mov    esi,eax
    b449:	bf 20 00 00 00       	mov    edi,0x20
    b44e:	e8 20 bb ff ff       	call   6f73 <obj_alloc>
    b453:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    b457:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b45b:	48 c7 40 10 00 00 00 	mov    QWORD PTR [rax+0x10],0x0
    b462:	00 
    b463:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b467:	48 c7 40 18 02 00 00 	mov    QWORD PTR [rax+0x18],0x2
    b46e:	00 
    b46f:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b473:	c9                   	leave
    b474:	c3                   	ret

000000000000b475 <cljn_sorted_map_empty>:
    b475:	f3 0f 1e fa          	endbr64
    b479:	55                   	push   rbp
    b47a:	48 89 e5             	mov    rbp,rsp
    b47d:	bf 0f 00 00 00       	mov    edi,0xf
    b482:	e8 ae ff ff ff       	call   b435 <sorted_alloc>
    b487:	5d                   	pop    rbp
    b488:	c3                   	ret

000000000000b489 <cljn_sorted_set_empty>:
    b489:	f3 0f 1e fa          	endbr64
    b48d:	55                   	push   rbp
    b48e:	48 89 e5             	mov    rbp,rsp
    b491:	bf 10 00 00 00       	mov    edi,0x10
    b496:	e8 9a ff ff ff       	call   b435 <sorted_alloc>
    b49b:	5d                   	pop    rbp
    b49c:	c3                   	ret

000000000000b49d <cljn_sorted_assoc>:
    b49d:	f3 0f 1e fa          	endbr64
    b4a1:	55                   	push   rbp
    b4a2:	48 89 e5             	mov    rbp,rsp
    b4a5:	48 83 ec 50          	sub    rsp,0x50
    b4a9:	48 89 7d c8          	mov    QWORD PTR [rbp-0x38],rdi
    b4ad:	48 89 75 c0          	mov    QWORD PTR [rbp-0x40],rsi
    b4b1:	48 89 55 b8          	mov    QWORD PTR [rbp-0x48],rdx
    b4b5:	64 48 8b 04 25 28 00 	mov    rax,QWORD PTR fs:0x28
    b4bc:	00 00 
    b4be:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    b4c2:	31 c0                	xor    eax,eax
    b4c4:	e8 58 ba ff ff       	call   6f21 <maybe_gc>
    b4c9:	8b 05 c9 8b 00 02    	mov    eax,DWORD PTR [rip+0x2008bc9]        # 2014098 <gc_disabled>
    b4cf:	83 c0 01             	add    eax,0x1
    b4d2:	89 05 c0 8b 00 02    	mov    DWORD PTR [rip+0x2008bc0],eax        # 2014098 <gc_disabled>
    b4d8:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    b4dc:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    b4e0:	c7 45 dc 00 00 00 00 	mov    DWORD PTR [rbp-0x24],0x0
    b4e7:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    b4eb:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    b4ef:	48 8d 4d dc          	lea    rcx,[rbp-0x24]
    b4f3:	48 8b 55 b8          	mov    rdx,QWORD PTR [rbp-0x48]
    b4f7:	48 8b 75 c0          	mov    rsi,QWORD PTR [rbp-0x40]
    b4fb:	48 89 c7             	mov    rdi,rax
    b4fe:	e8 94 fc ff ff       	call   b197 <tn_insert>
    b503:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    b507:	48 83 7d e8 02       	cmp    QWORD PTR [rbp-0x18],0x2
    b50c:	74 0c                	je     b51a <cljn_sorted_assoc+0x7d>
    b50e:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b512:	48 c7 40 30 00 00 00 	mov    QWORD PTR [rax+0x30],0x0
    b519:	00 
    b51a:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    b51e:	48 89 c7             	mov    rdi,rax
    b521:	e8 a4 b8 ff ff       	call   6dca <obj_type>
    b526:	89 c6                	mov    esi,eax
    b528:	bf 20 00 00 00       	mov    edi,0x20
    b52d:	e8 41 ba ff ff       	call   6f73 <obj_alloc>
    b532:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    b536:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    b53a:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    b53e:	8b 45 dc             	mov    eax,DWORD PTR [rbp-0x24]
    b541:	48 98                	cdqe
    b543:	48 01 c2             	add    rdx,rax
    b546:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    b54a:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    b54e:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    b552:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    b556:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    b55a:	8b 05 38 8b 00 02    	mov    eax,DWORD PTR [rip+0x2008b38]        # 2014098 <gc_disabled>
    b560:	83 e8 01             	sub    eax,0x1
    b563:	89 05 2f 8b 00 02    	mov    DWORD PTR [rip+0x2008b2f],eax        # 2014098 <gc_disabled>
    b569:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    b56d:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    b571:	64 48 2b 14 25 28 00 	sub    rdx,QWORD PTR fs:0x28
    b578:	00 00 
    b57a:	74 05                	je     b581 <cljn_sorted_assoc+0xe4>
    b57c:	e8 df 5a ff ff       	call   1060 <__stack_chk_fail@plt>
    b581:	c9                   	leave
    b582:	c3                   	ret

000000000000b583 <cljn_sorted_set_conj>:
    b583:	f3 0f 1e fa          	endbr64
    b587:	55                   	push   rbp
    b588:	48 89 e5             	mov    rbp,rsp
    b58b:	48 83 ec 10          	sub    rsp,0x10
    b58f:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    b593:	48 89 75 f0          	mov    QWORD PTR [rbp-0x10],rsi
    b597:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    b59b:	48 8b 4d f0          	mov    rcx,QWORD PTR [rbp-0x10]
    b59f:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b5a3:	48 89 ce             	mov    rsi,rcx
    b5a6:	48 89 c7             	mov    rdi,rax
    b5a9:	e8 ef fe ff ff       	call   b49d <cljn_sorted_assoc>
    b5ae:	c9                   	leave
    b5af:	c3                   	ret

000000000000b5b0 <cljn_sorted_get>:
    b5b0:	f3 0f 1e fa          	endbr64
    b5b4:	55                   	push   rbp
    b5b5:	48 89 e5             	mov    rbp,rsp
    b5b8:	48 83 ec 20          	sub    rsp,0x20
    b5bc:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    b5c0:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    b5c4:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b5c8:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    b5cc:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    b5d0:	48 89 d6             	mov    rsi,rdx
    b5d3:	48 89 c7             	mov    rdi,rax
    b5d6:	e8 f9 fc ff ff       	call   b2d4 <tn_get>
    b5db:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    b5df:	48 83 7d f8 2a       	cmp    QWORD PTR [rbp-0x8],0x2a
    b5e4:	74 06                	je     b5ec <cljn_sorted_get+0x3c>
    b5e6:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b5ea:	eb 05                	jmp    b5f1 <cljn_sorted_get+0x41>
    b5ec:	b8 02 00 00 00       	mov    eax,0x2
    b5f1:	c9                   	leave
    b5f2:	c3                   	ret

000000000000b5f3 <cljn_sorted_contains>:
    b5f3:	f3 0f 1e fa          	endbr64
    b5f7:	55                   	push   rbp
    b5f8:	48 89 e5             	mov    rbp,rsp
    b5fb:	48 83 ec 10          	sub    rsp,0x10
    b5ff:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    b603:	48 89 75 f0          	mov    QWORD PTR [rbp-0x10],rsi
    b607:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b60b:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    b60f:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    b613:	48 89 d6             	mov    rsi,rdx
    b616:	48 89 c7             	mov    rdi,rax
    b619:	e8 b6 fc ff ff       	call   b2d4 <tn_get>
    b61e:	48 83 f8 2a          	cmp    rax,0x2a
    b622:	0f 95 c0             	setne  al
    b625:	0f b6 c0             	movzx  eax,al
    b628:	89 c7                	mov    edi,eax
    b62a:	e8 69 1d 00 00       	call   d398 <b2v>
    b62f:	c9                   	leave
    b630:	c3                   	ret

000000000000b631 <cljn_sorted_first>:
    b631:	f3 0f 1e fa          	endbr64
    b635:	55                   	push   rbp
    b636:	48 89 e5             	mov    rbp,rsp
    b639:	48 83 ec 20          	sub    rsp,0x20
    b63d:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    b641:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b645:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    b649:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    b64d:	48 83 7d f8 02       	cmp    QWORD PTR [rbp-0x8],0x2
    b652:	75 07                	jne    b65b <cljn_sorted_first+0x2a>
    b654:	b8 02 00 00 00       	mov    eax,0x2
    b659:	eb 5a                	jmp    b6b5 <cljn_sorted_first+0x84>
    b65b:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b65f:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    b663:	eb 0c                	jmp    b671 <cljn_sorted_first+0x40>
    b665:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    b669:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    b66d:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    b671:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    b675:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    b679:	48 83 f8 02          	cmp    rax,0x2
    b67d:	75 e6                	jne    b665 <cljn_sorted_first+0x34>
    b67f:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b683:	48 89 c7             	mov    rdi,rax
    b686:	e8 3f b7 ff ff       	call   6dca <obj_type>
    b68b:	83 f8 10             	cmp    eax,0x10
    b68e:	75 0a                	jne    b69a <cljn_sorted_first+0x69>
    b690:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    b694:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    b698:	eb 1b                	jmp    b6b5 <cljn_sorted_first+0x84>
    b69a:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    b69e:	48 8b 50 18          	mov    rdx,QWORD PTR [rax+0x18]
    b6a2:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    b6a6:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    b6aa:	48 89 d6             	mov    rsi,rdx
    b6ad:	48 89 c7             	mov    rdi,rax
    b6b0:	e8 cd f6 ff ff       	call   ad82 <cljn_vec_pair>
    b6b5:	c9                   	leave
    b6b6:	c3                   	ret

000000000000b6b7 <sorted_seq>:
    b6b7:	f3 0f 1e fa          	endbr64
    b6bb:	55                   	push   rbp
    b6bc:	48 89 e5             	mov    rbp,rsp
    b6bf:	48 83 ec 20          	sub    rsp,0x20
    b6c3:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    b6c7:	89 75 e4             	mov    DWORD PTR [rbp-0x1c],esi
    b6ca:	bf 12 00 00 00       	mov    edi,0x12
    b6cf:	e8 90 b5 ff ff       	call   6c64 <cljn_gc_push>
    b6d4:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b6d8:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    b6dc:	8b 55 e4             	mov    edx,DWORD PTR [rbp-0x1c]
    b6df:	89 d6                	mov    esi,edx
    b6e1:	48 89 c7             	mov    rdi,rax
    b6e4:	e8 5d fc ff ff       	call   b346 <tn_walk_desc>
    b6e9:	48 8b 05 90 89 00 02 	mov    rax,QWORD PTR [rip+0x2008990]        # 2014080 <gc_sp>
    b6f0:	48 83 e8 01          	sub    rax,0x1
    b6f4:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    b6fb:	00 
    b6fc:	48 8d 05 7d 89 00 00 	lea    rax,[rip+0x897d]        # 14080 <gc_stack>
    b703:	48 8b 04 02          	mov    rax,QWORD PTR [rdx+rax*1]
    b707:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    b70b:	bf 01 00 00 00       	mov    edi,0x1
    b710:	e8 c7 b5 ff ff       	call   6cdc <cljn_gc_popn>
    b715:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b719:	c9                   	leave
    b71a:	c3                   	ret

000000000000b71b <tn_all_in>:
    b71b:	f3 0f 1e fa          	endbr64
    b71f:	55                   	push   rbp
    b720:	48 89 e5             	mov    rbp,rsp
    b723:	48 83 ec 20          	sub    rsp,0x20
    b727:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    b72b:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    b72f:	48 83 7d e8 02       	cmp    QWORD PTR [rbp-0x18],0x2
    b734:	75 07                	jne    b73d <tn_all_in+0x22>
    b736:	b8 01 00 00 00       	mov    eax,0x1
    b73b:	eb 6b                	jmp    b7a8 <tn_all_in+0x8d>
    b73d:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b741:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    b745:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b749:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    b74d:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    b751:	48 89 d6             	mov    rsi,rdx
    b754:	48 89 c7             	mov    rdi,rax
    b757:	e8 bf ff ff ff       	call   b71b <tn_all_in>
    b75c:	85 c0                	test   eax,eax
    b75e:	75 07                	jne    b767 <tn_all_in+0x4c>
    b760:	b8 00 00 00 00       	mov    eax,0x0
    b765:	eb 41                	jmp    b7a8 <tn_all_in+0x8d>
    b767:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b76b:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    b76f:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    b773:	48 89 d6             	mov    rsi,rdx
    b776:	48 89 c7             	mov    rdi,rax
    b779:	e8 08 08 00 00       	call   bf86 <cljn_contains>
    b77e:	48 89 c7             	mov    rdi,rax
    b781:	e8 f9 24 00 00       	call   dc7f <cljn_truthy>
    b786:	85 c0                	test   eax,eax
    b788:	75 07                	jne    b791 <tn_all_in+0x76>
    b78a:	b8 00 00 00 00       	mov    eax,0x0
    b78f:	eb 17                	jmp    b7a8 <tn_all_in+0x8d>
    b791:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b795:	48 8b 40 28          	mov    rax,QWORD PTR [rax+0x28]
    b799:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    b79d:	48 89 d6             	mov    rsi,rdx
    b7a0:	48 89 c7             	mov    rdi,rax
    b7a3:	e8 73 ff ff ff       	call   b71b <tn_all_in>
    b7a8:	c9                   	leave
    b7a9:	c3                   	ret

000000000000b7aa <tn_map_subset>:
    b7aa:	f3 0f 1e fa          	endbr64
    b7ae:	55                   	push   rbp
    b7af:	48 89 e5             	mov    rbp,rsp
    b7b2:	48 83 ec 20          	sub    rsp,0x20
    b7b6:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    b7ba:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    b7be:	48 83 7d e8 02       	cmp    QWORD PTR [rbp-0x18],0x2
    b7c3:	75 0a                	jne    b7cf <tn_map_subset+0x25>
    b7c5:	b8 01 00 00 00       	mov    eax,0x1
    b7ca:	e9 9c 00 00 00       	jmp    b86b <tn_map_subset+0xc1>
    b7cf:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b7d3:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    b7d7:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b7db:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    b7df:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    b7e3:	48 89 d6             	mov    rsi,rdx
    b7e6:	48 89 c7             	mov    rdi,rax
    b7e9:	e8 bc ff ff ff       	call   b7aa <tn_map_subset>
    b7ee:	85 c0                	test   eax,eax
    b7f0:	75 07                	jne    b7f9 <tn_map_subset+0x4f>
    b7f2:	b8 00 00 00 00       	mov    eax,0x0
    b7f7:	eb 72                	jmp    b86b <tn_map_subset+0xc1>
    b7f9:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b7fd:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    b801:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    b805:	48 89 d6             	mov    rsi,rdx
    b808:	48 89 c7             	mov    rdi,rax
    b80b:	e8 58 e7 ff ff       	call   9f68 <cljn_map_contains>
    b810:	48 89 c7             	mov    rdi,rax
    b813:	e8 67 24 00 00       	call   dc7f <cljn_truthy>
    b818:	85 c0                	test   eax,eax
    b81a:	74 31                	je     b84d <tn_map_subset+0xa3>
    b81c:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b820:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    b824:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    b828:	48 89 d6             	mov    rsi,rdx
    b82b:	48 89 c7             	mov    rdi,rax
    b82e:	e8 41 e6 ff ff       	call   9e74 <cljn_map_get>
    b833:	48 89 c2             	mov    rdx,rax
    b836:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b83a:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    b83e:	48 89 d6             	mov    rsi,rdx
    b841:	48 89 c7             	mov    rdi,rax
    b844:	e8 bb 1d 00 00       	call   d604 <cljn_equal_raw>
    b849:	85 c0                	test   eax,eax
    b84b:	75 07                	jne    b854 <tn_map_subset+0xaa>
    b84d:	b8 00 00 00 00       	mov    eax,0x0
    b852:	eb 17                	jmp    b86b <tn_map_subset+0xc1>
    b854:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b858:	48 8b 40 28          	mov    rax,QWORD PTR [rax+0x28]
    b85c:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    b860:	48 89 d6             	mov    rsi,rdx
    b863:	48 89 c7             	mov    rdi,rax
    b866:	e8 3f ff ff ff       	call   b7aa <tn_map_subset>
    b86b:	c9                   	leave
    b86c:	c3                   	ret

000000000000b86d <tn_push_spread>:
    b86d:	f3 0f 1e fa          	endbr64
    b871:	55                   	push   rbp
    b872:	48 89 e5             	mov    rbp,rsp
    b875:	48 83 ec 30          	sub    rsp,0x30
    b879:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    b87d:	89 75 d4             	mov    DWORD PTR [rbp-0x2c],esi
    b880:	48 83 7d d8 02       	cmp    QWORD PTR [rbp-0x28],0x2
    b885:	75 07                	jne    b88e <tn_push_spread+0x21>
    b887:	b8 00 00 00 00       	mov    eax,0x0
    b88c:	eb 7e                	jmp    b90c <tn_push_spread+0x9f>
    b88e:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    b892:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    b896:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b89a:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    b89e:	8b 55 d4             	mov    edx,DWORD PTR [rbp-0x2c]
    b8a1:	89 d6                	mov    esi,edx
    b8a3:	48 89 c7             	mov    rdi,rax
    b8a6:	e8 c2 ff ff ff       	call   b86d <tn_push_spread>
    b8ab:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    b8af:	83 7d d4 00          	cmp    DWORD PTR [rbp-0x2c],0x0
    b8b3:	74 1d                	je     b8d2 <tn_push_spread+0x65>
    b8b5:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b8b9:	48 8b 50 18          	mov    rdx,QWORD PTR [rax+0x18]
    b8bd:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b8c1:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    b8c5:	48 89 d6             	mov    rsi,rdx
    b8c8:	48 89 c7             	mov    rdi,rax
    b8cb:	e8 b2 f4 ff ff       	call   ad82 <cljn_vec_pair>
    b8d0:	eb 08                	jmp    b8da <tn_push_spread+0x6d>
    b8d2:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b8d6:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    b8da:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    b8de:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b8e2:	48 89 c7             	mov    rdi,rax
    b8e5:	e8 7a b3 ff ff       	call   6c64 <cljn_gc_push>
    b8ea:	48 83 45 f0 01       	add    QWORD PTR [rbp-0x10],0x1
    b8ef:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b8f3:	48 8b 40 28          	mov    rax,QWORD PTR [rax+0x28]
    b8f7:	8b 55 d4             	mov    edx,DWORD PTR [rbp-0x2c]
    b8fa:	89 d6                	mov    esi,edx
    b8fc:	48 89 c7             	mov    rdi,rax
    b8ff:	e8 69 ff ff ff       	call   b86d <tn_push_spread>
    b904:	48 01 45 f0          	add    QWORD PTR [rbp-0x10],rax
    b908:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    b90c:	c9                   	leave
    b90d:	c3                   	ret

000000000000b90e <tn_reassoc_walk>:
    b90e:	f3 0f 1e fa          	endbr64
    b912:	55                   	push   rbp
    b913:	48 89 e5             	mov    rbp,rsp
    b916:	48 83 ec 20          	sub    rsp,0x20
    b91a:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    b91e:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    b922:	48 83 7d e8 02       	cmp    QWORD PTR [rbp-0x18],0x2
    b927:	0f 84 b2 00 00 00    	je     b9df <tn_reassoc_walk+0xd1>
    b92d:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    b931:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    b935:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    b939:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    b93d:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    b941:	48 89 d6             	mov    rsi,rdx
    b944:	48 89 c7             	mov    rdi,rax
    b947:	e8 c2 ff ff ff       	call   b90e <tn_reassoc_walk>
    b94c:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    b950:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    b954:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    b958:	48 89 d6             	mov    rsi,rdx
    b95b:	48 89 c7             	mov    rdi,rax
    b95e:	e8 a1 1c 00 00       	call   d604 <cljn_equal_raw>
    b963:	85 c0                	test   eax,eax
    b965:	75 5f                	jne    b9c6 <tn_reassoc_walk+0xb8>
    b967:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    b96b:	48 8b 50 18          	mov    rdx,QWORD PTR [rax+0x18]
    b96f:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    b973:	48 8b 48 10          	mov    rcx,QWORD PTR [rax+0x10]
    b977:	48 8b 05 02 87 00 02 	mov    rax,QWORD PTR [rip+0x2008702]        # 2014080 <gc_sp>
    b97e:	48 83 e8 01          	sub    rax,0x1
    b982:	48 8d 34 c5 00 00 00 	lea    rsi,[rax*8+0x0]
    b989:	00 
    b98a:	48 8d 05 ef 86 00 00 	lea    rax,[rip+0x86ef]        # 14080 <gc_stack>
    b991:	48 8b 04 06          	mov    rax,QWORD PTR [rsi+rax*1]
    b995:	48 89 ce             	mov    rsi,rcx
    b998:	48 89 c7             	mov    rdi,rax
    b99b:	e8 fd fa ff ff       	call   b49d <cljn_sorted_assoc>
    b9a0:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    b9a4:	48 8b 05 d5 86 00 02 	mov    rax,QWORD PTR [rip+0x20086d5]        # 2014080 <gc_sp>
    b9ab:	48 83 e8 01          	sub    rax,0x1
    b9af:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    b9b6:	00 
    b9b7:	48 8d 15 c2 86 00 00 	lea    rdx,[rip+0x86c2]        # 14080 <gc_stack>
    b9be:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    b9c2:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    b9c6:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    b9ca:	48 8b 40 28          	mov    rax,QWORD PTR [rax+0x28]
    b9ce:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    b9d2:	48 89 d6             	mov    rsi,rdx
    b9d5:	48 89 c7             	mov    rdi,rax
    b9d8:	e8 31 ff ff ff       	call   b90e <tn_reassoc_walk>
    b9dd:	eb 01                	jmp    b9e0 <tn_reassoc_walk+0xd2>
    b9df:	90                   	nop
    b9e0:	c9                   	leave
    b9e1:	c3                   	ret

000000000000b9e2 <cljn_sorted_dissoc>:
    b9e2:	f3 0f 1e fa          	endbr64
    b9e6:	55                   	push   rbp
    b9e7:	48 89 e5             	mov    rbp,rsp
    b9ea:	48 83 ec 30          	sub    rsp,0x30
    b9ee:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    b9f2:	48 89 75 d0          	mov    QWORD PTR [rbp-0x30],rsi
    b9f6:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    b9fa:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    b9fe:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    ba02:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    ba06:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    ba0a:	48 89 d6             	mov    rsi,rdx
    ba0d:	48 89 c7             	mov    rdi,rax
    ba10:	e8 bf f8 ff ff       	call   b2d4 <tn_get>
    ba15:	48 83 f8 2a          	cmp    rax,0x2a
    ba19:	75 06                	jne    ba21 <cljn_sorted_dissoc+0x3f>
    ba1b:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    ba1f:	eb 61                	jmp    ba82 <cljn_sorted_dissoc+0xa0>
    ba21:	e8 fb b4 ff ff       	call   6f21 <maybe_gc>
    ba26:	e8 4a fa ff ff       	call   b475 <cljn_sorted_map_empty>
    ba2b:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    ba2f:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    ba33:	48 89 c7             	mov    rdi,rax
    ba36:	e8 29 b2 ff ff       	call   6c64 <cljn_gc_push>
    ba3b:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    ba3f:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    ba43:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    ba47:	48 89 d6             	mov    rsi,rdx
    ba4a:	48 89 c7             	mov    rdi,rax
    ba4d:	e8 bc fe ff ff       	call   b90e <tn_reassoc_walk>
    ba52:	48 8b 05 27 86 00 02 	mov    rax,QWORD PTR [rip+0x2008627]        # 2014080 <gc_sp>
    ba59:	48 83 e8 01          	sub    rax,0x1
    ba5d:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    ba64:	00 
    ba65:	48 8d 05 14 86 00 00 	lea    rax,[rip+0x8614]        # 14080 <gc_stack>
    ba6c:	48 8b 04 02          	mov    rax,QWORD PTR [rdx+rax*1]
    ba70:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    ba74:	bf 01 00 00 00       	mov    edi,0x1
    ba79:	e8 5e b2 ff ff       	call   6cdc <cljn_gc_popn>
    ba7e:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    ba82:	c9                   	leave
    ba83:	c3                   	ret

000000000000ba84 <cljn_make_record>:
    ba84:	f3 0f 1e fa          	endbr64
    ba88:	55                   	push   rbp
    ba89:	48 89 e5             	mov    rbp,rsp
    ba8c:	48 83 ec 20          	sub    rsp,0x20
    ba90:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    ba94:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    ba98:	be 08 00 00 00       	mov    esi,0x8
    ba9d:	bf 20 00 00 00       	mov    edi,0x20
    baa2:	e8 cc b4 ff ff       	call   6f73 <obj_alloc>
    baa7:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    baab:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    baaf:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    bab3:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    bab7:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    babb:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    babf:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    bac3:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    bac7:	c9                   	leave
    bac8:	c3                   	ret

000000000000bac9 <cljn_record_type>:
    bac9:	f3 0f 1e fa          	endbr64
    bacd:	55                   	push   rbp
    bace:	48 89 e5             	mov    rbp,rsp
    bad1:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    bad5:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    bad9:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    badd:	5d                   	pop    rbp
    bade:	c3                   	ret

000000000000badf <cljn_record_map>:
    badf:	f3 0f 1e fa          	endbr64
    bae3:	55                   	push   rbp
    bae4:	48 89 e5             	mov    rbp,rsp
    bae7:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    baeb:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    baef:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    baf3:	5d                   	pop    rbp
    baf4:	c3                   	ret

000000000000baf5 <cljn_type_key>:
    baf5:	f3 0f 1e fa          	endbr64
    baf9:	55                   	push   rbp
    bafa:	48 89 e5             	mov    rbp,rsp
    bafd:	48 83 ec 08          	sub    rsp,0x8
    bb01:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    bb05:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    bb09:	83 e0 01             	and    eax,0x1
    bb0c:	48 85 c0             	test   rax,rax
    bb0f:	74 0a                	je     bb1b <cljn_type_key+0x26>
    bb11:	b8 d1 07 00 00       	mov    eax,0x7d1
    bb16:	e9 10 01 00 00       	jmp    bc2b <cljn_type_key+0x136>
    bb1b:	48 83 7d f8 02       	cmp    QWORD PTR [rbp-0x8],0x2
    bb20:	75 0a                	jne    bb2c <cljn_type_key+0x37>
    bb22:	b8 e5 07 00 00       	mov    eax,0x7e5
    bb27:	e9 ff 00 00 00       	jmp    bc2b <cljn_type_key+0x136>
    bb2c:	48 83 7d f8 0a       	cmp    QWORD PTR [rbp-0x8],0xa
    bb31:	74 07                	je     bb3a <cljn_type_key+0x45>
    bb33:	48 83 7d f8 06       	cmp    QWORD PTR [rbp-0x8],0x6
    bb38:	75 0a                	jne    bb44 <cljn_type_key+0x4f>
    bb3a:	b8 e7 07 00 00       	mov    eax,0x7e7
    bb3f:	e9 e7 00 00 00       	jmp    bc2b <cljn_type_key+0x136>
    bb44:	48 83 7d f8 12       	cmp    QWORD PTR [rbp-0x8],0x12
    bb49:	75 0a                	jne    bb55 <cljn_type_key+0x60>
    bb4b:	b8 d5 07 00 00       	mov    eax,0x7d5
    bb50:	e9 d6 00 00 00       	jmp    bc2b <cljn_type_key+0x136>
    bb55:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    bb59:	48 89 c7             	mov    rdi,rax
    bb5c:	e8 69 b2 ff ff       	call   6dca <obj_type>
    bb61:	83 f8 05             	cmp    eax,0x5
    bb64:	7f 79                	jg     bbdf <cljn_type_key+0xea>
    bb66:	85 c0                	test   eax,eax
    bb68:	7f 4d                	jg     bbb7 <cljn_type_key+0xc2>
    bb6a:	e9 b7 00 00 00       	jmp    bc26 <cljn_type_key+0x131>
    bb6f:	ba 01 00 00 00       	mov    edx,0x1
    bb74:	89 c1                	mov    ecx,eax
    bb76:	48 d3 e2             	shl    rdx,cl
    bb79:	48 89 d0             	mov    rax,rdx
    bb7c:	48 89 c2             	mov    rdx,rax
    bb7f:	81 e2 80 20 01 00    	and    edx,0x12080
    bb85:	48 85 d2             	test   rdx,rdx
    bb88:	0f 95 c2             	setne  dl
    bb8b:	84 d2                	test   dl,dl
    bb8d:	0f 85 82 00 00 00    	jne    bc15 <cljn_type_key+0x120>
    bb93:	48 89 c2             	mov    rdx,rax
    bb96:	81 e2 40 84 00 00    	and    edx,0x8440
    bb9c:	48 85 d2             	test   rdx,rdx
    bb9f:	0f 95 c2             	setne  dl
    bba2:	84 d2                	test   dl,dl
    bba4:	75 68                	jne    bc0e <cljn_type_key+0x119>
    bba6:	25 00 01 00 00       	and    eax,0x100
    bbab:	48 85 c0             	test   rax,rax
    bbae:	0f 95 c0             	setne  al
    bbb1:	84 c0                	test   al,al
    bbb3:	75 67                	jne    bc1c <cljn_type_key+0x127>
    bbb5:	eb 6f                	jmp    bc26 <cljn_type_key+0x131>
    bbb7:	83 f8 05             	cmp    eax,0x5
    bbba:	77 6a                	ja     bc26 <cljn_type_key+0x131>
    bbbc:	89 c0                	mov    eax,eax
    bbbe:	48 8d 14 85 00 00 00 	lea    rdx,[rax*4+0x0]
    bbc5:	00 
    bbc6:	48 8d 05 67 45 00 00 	lea    rax,[rip+0x4567]        # 10134 <_IO_stdin_used+0x134>
    bbcd:	8b 04 02             	mov    eax,DWORD PTR [rdx+rax*1]
    bbd0:	48 98                	cdqe
    bbd2:	48 8d 15 5b 45 00 00 	lea    rdx,[rip+0x455b]        # 10134 <_IO_stdin_used+0x134>
    bbd9:	48 01 d0             	add    rax,rdx
    bbdc:	3e ff e0             	notrack jmp rax
    bbdf:	83 f8 10             	cmp    eax,0x10
    bbe2:	7f 42                	jg     bc26 <cljn_type_key+0x131>
    bbe4:	83 f8 06             	cmp    eax,0x6
    bbe7:	7d 86                	jge    bb6f <cljn_type_key+0x7a>
    bbe9:	eb 3b                	jmp    bc26 <cljn_type_key+0x131>
    bbeb:	b8 d3 07 00 00       	mov    eax,0x7d3
    bbf0:	eb 39                	jmp    bc2b <cljn_type_key+0x136>
    bbf2:	b8 d5 07 00 00       	mov    eax,0x7d5
    bbf7:	eb 32                	jmp    bc2b <cljn_type_key+0x136>
    bbf9:	b8 d7 07 00 00       	mov    eax,0x7d7
    bbfe:	eb 2b                	jmp    bc2b <cljn_type_key+0x136>
    bc00:	b8 d9 07 00 00       	mov    eax,0x7d9
    bc05:	eb 24                	jmp    bc2b <cljn_type_key+0x136>
    bc07:	b8 db 07 00 00       	mov    eax,0x7db
    bc0c:	eb 1d                	jmp    bc2b <cljn_type_key+0x136>
    bc0e:	b8 dd 07 00 00       	mov    eax,0x7dd
    bc13:	eb 16                	jmp    bc2b <cljn_type_key+0x136>
    bc15:	b8 df 07 00 00       	mov    eax,0x7df
    bc1a:	eb 0f                	jmp    bc2b <cljn_type_key+0x136>
    bc1c:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    bc20:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    bc24:	eb 05                	jmp    bc2b <cljn_type_key+0x136>
    bc26:	b8 97 08 00 00       	mov    eax,0x897
    bc2b:	c9                   	leave
    bc2c:	c3                   	ret

000000000000bc2d <cljn_register_method>:
    bc2d:	f3 0f 1e fa          	endbr64
    bc31:	55                   	push   rbp
    bc32:	48 89 e5             	mov    rbp,rsp
    bc35:	48 83 ec 30          	sub    rsp,0x30
    bc39:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    bc3d:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    bc41:	48 89 55 d8          	mov    QWORD PTR [rbp-0x28],rdx
    bc45:	bf 20 00 00 00       	mov    edi,0x20
    bc4a:	e8 e5 b0 ff ff       	call   6d34 <xalloc>
    bc4f:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    bc53:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    bc57:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    bc5b:	48 89 10             	mov    QWORD PTR [rax],rdx
    bc5e:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    bc62:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    bc66:	48 89 50 08          	mov    QWORD PTR [rax+0x8],rdx
    bc6a:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    bc6e:	48 8b 55 d8          	mov    rdx,QWORD PTR [rbp-0x28]
    bc72:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    bc76:	48 8b 15 3b 85 00 02 	mov    rdx,QWORD PTR [rip+0x200853b]        # 20141b8 <method_table>
    bc7d:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    bc81:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    bc85:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    bc89:	48 89 05 28 85 00 02 	mov    QWORD PTR [rip+0x2008528],rax        # 20141b8 <method_table>
    bc90:	90                   	nop
    bc91:	c9                   	leave
    bc92:	c3                   	ret

000000000000bc93 <cljn_lookup_method>:
    bc93:	f3 0f 1e fa          	endbr64
    bc97:	55                   	push   rbp
    bc98:	48 89 e5             	mov    rbp,rsp
    bc9b:	48 83 ec 20          	sub    rsp,0x20
    bc9f:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    bca3:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    bca7:	48 8b 05 0a 85 00 02 	mov    rax,QWORD PTR [rip+0x200850a]        # 20141b8 <method_table>
    bcae:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    bcb2:	eb 3e                	jmp    bcf2 <cljn_lookup_method+0x5f>
    bcb4:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    bcb8:	48 8b 00             	mov    rax,QWORD PTR [rax]
    bcbb:	48 39 45 e8          	cmp    QWORD PTR [rbp-0x18],rax
    bcbf:	75 25                	jne    bce6 <cljn_lookup_method+0x53>
    bcc1:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    bcc5:	48 8b 40 08          	mov    rax,QWORD PTR [rax+0x8]
    bcc9:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    bccd:	48 89 d6             	mov    rsi,rdx
    bcd0:	48 89 c7             	mov    rdi,rax
    bcd3:	e8 2c 19 00 00       	call   d604 <cljn_equal_raw>
    bcd8:	85 c0                	test   eax,eax
    bcda:	74 0a                	je     bce6 <cljn_lookup_method+0x53>
    bcdc:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    bce0:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    bce4:	eb 18                	jmp    bcfe <cljn_lookup_method+0x6b>
    bce6:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    bcea:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    bcee:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    bcf2:	48 83 7d f8 00       	cmp    QWORD PTR [rbp-0x8],0x0
    bcf7:	75 bb                	jne    bcb4 <cljn_lookup_method+0x21>
    bcf9:	b8 02 00 00 00       	mov    eax,0x2
    bcfe:	c9                   	leave
    bcff:	c3                   	ret

000000000000bd00 <cljn_no_method>:
    bd00:	f3 0f 1e fa          	endbr64
    bd04:	55                   	push   rbp
    bd05:	48 89 e5             	mov    rbp,rsp
    bd08:	48 83 ec 10          	sub    rsp,0x10
    bd0c:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    bd10:	48 8b 05 49 83 00 00 	mov    rax,QWORD PTR [rip+0x8349]        # 14060 <stderr@GLIBC_2.2.5>
    bd17:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    bd1b:	48 8d 0d 2e 44 00 00 	lea    rcx,[rip+0x442e]        # 10150 <_IO_stdin_used+0x150>
    bd22:	48 89 ce             	mov    rsi,rcx
    bd25:	48 89 c7             	mov    rdi,rax
    bd28:	b8 00 00 00 00       	mov    eax,0x0
    bd2d:	e8 7e 53 ff ff       	call   10b0 <fprintf@plt>
    bd32:	bf 01 00 00 00       	mov    edi,0x1
    bd37:	e8 c4 53 ff ff       	call   1100 <exit@plt>

000000000000bd3c <gc_mark_method_table>:
    bd3c:	f3 0f 1e fa          	endbr64
    bd40:	55                   	push   rbp
    bd41:	48 89 e5             	mov    rbp,rsp
    bd44:	48 83 ec 10          	sub    rsp,0x10
    bd48:	48 8b 05 69 84 00 02 	mov    rax,QWORD PTR [rip+0x2008469]        # 20141b8 <method_table>
    bd4f:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    bd53:	eb 2c                	jmp    bd81 <gc_mark_method_table+0x45>
    bd55:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    bd59:	48 8b 40 08          	mov    rax,QWORD PTR [rax+0x8]
    bd5d:	48 89 c7             	mov    rdi,rax
    bd60:	e8 5d b3 ff ff       	call   70c2 <gc_mark>
    bd65:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    bd69:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    bd6d:	48 89 c7             	mov    rdi,rax
    bd70:	e8 4d b3 ff ff       	call   70c2 <gc_mark>
    bd75:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    bd79:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    bd7d:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    bd81:	48 83 7d f8 00       	cmp    QWORD PTR [rbp-0x8],0x0
    bd86:	75 cd                	jne    bd55 <gc_mark_method_table+0x19>
    bd88:	90                   	nop
    bd89:	90                   	nop
    bd8a:	c9                   	leave
    bd8b:	c3                   	ret

000000000000bd8c <cljn_get>:
    bd8c:	f3 0f 1e fa          	endbr64
    bd90:	55                   	push   rbp
    bd91:	48 89 e5             	mov    rbp,rsp
    bd94:	48 83 ec 30          	sub    rsp,0x30
    bd98:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    bd9c:	48 89 75 d0          	mov    QWORD PTR [rbp-0x30],rsi
    bda0:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    bda4:	48 89 c7             	mov    rdi,rax
    bda7:	e8 1e b0 ff ff       	call   6dca <obj_type>
    bdac:	83 e8 05             	sub    eax,0x5
    bdaf:	83 f8 0d             	cmp    eax,0xd
    bdb2:	0f 87 c7 01 00 00    	ja     bf7f <cljn_get+0x1f3>
    bdb8:	89 c0                	mov    eax,eax
    bdba:	48 8d 14 85 00 00 00 	lea    rdx,[rax*4+0x0]
    bdc1:	00 
    bdc2:	48 8d 05 c7 43 00 00 	lea    rax,[rip+0x43c7]        # 10190 <_IO_stdin_used+0x190>
    bdc9:	8b 04 02             	mov    eax,DWORD PTR [rdx+rax*1]
    bdcc:	48 98                	cdqe
    bdce:	48 8d 15 bb 43 00 00 	lea    rdx,[rip+0x43bb]        # 10190 <_IO_stdin_used+0x190>
    bdd5:	48 01 d0             	add    rax,rdx
    bdd8:	3e ff e0             	notrack jmp rax
    bddb:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    bddf:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    bde3:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    bde7:	48 89 d6             	mov    rsi,rdx
    bdea:	48 89 c7             	mov    rdi,rax
    bded:	e8 82 e0 ff ff       	call   9e74 <cljn_map_get>
    bdf2:	e9 8d 01 00 00       	jmp    bf84 <cljn_get+0x1f8>
    bdf7:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    bdfb:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    bdff:	48 89 d6             	mov    rsi,rdx
    be02:	48 89 c7             	mov    rdi,rax
    be05:	e8 6a e0 ff ff       	call   9e74 <cljn_map_get>
    be0a:	e9 75 01 00 00       	jmp    bf84 <cljn_get+0x1f8>
    be0f:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    be13:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    be17:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    be1b:	83 e0 01             	and    eax,0x1
    be1e:	48 85 c0             	test   rax,rax
    be21:	74 38                	je     be5b <cljn_get+0xcf>
    be23:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    be27:	48 d1 f8             	sar    rax,1
    be2a:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    be2e:	48 83 7d f8 00       	cmp    QWORD PTR [rbp-0x8],0x0
    be33:	78 26                	js     be5b <cljn_get+0xcf>
    be35:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    be39:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    be3d:	48 39 45 f8          	cmp    QWORD PTR [rbp-0x8],rax
    be41:	7d 18                	jge    be5b <cljn_get+0xcf>
    be43:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    be47:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    be4b:	48 89 d6             	mov    rsi,rdx
    be4e:	48 89 c7             	mov    rdi,rax
    be51:	e8 89 c1 ff ff       	call   7fdf <pv_nth>
    be56:	e9 29 01 00 00       	jmp    bf84 <cljn_get+0x1f8>
    be5b:	b8 02 00 00 00       	mov    eax,0x2
    be60:	e9 1f 01 00 00       	jmp    bf84 <cljn_get+0x1f8>
    be65:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    be69:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    be6d:	48 89 d6             	mov    rsi,rdx
    be70:	48 89 c7             	mov    rdi,rax
    be73:	e8 fc c7 ff ff       	call   8674 <set_member>
    be78:	85 c0                	test   eax,eax
    be7a:	74 09                	je     be85 <cljn_get+0xf9>
    be7c:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    be80:	e9 ff 00 00 00       	jmp    bf84 <cljn_get+0x1f8>
    be85:	b8 02 00 00 00       	mov    eax,0x2
    be8a:	e9 f5 00 00 00       	jmp    bf84 <cljn_get+0x1f8>
    be8f:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    be93:	48 89 c7             	mov    rdi,rax
    be96:	e8 c9 cd ff ff       	call   8c64 <cljn_hash>
    be9b:	89 c6                	mov    esi,eax
    be9d:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    bea1:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    bea5:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    bea9:	48 89 d1             	mov    rcx,rdx
    beac:	89 f2                	mov    edx,esi
    beae:	be 00 00 00 00       	mov    esi,0x0
    beb3:	48 89 c7             	mov    rdi,rax
    beb6:	e8 f8 ce ff ff       	call   8db3 <node_get>
    bebb:	48 83 f8 2a          	cmp    rax,0x2a
    bebf:	74 09                	je     beca <cljn_get+0x13e>
    bec1:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    bec5:	e9 ba 00 00 00       	jmp    bf84 <cljn_get+0x1f8>
    beca:	b8 02 00 00 00       	mov    eax,0x2
    becf:	e9 b0 00 00 00       	jmp    bf84 <cljn_get+0x1f8>
    bed4:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    bed8:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    bedc:	48 89 d6             	mov    rsi,rdx
    bedf:	48 89 c7             	mov    rdi,rax
    bee2:	e8 c9 f6 ff ff       	call   b5b0 <cljn_sorted_get>
    bee7:	e9 98 00 00 00       	jmp    bf84 <cljn_get+0x1f8>
    beec:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    bef0:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    bef4:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    bef8:	48 89 d6             	mov    rsi,rdx
    befb:	48 89 c7             	mov    rdi,rax
    befe:	e8 d1 f3 ff ff       	call   b2d4 <tn_get>
    bf03:	48 83 f8 2a          	cmp    rax,0x2a
    bf07:	74 06                	je     bf0f <cljn_get+0x183>
    bf09:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    bf0d:	eb 75                	jmp    bf84 <cljn_get+0x1f8>
    bf0f:	b8 02 00 00 00       	mov    eax,0x2
    bf14:	eb 6e                	jmp    bf84 <cljn_get+0x1f8>
    bf16:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    bf1a:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    bf1e:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    bf22:	83 e0 01             	and    eax,0x1
    bf25:	48 85 c0             	test   rax,rax
    bf28:	74 35                	je     bf5f <cljn_get+0x1d3>
    bf2a:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    bf2e:	48 d1 f8             	sar    rax,1
    bf31:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    bf35:	48 83 7d e8 00       	cmp    QWORD PTR [rbp-0x18],0x0
    bf3a:	78 23                	js     bf5f <cljn_get+0x1d3>
    bf3c:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    bf40:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    bf44:	48 39 45 e8          	cmp    QWORD PTR [rbp-0x18],rax
    bf48:	7d 15                	jge    bf5f <cljn_get+0x1d3>
    bf4a:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    bf4e:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    bf52:	48 89 d6             	mov    rsi,rdx
    bf55:	48 89 c7             	mov    rdi,rax
    bf58:	e8 82 c0 ff ff       	call   7fdf <pv_nth>
    bf5d:	eb 25                	jmp    bf84 <cljn_get+0x1f8>
    bf5f:	b8 02 00 00 00       	mov    eax,0x2
    bf64:	eb 1e                	jmp    bf84 <cljn_get+0x1f8>
    bf66:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    bf6a:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    bf6e:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    bf72:	48 89 d6             	mov    rsi,rdx
    bf75:	48 89 c7             	mov    rdi,rax
    bf78:	e8 0f fe ff ff       	call   bd8c <cljn_get>
    bf7d:	eb 05                	jmp    bf84 <cljn_get+0x1f8>
    bf7f:	b8 02 00 00 00       	mov    eax,0x2
    bf84:	c9                   	leave
    bf85:	c3                   	ret

000000000000bf86 <cljn_contains>:
    bf86:	f3 0f 1e fa          	endbr64
    bf8a:	55                   	push   rbp
    bf8b:	48 89 e5             	mov    rbp,rsp
    bf8e:	48 83 ec 20          	sub    rsp,0x20
    bf92:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    bf96:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    bf9a:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    bf9e:	48 89 c7             	mov    rdi,rax
    bfa1:	e8 24 ae ff ff       	call   6dca <obj_type>
    bfa6:	83 e8 05             	sub    eax,0x5
    bfa9:	83 f8 0d             	cmp    eax,0xd
    bfac:	0f 87 38 01 00 00    	ja     c0ea <cljn_contains+0x164>
    bfb2:	89 c0                	mov    eax,eax
    bfb4:	48 8d 14 85 00 00 00 	lea    rdx,[rax*4+0x0]
    bfbb:	00 
    bfbc:	48 8d 05 05 42 00 00 	lea    rax,[rip+0x4205]        # 101c8 <_IO_stdin_used+0x1c8>
    bfc3:	8b 04 02             	mov    eax,DWORD PTR [rdx+rax*1]
    bfc6:	48 98                	cdqe
    bfc8:	48 8d 15 f9 41 00 00 	lea    rdx,[rip+0x41f9]        # 101c8 <_IO_stdin_used+0x1c8>
    bfcf:	48 01 d0             	add    rax,rdx
    bfd2:	3e ff e0             	notrack jmp rax
    bfd5:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    bfd9:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    bfdd:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    bfe1:	48 89 d6             	mov    rsi,rdx
    bfe4:	48 89 c7             	mov    rdi,rax
    bfe7:	e8 7c df ff ff       	call   9f68 <cljn_map_contains>
    bfec:	e9 fe 00 00 00       	jmp    c0ef <cljn_contains+0x169>
    bff1:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    bff5:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    bff9:	48 89 d6             	mov    rsi,rdx
    bffc:	48 89 c7             	mov    rdi,rax
    bfff:	e8 64 df ff ff       	call   9f68 <cljn_map_contains>
    c004:	e9 e6 00 00 00       	jmp    c0ef <cljn_contains+0x169>
    c009:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    c00d:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c011:	48 89 d6             	mov    rsi,rdx
    c014:	48 89 c7             	mov    rdi,rax
    c017:	e8 73 cb ff ff       	call   8b8f <cljn_set_contains>
    c01c:	e9 ce 00 00 00       	jmp    c0ef <cljn_contains+0x169>
    c021:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    c025:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c029:	48 89 d6             	mov    rsi,rdx
    c02c:	48 89 c7             	mov    rdi,rax
    c02f:	e8 bf f5 ff ff       	call   b5f3 <cljn_sorted_contains>
    c034:	e9 b6 00 00 00       	jmp    c0ef <cljn_contains+0x169>
    c039:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c03d:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    c041:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    c045:	83 e0 01             	and    eax,0x1
    c048:	48 85 c0             	test   rax,rax
    c04b:	74 2a                	je     c077 <cljn_contains+0xf1>
    c04d:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    c051:	48 d1 f8             	sar    rax,1
    c054:	48 85 c0             	test   rax,rax
    c057:	78 1e                	js     c077 <cljn_contains+0xf1>
    c059:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    c05d:	48 d1 f8             	sar    rax,1
    c060:	48 89 c2             	mov    rdx,rax
    c063:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    c067:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    c06b:	48 39 c2             	cmp    rdx,rax
    c06e:	7d 07                	jge    c077 <cljn_contains+0xf1>
    c070:	b8 01 00 00 00       	mov    eax,0x1
    c075:	eb 05                	jmp    c07c <cljn_contains+0xf6>
    c077:	b8 00 00 00 00       	mov    eax,0x0
    c07c:	89 c7                	mov    edi,eax
    c07e:	e8 15 13 00 00       	call   d398 <b2v>
    c083:	eb 6a                	jmp    c0ef <cljn_contains+0x169>
    c085:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c089:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    c08d:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    c091:	48 89 d6             	mov    rsi,rdx
    c094:	48 89 c7             	mov    rdi,rax
    c097:	e8 ea fe ff ff       	call   bf86 <cljn_contains>
    c09c:	eb 51                	jmp    c0ef <cljn_contains+0x169>
    c09e:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c0a2:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    c0a6:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    c0aa:	83 e0 01             	and    eax,0x1
    c0ad:	48 85 c0             	test   rax,rax
    c0b0:	74 2a                	je     c0dc <cljn_contains+0x156>
    c0b2:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    c0b6:	48 d1 f8             	sar    rax,1
    c0b9:	48 85 c0             	test   rax,rax
    c0bc:	78 1e                	js     c0dc <cljn_contains+0x156>
    c0be:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    c0c2:	48 d1 f8             	sar    rax,1
    c0c5:	48 89 c2             	mov    rdx,rax
    c0c8:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    c0cc:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    c0d0:	48 39 c2             	cmp    rdx,rax
    c0d3:	7d 07                	jge    c0dc <cljn_contains+0x156>
    c0d5:	b8 01 00 00 00       	mov    eax,0x1
    c0da:	eb 05                	jmp    c0e1 <cljn_contains+0x15b>
    c0dc:	b8 00 00 00 00       	mov    eax,0x0
    c0e1:	89 c7                	mov    edi,eax
    c0e3:	e8 b0 12 00 00       	call   d398 <b2v>
    c0e8:	eb 05                	jmp    c0ef <cljn_contains+0x169>
    c0ea:	b8 06 00 00 00       	mov    eax,0x6
    c0ef:	c9                   	leave
    c0f0:	c3                   	ret

000000000000c0f1 <cljn_conj>:
    c0f1:	f3 0f 1e fa          	endbr64
    c0f5:	55                   	push   rbp
    c0f6:	48 89 e5             	mov    rbp,rsp
    c0f9:	53                   	push   rbx
    c0fa:	48 83 ec 18          	sub    rsp,0x18
    c0fe:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    c102:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    c106:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c10a:	48 89 c7             	mov    rdi,rax
    c10d:	e8 b8 ac ff ff       	call   6dca <obj_type>
    c112:	83 f8 10             	cmp    eax,0x10
    c115:	0f 87 69 01 00 00    	ja     c284 <cljn_conj+0x193>
    c11b:	89 c0                	mov    eax,eax
    c11d:	48 8d 14 85 00 00 00 	lea    rdx,[rax*4+0x0]
    c124:	00 
    c125:	48 8d 05 34 41 00 00 	lea    rax,[rip+0x4134]        # 10260 <_IO_stdin_used+0x260>
    c12c:	8b 04 02             	mov    eax,DWORD PTR [rdx+rax*1]
    c12f:	48 98                	cdqe
    c131:	48 8d 15 28 41 00 00 	lea    rdx,[rip+0x4128]        # 10260 <_IO_stdin_used+0x260>
    c138:	48 01 d0             	add    rax,rdx
    c13b:	3e ff e0             	notrack jmp rax
    c13e:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    c142:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c146:	48 89 d6             	mov    rsi,rdx
    c149:	48 89 c7             	mov    rdi,rax
    c14c:	e8 6b c0 ff ff       	call   81bc <cljn_vec_conj>
    c151:	e9 62 01 00 00       	jmp    c2b8 <cljn_conj+0x1c7>
    c156:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    c15a:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c15e:	48 89 d6             	mov    rsi,rdx
    c161:	48 89 c7             	mov    rdi,rax
    c164:	e8 5c c6 ff ff       	call   87c5 <cljn_set_conj>
    c169:	e9 4a 01 00 00       	jmp    c2b8 <cljn_conj+0x1c7>
    c16e:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    c172:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c176:	48 89 d6             	mov    rsi,rdx
    c179:	48 89 c7             	mov    rdi,rax
    c17c:	e8 02 f4 ff ff       	call   b583 <cljn_sorted_set_conj>
    c181:	e9 32 01 00 00       	jmp    c2b8 <cljn_conj+0x1c7>
    c186:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    c18a:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    c18e:	48 89 d6             	mov    rsi,rdx
    c191:	48 89 c7             	mov    rdi,rax
    c194:	e8 48 b5 ff ff       	call   76e1 <cljn_cons>
    c199:	e9 1a 01 00 00       	jmp    c2b8 <cljn_conj+0x1c7>
    c19e:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    c1a2:	48 89 c7             	mov    rdi,rax
    c1a5:	e8 20 ac ff ff       	call   6dca <obj_type>
    c1aa:	83 f8 05             	cmp    eax,0x5
    c1ad:	75 4d                	jne    c1fc <cljn_conj+0x10b>
    c1af:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    c1b3:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    c1b7:	48 83 f8 02          	cmp    rax,0x2
    c1bb:	75 3f                	jne    c1fc <cljn_conj+0x10b>
    c1bd:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    c1c1:	be 01 00 00 00       	mov    esi,0x1
    c1c6:	48 89 c7             	mov    rdi,rax
    c1c9:	e8 11 be ff ff       	call   7fdf <pv_nth>
    c1ce:	48 89 c3             	mov    rbx,rax
    c1d1:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    c1d5:	be 00 00 00 00       	mov    esi,0x0
    c1da:	48 89 c7             	mov    rdi,rax
    c1dd:	e8 fd bd ff ff       	call   7fdf <pv_nth>
    c1e2:	48 89 c1             	mov    rcx,rax
    c1e5:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c1e9:	48 89 da             	mov    rdx,rbx
    c1ec:	48 89 ce             	mov    rsi,rcx
    c1ef:	48 89 c7             	mov    rdi,rax
    c1f2:	e8 39 de ff ff       	call   a030 <cljn_map_assoc>
    c1f7:	e9 bc 00 00 00       	jmp    c2b8 <cljn_conj+0x1c7>
    c1fc:	48 8d 05 fd 3f 00 00 	lea    rax,[rip+0x3ffd]        # 10200 <_IO_stdin_used+0x200>
    c203:	48 89 c7             	mov    rdi,rax
    c206:	e8 83 ab ff ff       	call   6d8e <die>
    c20b:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c20f:	e9 a4 00 00 00       	jmp    c2b8 <cljn_conj+0x1c7>
    c214:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    c218:	48 89 c7             	mov    rdi,rax
    c21b:	e8 aa ab ff ff       	call   6dca <obj_type>
    c220:	83 f8 05             	cmp    eax,0x5
    c223:	75 4a                	jne    c26f <cljn_conj+0x17e>
    c225:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    c229:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    c22d:	48 83 f8 02          	cmp    rax,0x2
    c231:	75 3c                	jne    c26f <cljn_conj+0x17e>
    c233:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    c237:	be 01 00 00 00       	mov    esi,0x1
    c23c:	48 89 c7             	mov    rdi,rax
    c23f:	e8 9b bd ff ff       	call   7fdf <pv_nth>
    c244:	48 89 c3             	mov    rbx,rax
    c247:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    c24b:	be 00 00 00 00       	mov    esi,0x0
    c250:	48 89 c7             	mov    rdi,rax
    c253:	e8 87 bd ff ff       	call   7fdf <pv_nth>
    c258:	48 89 c1             	mov    rcx,rax
    c25b:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c25f:	48 89 da             	mov    rdx,rbx
    c262:	48 89 ce             	mov    rsi,rcx
    c265:	48 89 c7             	mov    rdi,rax
    c268:	e8 30 f2 ff ff       	call   b49d <cljn_sorted_assoc>
    c26d:	eb 49                	jmp    c2b8 <cljn_conj+0x1c7>
    c26f:	48 8d 05 aa 3f 00 00 	lea    rax,[rip+0x3faa]        # 10220 <_IO_stdin_used+0x220>
    c276:	48 89 c7             	mov    rdi,rax
    c279:	e8 10 ab ff ff       	call   6d8e <die>
    c27e:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c282:	eb 34                	jmp    c2b8 <cljn_conj+0x1c7>
    c284:	48 83 7d e8 12       	cmp    QWORD PTR [rbp-0x18],0x12
    c289:	74 07                	je     c292 <cljn_conj+0x1a1>
    c28b:	48 83 7d e8 02       	cmp    QWORD PTR [rbp-0x18],0x2
    c290:	75 13                	jne    c2a5 <cljn_conj+0x1b4>
    c292:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    c296:	be 12 00 00 00       	mov    esi,0x12
    c29b:	48 89 c7             	mov    rdi,rax
    c29e:	e8 3e b4 ff ff       	call   76e1 <cljn_cons>
    c2a3:	eb 13                	jmp    c2b8 <cljn_conj+0x1c7>
    c2a5:	48 8d 05 94 3f 00 00 	lea    rax,[rip+0x3f94]        # 10240 <_IO_stdin_used+0x240>
    c2ac:	48 89 c7             	mov    rdi,rax
    c2af:	e8 da aa ff ff       	call   6d8e <die>
    c2b4:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c2b8:	48 8b 5d f8          	mov    rbx,QWORD PTR [rbp-0x8]
    c2bc:	c9                   	leave
    c2bd:	c3                   	ret

000000000000c2be <cljn_assoc>:
    c2be:	f3 0f 1e fa          	endbr64
    c2c2:	55                   	push   rbp
    c2c3:	48 89 e5             	mov    rbp,rsp
    c2c6:	48 83 ec 50          	sub    rsp,0x50
    c2ca:	48 89 7d c8          	mov    QWORD PTR [rbp-0x38],rdi
    c2ce:	48 89 75 c0          	mov    QWORD PTR [rbp-0x40],rsi
    c2d2:	48 89 55 b8          	mov    QWORD PTR [rbp-0x48],rdx
    c2d6:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    c2da:	48 89 c7             	mov    rdi,rax
    c2dd:	e8 e8 aa ff ff       	call   6dca <obj_type>
    c2e2:	83 e8 05             	sub    eax,0x5
    c2e5:	83 f8 0a             	cmp    eax,0xa
    c2e8:	0f 87 d8 00 00 00    	ja     c3c6 <cljn_assoc+0x108>
    c2ee:	89 c0                	mov    eax,eax
    c2f0:	48 8d 14 85 00 00 00 	lea    rdx,[rax*4+0x0]
    c2f7:	00 
    c2f8:	48 8d 05 c1 3f 00 00 	lea    rax,[rip+0x3fc1]        # 102c0 <_IO_stdin_used+0x2c0>
    c2ff:	8b 04 02             	mov    eax,DWORD PTR [rdx+rax*1]
    c302:	48 98                	cdqe
    c304:	48 8d 15 b5 3f 00 00 	lea    rdx,[rip+0x3fb5]        # 102c0 <_IO_stdin_used+0x2c0>
    c30b:	48 01 d0             	add    rax,rdx
    c30e:	3e ff e0             	notrack jmp rax
    c311:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    c315:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    c319:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c31d:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    c321:	48 8b 55 b8          	mov    rdx,QWORD PTR [rbp-0x48]
    c325:	48 8b 4d c0          	mov    rcx,QWORD PTR [rbp-0x40]
    c329:	48 89 ce             	mov    rsi,rcx
    c32c:	48 89 c7             	mov    rdi,rax
    c32f:	e8 fc dc ff ff       	call   a030 <cljn_map_assoc>
    c334:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    c338:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    c33c:	48 89 c7             	mov    rdi,rax
    c33f:	e8 20 a9 ff ff       	call   6c64 <cljn_gc_push>
    c344:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    c348:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    c34c:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    c350:	48 89 d6             	mov    rsi,rdx
    c353:	48 89 c7             	mov    rdi,rax
    c356:	e8 29 f7 ff ff       	call   ba84 <cljn_make_record>
    c35b:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    c35f:	bf 01 00 00 00       	mov    edi,0x1
    c364:	e8 73 a9 ff ff       	call   6cdc <cljn_gc_popn>
    c369:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c36d:	e9 d5 00 00 00       	jmp    c447 <cljn_assoc+0x189>
    c372:	48 8b 55 b8          	mov    rdx,QWORD PTR [rbp-0x48]
    c376:	48 8b 4d c0          	mov    rcx,QWORD PTR [rbp-0x40]
    c37a:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    c37e:	48 89 ce             	mov    rsi,rcx
    c381:	48 89 c7             	mov    rdi,rax
    c384:	e8 a7 dc ff ff       	call   a030 <cljn_map_assoc>
    c389:	e9 b9 00 00 00       	jmp    c447 <cljn_assoc+0x189>
    c38e:	48 8b 55 b8          	mov    rdx,QWORD PTR [rbp-0x48]
    c392:	48 8b 4d c0          	mov    rcx,QWORD PTR [rbp-0x40]
    c396:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    c39a:	48 89 ce             	mov    rsi,rcx
    c39d:	48 89 c7             	mov    rdi,rax
    c3a0:	e8 f8 f0 ff ff       	call   b49d <cljn_sorted_assoc>
    c3a5:	e9 9d 00 00 00       	jmp    c447 <cljn_assoc+0x189>
    c3aa:	48 8b 55 b8          	mov    rdx,QWORD PTR [rbp-0x48]
    c3ae:	48 8b 4d c0          	mov    rcx,QWORD PTR [rbp-0x40]
    c3b2:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    c3b6:	48 89 ce             	mov    rsi,rcx
    c3b9:	48 89 c7             	mov    rdi,rax
    c3bc:	e8 8c c0 ff ff       	call   844d <cljn_vec_assoc>
    c3c1:	e9 81 00 00 00       	jmp    c447 <cljn_assoc+0x189>
    c3c6:	48 83 7d c8 02       	cmp    QWORD PTR [rbp-0x38],0x2
    c3cb:	75 27                	jne    c3f4 <cljn_assoc+0x136>
    c3cd:	bf 00 00 00 00       	mov    edi,0x0
    c3d2:	e8 a0 d6 ff ff       	call   9a77 <cljn_map_alloc>
    c3d7:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    c3db:	48 8b 55 b8          	mov    rdx,QWORD PTR [rbp-0x48]
    c3df:	48 8b 4d c0          	mov    rcx,QWORD PTR [rbp-0x40]
    c3e3:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    c3e7:	48 89 ce             	mov    rsi,rcx
    c3ea:	48 89 c7             	mov    rdi,rax
    c3ed:	e8 3e dc ff ff       	call   a030 <cljn_map_assoc>
    c3f2:	eb 53                	jmp    c447 <cljn_assoc+0x189>
    c3f4:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    c3f8:	48 89 c7             	mov    rdi,rax
    c3fb:	e8 f5 f6 ff ff       	call   baf5 <cljn_type_key>
    c400:	48 89 c6             	mov    rsi,rax
    c403:	48 c7 c7 ff ff ff ff 	mov    rdi,0xffffffffffffffff
    c40a:	e8 84 f8 ff ff       	call   bc93 <cljn_lookup_method>
    c40f:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    c413:	48 83 7d f0 02       	cmp    QWORD PTR [rbp-0x10],0x2
    c418:	74 1a                	je     c434 <cljn_assoc+0x176>
    c41a:	48 8b 4d b8          	mov    rcx,QWORD PTR [rbp-0x48]
    c41e:	48 8b 55 c0          	mov    rdx,QWORD PTR [rbp-0x40]
    c422:	48 8b 75 c8          	mov    rsi,QWORD PTR [rbp-0x38]
    c426:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    c42a:	48 89 c7             	mov    rdi,rax
    c42d:	e8 b5 2f 00 00       	call   f3e7 <call_fn3>
    c432:	eb 13                	jmp    c447 <cljn_assoc+0x189>
    c434:	48 8d 05 69 3e 00 00 	lea    rax,[rip+0x3e69]        # 102a4 <_IO_stdin_used+0x2a4>
    c43b:	48 89 c7             	mov    rdi,rax
    c43e:	e8 4b a9 ff ff       	call   6d8e <die>
    c443:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    c447:	c9                   	leave
    c448:	c3                   	ret

000000000000c449 <nth_builtin>:
    c449:	f3 0f 1e fa          	endbr64
    c44d:	55                   	push   rbp
    c44e:	48 89 e5             	mov    rbp,rsp
    c451:	48 83 ec 30          	sub    rsp,0x30
    c455:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    c459:	48 89 75 d0          	mov    QWORD PTR [rbp-0x30],rsi
    c45d:	48 83 7d d8 12       	cmp    QWORD PTR [rbp-0x28],0x12
    c462:	75 0a                	jne    c46e <nth_builtin+0x25>
    c464:	b8 2a 00 00 00       	mov    eax,0x2a
    c469:	e9 11 01 00 00       	jmp    c57f <nth_builtin+0x136>
    c46e:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c472:	48 89 c7             	mov    rdi,rax
    c475:	e8 50 a9 ff ff       	call   6dca <obj_type>
    c47a:	83 f8 11             	cmp    eax,0x11
    c47d:	74 5a                	je     c4d9 <nth_builtin+0x90>
    c47f:	83 f8 11             	cmp    eax,0x11
    c482:	0f 8f f2 00 00 00    	jg     c57a <nth_builtin+0x131>
    c488:	83 f8 02             	cmp    eax,0x2
    c48b:	0f 84 81 00 00 00    	je     c512 <nth_builtin+0xc9>
    c491:	83 f8 05             	cmp    eax,0x5
    c494:	0f 85 e0 00 00 00    	jne    c57a <nth_builtin+0x131>
    c49a:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c49e:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    c4a2:	48 83 7d d0 00       	cmp    QWORD PTR [rbp-0x30],0x0
    c4a7:	78 26                	js     c4cf <nth_builtin+0x86>
    c4a9:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    c4ad:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    c4b1:	48 39 45 d0          	cmp    QWORD PTR [rbp-0x30],rax
    c4b5:	7d 18                	jge    c4cf <nth_builtin+0x86>
    c4b7:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    c4bb:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    c4bf:	48 89 d6             	mov    rsi,rdx
    c4c2:	48 89 c7             	mov    rdi,rax
    c4c5:	e8 15 bb ff ff       	call   7fdf <pv_nth>
    c4ca:	e9 b0 00 00 00       	jmp    c57f <nth_builtin+0x136>
    c4cf:	b8 2a 00 00 00       	mov    eax,0x2a
    c4d4:	e9 a6 00 00 00       	jmp    c57f <nth_builtin+0x136>
    c4d9:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c4dd:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    c4e1:	48 83 7d d0 00       	cmp    QWORD PTR [rbp-0x30],0x0
    c4e6:	78 23                	js     c50b <nth_builtin+0xc2>
    c4e8:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    c4ec:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    c4f0:	48 39 45 d0          	cmp    QWORD PTR [rbp-0x30],rax
    c4f4:	7d 15                	jge    c50b <nth_builtin+0xc2>
    c4f6:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    c4fa:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    c4fe:	48 89 d6             	mov    rsi,rdx
    c501:	48 89 c7             	mov    rdi,rax
    c504:	e8 d6 ba ff ff       	call   7fdf <pv_nth>
    c509:	eb 74                	jmp    c57f <nth_builtin+0x136>
    c50b:	b8 2a 00 00 00       	mov    eax,0x2a
    c510:	eb 6d                	jmp    c57f <nth_builtin+0x136>
    c512:	48 83 7d d0 00       	cmp    QWORD PTR [rbp-0x30],0x0
    c517:	79 07                	jns    c520 <nth_builtin+0xd7>
    c519:	b8 2a 00 00 00       	mov    eax,0x2a
    c51e:	eb 5f                	jmp    c57f <nth_builtin+0x136>
    c520:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c524:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    c528:	eb 0c                	jmp    c536 <nth_builtin+0xed>
    c52a:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c52e:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    c532:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    c536:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    c53a:	48 8d 50 ff          	lea    rdx,[rax-0x1]
    c53e:	48 89 55 d0          	mov    QWORD PTR [rbp-0x30],rdx
    c542:	48 85 c0             	test   rax,rax
    c545:	7e 11                	jle    c558 <nth_builtin+0x10f>
    c547:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c54b:	48 89 c7             	mov    rdi,rax
    c54e:	e8 77 a8 ff ff       	call   6dca <obj_type>
    c553:	83 f8 02             	cmp    eax,0x2
    c556:	74 d2                	je     c52a <nth_builtin+0xe1>
    c558:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c55c:	48 89 c7             	mov    rdi,rax
    c55f:	e8 66 a8 ff ff       	call   6dca <obj_type>
    c564:	83 f8 02             	cmp    eax,0x2
    c567:	75 0a                	jne    c573 <nth_builtin+0x12a>
    c569:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c56d:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    c571:	eb 0c                	jmp    c57f <nth_builtin+0x136>
    c573:	b8 2a 00 00 00       	mov    eax,0x2a
    c578:	eb 05                	jmp    c57f <nth_builtin+0x136>
    c57a:	b8 1a 00 00 00       	mov    eax,0x1a
    c57f:	c9                   	leave
    c580:	c3                   	ret

000000000000c581 <cljn_nth>:
    c581:	f3 0f 1e fa          	endbr64
    c585:	55                   	push   rbp
    c586:	48 89 e5             	mov    rbp,rsp
    c589:	48 83 ec 30          	sub    rsp,0x30
    c58d:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    c591:	48 89 75 d0          	mov    QWORD PTR [rbp-0x30],rsi
    c595:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    c599:	83 e0 01             	and    eax,0x1
    c59c:	48 85 c0             	test   rax,rax
    c59f:	75 0f                	jne    c5b0 <cljn_nth+0x2f>
    c5a1:	48 8d 05 44 3d 00 00 	lea    rax,[rip+0x3d44]        # 102ec <_IO_stdin_used+0x2ec>
    c5a8:	48 89 c7             	mov    rdi,rax
    c5ab:	e8 de a7 ff ff       	call   6d8e <die>
    c5b0:	48 83 7d d8 02       	cmp    QWORD PTR [rbp-0x28],0x2
    c5b5:	75 0a                	jne    c5c1 <cljn_nth+0x40>
    c5b7:	b8 02 00 00 00       	mov    eax,0x2
    c5bc:	e9 98 00 00 00       	jmp    c659 <cljn_nth+0xd8>
    c5c1:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    c5c5:	48 d1 f8             	sar    rax,1
    c5c8:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    c5cc:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    c5d0:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c5d4:	48 89 d6             	mov    rsi,rdx
    c5d7:	48 89 c7             	mov    rdi,rax
    c5da:	e8 6a fe ff ff       	call   c449 <nth_builtin>
    c5df:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    c5e3:	48 83 7d f0 1a       	cmp    QWORD PTR [rbp-0x10],0x1a
    c5e8:	74 1c                	je     c606 <cljn_nth+0x85>
    c5ea:	48 83 7d f0 2a       	cmp    QWORD PTR [rbp-0x10],0x2a
    c5ef:	75 0f                	jne    c600 <cljn_nth+0x7f>
    c5f1:	48 8d 05 12 3d 00 00 	lea    rax,[rip+0x3d12]        # 1030a <_IO_stdin_used+0x30a>
    c5f8:	48 89 c7             	mov    rdi,rax
    c5fb:	e8 8e a7 ff ff       	call   6d8e <die>
    c600:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    c604:	eb 53                	jmp    c659 <cljn_nth+0xd8>
    c606:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c60a:	48 89 c7             	mov    rdi,rax
    c60d:	e8 e3 f4 ff ff       	call   baf5 <cljn_type_key>
    c612:	48 89 c6             	mov    rsi,rax
    c615:	48 c7 c7 fe ff ff ff 	mov    rdi,0xfffffffffffffffe
    c61c:	e8 72 f6 ff ff       	call   bc93 <cljn_lookup_method>
    c621:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    c625:	48 83 7d f8 02       	cmp    QWORD PTR [rbp-0x8],0x2
    c62a:	74 19                	je     c645 <cljn_nth+0xc4>
    c62c:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    c630:	48 8b 4d d8          	mov    rcx,QWORD PTR [rbp-0x28]
    c634:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    c638:	48 89 ce             	mov    rsi,rcx
    c63b:	48 89 c7             	mov    rdi,rax
    c63e:	e8 e8 2c 00 00       	call   f32b <call_fn2>
    c643:	eb 14                	jmp    c659 <cljn_nth+0xd8>
    c645:	48 8d 05 dc 3c 00 00 	lea    rax,[rip+0x3cdc]        # 10328 <_IO_stdin_used+0x328>
    c64c:	48 89 c7             	mov    rdi,rax
    c64f:	e8 3a a7 ff ff       	call   6d8e <die>
    c654:	b8 02 00 00 00       	mov    eax,0x2
    c659:	c9                   	leave
    c65a:	c3                   	ret

000000000000c65b <cljn_nth_or>:
    c65b:	f3 0f 1e fa          	endbr64
    c65f:	55                   	push   rbp
    c660:	48 89 e5             	mov    rbp,rsp
    c663:	48 83 ec 40          	sub    rsp,0x40
    c667:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    c66b:	48 89 75 d0          	mov    QWORD PTR [rbp-0x30],rsi
    c66f:	48 89 55 c8          	mov    QWORD PTR [rbp-0x38],rdx
    c673:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    c677:	83 e0 01             	and    eax,0x1
    c67a:	48 85 c0             	test   rax,rax
    c67d:	75 0f                	jne    c68e <cljn_nth_or+0x33>
    c67f:	48 8d 05 66 3c 00 00 	lea    rax,[rip+0x3c66]        # 102ec <_IO_stdin_used+0x2ec>
    c686:	48 89 c7             	mov    rdi,rax
    c689:	e8 00 a7 ff ff       	call   6d8e <die>
    c68e:	48 83 7d d8 02       	cmp    QWORD PTR [rbp-0x28],0x2
    c693:	75 09                	jne    c69e <cljn_nth_or+0x43>
    c695:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    c699:	e9 8f 00 00 00       	jmp    c72d <cljn_nth_or+0xd2>
    c69e:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    c6a2:	48 d1 f8             	sar    rax,1
    c6a5:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    c6a9:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    c6ad:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c6b1:	48 89 d6             	mov    rsi,rdx
    c6b4:	48 89 c7             	mov    rdi,rax
    c6b7:	e8 8d fd ff ff       	call   c449 <nth_builtin>
    c6bc:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    c6c0:	48 83 7d f0 1a       	cmp    QWORD PTR [rbp-0x10],0x1a
    c6c5:	74 13                	je     c6da <cljn_nth_or+0x7f>
    c6c7:	48 83 7d f0 2a       	cmp    QWORD PTR [rbp-0x10],0x2a
    c6cc:	75 06                	jne    c6d4 <cljn_nth_or+0x79>
    c6ce:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    c6d2:	eb 59                	jmp    c72d <cljn_nth_or+0xd2>
    c6d4:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    c6d8:	eb 53                	jmp    c72d <cljn_nth_or+0xd2>
    c6da:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c6de:	48 89 c7             	mov    rdi,rax
    c6e1:	e8 0f f4 ff ff       	call   baf5 <cljn_type_key>
    c6e6:	48 89 c6             	mov    rsi,rax
    c6e9:	48 c7 c7 fd ff ff ff 	mov    rdi,0xfffffffffffffffd
    c6f0:	e8 9e f5 ff ff       	call   bc93 <cljn_lookup_method>
    c6f5:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    c6f9:	48 83 7d f8 02       	cmp    QWORD PTR [rbp-0x8],0x2
    c6fe:	74 1a                	je     c71a <cljn_nth_or+0xbf>
    c700:	48 8b 4d c8          	mov    rcx,QWORD PTR [rbp-0x38]
    c704:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    c708:	48 8b 75 d8          	mov    rsi,QWORD PTR [rbp-0x28]
    c70c:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    c710:	48 89 c7             	mov    rdi,rax
    c713:	e8 cf 2c 00 00       	call   f3e7 <call_fn3>
    c718:	eb 13                	jmp    c72d <cljn_nth_or+0xd2>
    c71a:	48 8d 05 07 3c 00 00 	lea    rax,[rip+0x3c07]        # 10328 <_IO_stdin_used+0x328>
    c721:	48 89 c7             	mov    rdi,rax
    c724:	e8 65 a6 ff ff       	call   6d8e <die>
    c729:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    c72d:	c9                   	leave
    c72e:	c3                   	ret

000000000000c72f <cljn_transient>:
    c72f:	f3 0f 1e fa          	endbr64
    c733:	55                   	push   rbp
    c734:	48 89 e5             	mov    rbp,rsp
    c737:	48 83 ec 30          	sub    rsp,0x30
    c73b:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    c73f:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c743:	48 89 c7             	mov    rdi,rax
    c746:	e8 7f a6 ff ff       	call   6dca <obj_type>
    c74b:	83 f8 05             	cmp    eax,0x5
    c74e:	0f 85 c6 00 00 00    	jne    c81a <cljn_transient+0xeb>
    c754:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c758:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    c75c:	e8 c0 a7 ff ff       	call   6f21 <maybe_gc>
    c761:	8b 05 31 79 00 02    	mov    eax,DWORD PTR [rip+0x2007931]        # 2014098 <gc_disabled>
    c767:	83 c0 01             	add    eax,0x1
    c76a:	89 05 28 79 00 02    	mov    DWORD PTR [rip+0x2007928],eax        # 2014098 <gc_disabled>
    c770:	e8 f6 b4 ff ff       	call   7c6b <cljn_edit_new>
    c775:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    c779:	be 11 00 00 00       	mov    esi,0x11
    c77e:	bf 40 00 00 00       	mov    edi,0x40
    c783:	e8 eb a7 ff ff       	call   6f73 <obj_alloc>
    c788:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    c78c:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c790:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    c794:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c798:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    c79c:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    c7a0:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    c7a4:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c7a8:	48 8b 50 18          	mov    rdx,QWORD PTR [rax+0x18]
    c7ac:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    c7b0:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    c7b4:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c7b8:	48 8b 50 20          	mov    rdx,QWORD PTR [rax+0x20]
    c7bc:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    c7c0:	48 89 50 20          	mov    QWORD PTR [rax+0x20],rdx
    c7c4:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c7c8:	48 8b 40 28          	mov    rax,QWORD PTR [rax+0x28]
    c7cc:	48 89 c2             	mov    rdx,rax
    c7cf:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    c7d3:	48 89 c6             	mov    rsi,rax
    c7d6:	48 89 d7             	mov    rdi,rdx
    c7d9:	e8 d1 b4 ff ff       	call   7caf <vnode_copy_edit>
    c7de:	48 89 c2             	mov    rdx,rax
    c7e1:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    c7e5:	48 89 50 28          	mov    QWORD PTR [rax+0x28],rdx
    c7e9:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c7ed:	48 8b 50 30          	mov    rdx,QWORD PTR [rax+0x30]
    c7f1:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    c7f5:	48 89 50 30          	mov    QWORD PTR [rax+0x30],rdx
    c7f9:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    c7fd:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    c801:	48 89 50 38          	mov    QWORD PTR [rax+0x38],rdx
    c805:	8b 05 8d 78 00 02    	mov    eax,DWORD PTR [rip+0x200788d]        # 2014098 <gc_disabled>
    c80b:	83 e8 01             	sub    eax,0x1
    c80e:	89 05 84 78 00 02    	mov    DWORD PTR [rip+0x2007884],eax        # 2014098 <gc_disabled>
    c814:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    c818:	eb 39                	jmp    c853 <cljn_transient+0x124>
    c81a:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c81e:	48 89 c7             	mov    rdi,rax
    c821:	e8 3e a4 ff ff       	call   6c64 <cljn_gc_push>
    c826:	be 12 00 00 00       	mov    esi,0x12
    c82b:	bf 18 00 00 00       	mov    edi,0x18
    c830:	e8 3e a7 ff ff       	call   6f73 <obj_alloc>
    c835:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    c839:	bf 01 00 00 00       	mov    edi,0x1
    c83e:	e8 99 a4 ff ff       	call   6cdc <cljn_gc_popn>
    c843:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    c847:	48 8b 55 d8          	mov    rdx,QWORD PTR [rbp-0x28]
    c84b:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    c84f:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    c853:	c9                   	leave
    c854:	c3                   	ret

000000000000c855 <tv_check>:
    c855:	f3 0f 1e fa          	endbr64
    c859:	55                   	push   rbp
    c85a:	48 89 e5             	mov    rbp,rsp
    c85d:	48 83 ec 10          	sub    rsp,0x10
    c861:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    c865:	48 89 75 f0          	mov    QWORD PTR [rbp-0x10],rsi
    c869:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    c86d:	48 8b 40 38          	mov    rax,QWORD PTR [rax+0x38]
    c871:	48 83 f8 02          	cmp    rax,0x2
    c875:	75 2c                	jne    c8a3 <tv_check+0x4e>
    c877:	48 8b 05 e2 77 00 00 	mov    rax,QWORD PTR [rip+0x77e2]        # 14060 <stderr@GLIBC_2.2.5>
    c87e:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    c882:	48 8d 0d cf 3a 00 00 	lea    rcx,[rip+0x3acf]        # 10358 <_IO_stdin_used+0x358>
    c889:	48 89 ce             	mov    rsi,rcx
    c88c:	48 89 c7             	mov    rdi,rax
    c88f:	b8 00 00 00 00       	mov    eax,0x0
    c894:	e8 17 48 ff ff       	call   10b0 <fprintf@plt>
    c899:	bf 01 00 00 00       	mov    edi,0x1
    c89e:	e8 5d 48 ff ff       	call   1100 <exit@plt>
    c8a3:	90                   	nop
    c8a4:	c9                   	leave
    c8a5:	c3                   	ret

000000000000c8a6 <cljn_conj_bang>:
    c8a6:	f3 0f 1e fa          	endbr64
    c8aa:	55                   	push   rbp
    c8ab:	48 89 e5             	mov    rbp,rsp
    c8ae:	48 83 ec 50          	sub    rsp,0x50
    c8b2:	48 89 7d b8          	mov    QWORD PTR [rbp-0x48],rdi
    c8b6:	48 89 75 b0          	mov    QWORD PTR [rbp-0x50],rsi
    c8ba:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    c8be:	48 89 c7             	mov    rdi,rax
    c8c1:	e8 04 a5 ff ff       	call   6dca <obj_type>
    c8c6:	83 f8 11             	cmp    eax,0x11
    c8c9:	0f 85 e1 01 00 00    	jne    cab0 <cljn_conj_bang+0x20a>
    c8cf:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    c8d3:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    c8d7:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c8db:	48 8d 15 9b 3a 00 00 	lea    rdx,[rip+0x3a9b]        # 1037d <_IO_stdin_used+0x37d>
    c8e2:	48 89 d6             	mov    rsi,rdx
    c8e5:	48 89 c7             	mov    rdi,rax
    c8e8:	e8 68 ff ff ff       	call   c855 <tv_check>
    c8ed:	e8 2f a6 ff ff       	call   6f21 <maybe_gc>
    c8f2:	8b 05 a0 77 00 02    	mov    eax,DWORD PTR [rip+0x20077a0]        # 2014098 <gc_disabled>
    c8f8:	83 c0 01             	add    eax,0x1
    c8fb:	89 05 97 77 00 02    	mov    DWORD PTR [rip+0x2007797],eax        # 2014098 <gc_disabled>
    c901:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c905:	48 8b 40 30          	mov    rax,QWORD PTR [rax+0x30]
    c909:	48 83 f8 1f          	cmp    rax,0x1f
    c90d:	7f 5d                	jg     c96c <cljn_conj_bang+0xc6>
    c90f:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c913:	48 8b 40 38          	mov    rax,QWORD PTR [rax+0x38]
    c917:	48 8b 55 d8          	mov    rdx,QWORD PTR [rbp-0x28]
    c91b:	48 8b 52 28          	mov    rdx,QWORD PTR [rdx+0x28]
    c91f:	48 89 c6             	mov    rsi,rax
    c922:	48 89 d7             	mov    rdi,rdx
    c925:	e8 bb b3 ff ff       	call   7ce5 <vnode_editable>
    c92a:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    c92e:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c932:	48 8b 50 30          	mov    rdx,QWORD PTR [rax+0x30]
    c936:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    c93a:	48 8d 4a 02          	lea    rcx,[rdx+0x2]
    c93e:	48 8b 55 b0          	mov    rdx,QWORD PTR [rbp-0x50]
    c942:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    c947:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    c94b:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c94f:	48 89 50 28          	mov    QWORD PTR [rax+0x28],rdx
    c953:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c957:	48 8b 40 30          	mov    rax,QWORD PTR [rax+0x30]
    c95b:	48 8d 50 01          	lea    rdx,[rax+0x1]
    c95f:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c963:	48 89 50 30          	mov    QWORD PTR [rax+0x30],rdx
    c967:	e9 1b 01 00 00       	jmp    ca87 <cljn_conj_bang+0x1e1>
    c96c:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c970:	48 8b 40 28          	mov    rax,QWORD PTR [rax+0x28]
    c974:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    c978:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c97c:	48 8b 40 38          	mov    rax,QWORD PTR [rax+0x38]
    c980:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    c984:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c988:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    c98c:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
    c990:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c994:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    c998:	48 c1 f8 05          	sar    rax,0x5
    c99c:	48 89 c2             	mov    rdx,rax
    c99f:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c9a3:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    c9a7:	be 01 00 00 00       	mov    esi,0x1
    c9ac:	89 c1                	mov    ecx,eax
    c9ae:	48 d3 e6             	shl    rsi,cl
    c9b1:	48 89 f0             	mov    rax,rsi
    c9b4:	48 39 c2             	cmp    rdx,rax
    c9b7:	7e 4d                	jle    ca06 <cljn_conj_bang+0x160>
    c9b9:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    c9bd:	48 89 c7             	mov    rdi,rax
    c9c0:	e8 bf b2 ff ff       	call   7c84 <vnode_new_edit>
    c9c5:	48 89 45 c0          	mov    QWORD PTR [rbp-0x40],rax
    c9c9:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c9cd:	48 8b 50 20          	mov    rdx,QWORD PTR [rax+0x20]
    c9d1:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    c9d5:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    c9d9:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    c9dd:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    c9e1:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    c9e5:	48 8b 4d e0          	mov    rcx,QWORD PTR [rbp-0x20]
    c9e9:	48 89 ce             	mov    rsi,rcx
    c9ec:	48 89 c7             	mov    rdi,rax
    c9ef:	e8 2e b3 ff ff       	call   7d22 <new_path_edit>
    c9f4:	48 89 c2             	mov    rdx,rax
    c9f7:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    c9fb:	48 89 50 20          	mov    QWORD PTR [rax+0x20],rdx
    c9ff:	48 83 45 c8 05       	add    QWORD PTR [rbp-0x38],0x5
    ca04:	eb 35                	jmp    ca3b <cljn_conj_bang+0x195>
    ca06:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    ca0a:	48 8b 48 10          	mov    rcx,QWORD PTR [rax+0x10]
    ca0e:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    ca12:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    ca16:	48 89 c7             	mov    rdi,rax
    ca19:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    ca1d:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    ca21:	48 8b 75 e8          	mov    rsi,QWORD PTR [rbp-0x18]
    ca25:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    ca29:	49 89 f0             	mov    r8,rsi
    ca2c:	48 89 fe             	mov    rsi,rdi
    ca2f:	48 89 c7             	mov    rdi,rax
    ca32:	e8 4c b3 ff ff       	call   7d83 <tv_push_tail>
    ca37:	48 89 45 c0          	mov    QWORD PTR [rbp-0x40],rax
    ca3b:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    ca3f:	48 89 c7             	mov    rdi,rax
    ca42:	e8 3d b2 ff ff       	call   7c84 <vnode_new_edit>
    ca47:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    ca4b:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    ca4f:	48 8b 55 b0          	mov    rdx,QWORD PTR [rbp-0x50]
    ca53:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    ca57:	48 8b 55 c0          	mov    rdx,QWORD PTR [rbp-0x40]
    ca5b:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    ca5f:	48 89 50 20          	mov    QWORD PTR [rax+0x20],rdx
    ca63:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    ca67:	48 8b 55 c8          	mov    rdx,QWORD PTR [rbp-0x38]
    ca6b:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    ca6f:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    ca73:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    ca77:	48 89 50 28          	mov    QWORD PTR [rax+0x28],rdx
    ca7b:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    ca7f:	48 c7 40 30 01 00 00 	mov    QWORD PTR [rax+0x30],0x1
    ca86:	00 
    ca87:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    ca8b:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    ca8f:	48 8d 50 01          	lea    rdx,[rax+0x1]
    ca93:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    ca97:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    ca9b:	8b 05 f7 75 00 02    	mov    eax,DWORD PTR [rip+0x20075f7]        # 2014098 <gc_disabled>
    caa1:	83 e8 01             	sub    eax,0x1
    caa4:	89 05 ee 75 00 02    	mov    DWORD PTR [rip+0x20075ee],eax        # 2014098 <gc_disabled>
    caaa:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    caae:	eb 51                	jmp    cb01 <cljn_conj_bang+0x25b>
    cab0:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    cab4:	48 89 c7             	mov    rdi,rax
    cab7:	e8 0e a3 ff ff       	call   6dca <obj_type>
    cabc:	83 f8 12             	cmp    eax,0x12
    cabf:	75 2d                	jne    caee <cljn_conj_bang+0x248>
    cac1:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    cac5:	48 89 45 d0          	mov    QWORD PTR [rbp-0x30],rax
    cac9:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    cacd:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    cad1:	48 8b 55 b0          	mov    rdx,QWORD PTR [rbp-0x50]
    cad5:	48 89 d6             	mov    rsi,rdx
    cad8:	48 89 c7             	mov    rdi,rax
    cadb:	e8 11 f6 ff ff       	call   c0f1 <cljn_conj>
    cae0:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    cae4:	48 89 42 10          	mov    QWORD PTR [rdx+0x10],rax
    cae8:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    caec:	eb 13                	jmp    cb01 <cljn_conj_bang+0x25b>
    caee:	48 8d 05 8e 38 00 00 	lea    rax,[rip+0x388e]        # 10383 <_IO_stdin_used+0x383>
    caf5:	48 89 c7             	mov    rdi,rax
    caf8:	e8 91 a2 ff ff       	call   6d8e <die>
    cafd:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    cb01:	c9                   	leave
    cb02:	c3                   	ret

000000000000cb03 <cljn_assoc_bang>:
    cb03:	f3 0f 1e fa          	endbr64
    cb07:	55                   	push   rbp
    cb08:	48 89 e5             	mov    rbp,rsp
    cb0b:	48 83 ec 50          	sub    rsp,0x50
    cb0f:	48 89 7d c8          	mov    QWORD PTR [rbp-0x38],rdi
    cb13:	48 89 75 c0          	mov    QWORD PTR [rbp-0x40],rsi
    cb17:	48 89 55 b8          	mov    QWORD PTR [rbp-0x48],rdx
    cb1b:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    cb1f:	48 89 c7             	mov    rdi,rax
    cb22:	e8 a3 a2 ff ff       	call   6dca <obj_type>
    cb27:	83 f8 11             	cmp    eax,0x11
    cb2a:	0f 85 5d 01 00 00    	jne    cc8d <cljn_assoc_bang+0x18a>
    cb30:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    cb34:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    cb38:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    cb3c:	48 8d 15 5c 38 00 00 	lea    rdx,[rip+0x385c]        # 1039f <_IO_stdin_used+0x39f>
    cb43:	48 89 d6             	mov    rsi,rdx
    cb46:	48 89 c7             	mov    rdi,rax
    cb49:	e8 07 fd ff ff       	call   c855 <tv_check>
    cb4e:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    cb52:	83 e0 01             	and    eax,0x1
    cb55:	48 85 c0             	test   rax,rax
    cb58:	75 0f                	jne    cb69 <cljn_assoc_bang+0x66>
    cb5a:	48 8d 05 47 38 00 00 	lea    rax,[rip+0x3847]        # 103a8 <_IO_stdin_used+0x3a8>
    cb61:	48 89 c7             	mov    rdi,rax
    cb64:	e8 25 a2 ff ff       	call   6d8e <die>
    cb69:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    cb6d:	48 d1 f8             	sar    rax,1
    cb70:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    cb74:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    cb78:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    cb7c:	48 39 45 e8          	cmp    QWORD PTR [rbp-0x18],rax
    cb80:	75 18                	jne    cb9a <cljn_assoc_bang+0x97>
    cb82:	48 8b 55 b8          	mov    rdx,QWORD PTR [rbp-0x48]
    cb86:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    cb8a:	48 89 d6             	mov    rsi,rdx
    cb8d:	48 89 c7             	mov    rdi,rax
    cb90:	e8 11 fd ff ff       	call   c8a6 <cljn_conj_bang>
    cb95:	e9 48 01 00 00       	jmp    cce2 <cljn_assoc_bang+0x1df>
    cb9a:	48 83 7d e8 00       	cmp    QWORD PTR [rbp-0x18],0x0
    cb9f:	78 0e                	js     cbaf <cljn_assoc_bang+0xac>
    cba1:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    cba5:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    cba9:	48 39 45 e8          	cmp    QWORD PTR [rbp-0x18],rax
    cbad:	7e 0f                	jle    cbbe <cljn_assoc_bang+0xbb>
    cbaf:	48 8d 05 22 38 00 00 	lea    rax,[rip+0x3822]        # 103d8 <_IO_stdin_used+0x3d8>
    cbb6:	48 89 c7             	mov    rdi,rax
    cbb9:	e8 d0 a1 ff ff       	call   6d8e <die>
    cbbe:	e8 5e a3 ff ff       	call   6f21 <maybe_gc>
    cbc3:	8b 05 cf 74 00 02    	mov    eax,DWORD PTR [rip+0x20074cf]        # 2014098 <gc_disabled>
    cbc9:	83 c0 01             	add    eax,0x1
    cbcc:	89 05 c6 74 00 02    	mov    DWORD PTR [rip+0x20074c6],eax        # 2014098 <gc_disabled>
    cbd2:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    cbd6:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    cbda:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    cbde:	48 8b 40 30          	mov    rax,QWORD PTR [rax+0x30]
    cbe2:	48 29 c2             	sub    rdx,rax
    cbe5:	48 89 55 f0          	mov    QWORD PTR [rbp-0x10],rdx
    cbe9:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    cbed:	48 3b 45 f0          	cmp    rax,QWORD PTR [rbp-0x10]
    cbf1:	7c 49                	jl     cc3c <cljn_assoc_bang+0x139>
    cbf3:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    cbf7:	48 8b 40 38          	mov    rax,QWORD PTR [rax+0x38]
    cbfb:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    cbff:	48 8b 52 28          	mov    rdx,QWORD PTR [rdx+0x28]
    cc03:	48 89 c6             	mov    rsi,rax
    cc06:	48 89 d7             	mov    rdi,rdx
    cc09:	e8 d7 b0 ff ff       	call   7ce5 <vnode_editable>
    cc0e:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    cc12:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    cc16:	48 2b 45 f0          	sub    rax,QWORD PTR [rbp-0x10]
    cc1a:	48 89 c2             	mov    rdx,rax
    cc1d:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    cc21:	48 8d 4a 02          	lea    rcx,[rdx+0x2]
    cc25:	48 8b 55 b8          	mov    rdx,QWORD PTR [rbp-0x48]
    cc29:	48 89 54 c8 08       	mov    QWORD PTR [rax+rcx*8+0x8],rdx
    cc2e:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    cc32:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    cc36:	48 89 50 28          	mov    QWORD PTR [rax+0x28],rdx
    cc3a:	eb 3c                	jmp    cc78 <cljn_assoc_bang+0x175>
    cc3c:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    cc40:	48 8b 70 38          	mov    rsi,QWORD PTR [rax+0x38]
    cc44:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    cc48:	48 8b 40 20          	mov    rax,QWORD PTR [rax+0x20]
    cc4c:	48 89 c7             	mov    rdi,rax
    cc4f:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    cc53:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    cc57:	48 8b 4d b8          	mov    rcx,QWORD PTR [rbp-0x48]
    cc5b:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    cc5f:	49 89 f0             	mov    r8,rsi
    cc62:	48 89 fe             	mov    rsi,rdi
    cc65:	48 89 c7             	mov    rdi,rax
    cc68:	e8 fe b1 ff ff       	call   7e6b <tv_do_assoc>
    cc6d:	48 89 c2             	mov    rdx,rax
    cc70:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    cc74:	48 89 50 20          	mov    QWORD PTR [rax+0x20],rdx
    cc78:	8b 05 1a 74 00 02    	mov    eax,DWORD PTR [rip+0x200741a]        # 2014098 <gc_disabled>
    cc7e:	83 e8 01             	sub    eax,0x1
    cc81:	89 05 11 74 00 02    	mov    DWORD PTR [rip+0x2007411],eax        # 2014098 <gc_disabled>
    cc87:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    cc8b:	eb 55                	jmp    cce2 <cljn_assoc_bang+0x1df>
    cc8d:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    cc91:	48 89 c7             	mov    rdi,rax
    cc94:	e8 31 a1 ff ff       	call   6dca <obj_type>
    cc99:	83 f8 12             	cmp    eax,0x12
    cc9c:	75 31                	jne    cccf <cljn_assoc_bang+0x1cc>
    cc9e:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    cca2:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    cca6:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    ccaa:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    ccae:	48 8b 55 b8          	mov    rdx,QWORD PTR [rbp-0x48]
    ccb2:	48 8b 4d c0          	mov    rcx,QWORD PTR [rbp-0x40]
    ccb6:	48 89 ce             	mov    rsi,rcx
    ccb9:	48 89 c7             	mov    rdi,rax
    ccbc:	e8 fd f5 ff ff       	call   c2be <cljn_assoc>
    ccc1:	48 8b 55 d8          	mov    rdx,QWORD PTR [rbp-0x28]
    ccc5:	48 89 42 10          	mov    QWORD PTR [rdx+0x10],rax
    ccc9:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    cccd:	eb 13                	jmp    cce2 <cljn_assoc_bang+0x1df>
    cccf:	48 8d 05 23 37 00 00 	lea    rax,[rip+0x3723]        # 103f9 <_IO_stdin_used+0x3f9>
    ccd6:	48 89 c7             	mov    rdi,rax
    ccd9:	e8 b0 a0 ff ff       	call   6d8e <die>
    ccde:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    cce2:	c9                   	leave
    cce3:	c3                   	ret

000000000000cce4 <cljn_dissoc_bang>:
    cce4:	f3 0f 1e fa          	endbr64
    cce8:	55                   	push   rbp
    cce9:	48 89 e5             	mov    rbp,rsp
    ccec:	48 83 ec 20          	sub    rsp,0x20
    ccf0:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    ccf4:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    ccf8:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    ccfc:	48 89 c7             	mov    rdi,rax
    ccff:	e8 c6 a0 ff ff       	call   6dca <obj_type>
    cd04:	83 f8 12             	cmp    eax,0x12
    cd07:	75 2d                	jne    cd36 <cljn_dissoc_bang+0x52>
    cd09:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    cd0d:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    cd11:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    cd15:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    cd19:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    cd1d:	48 89 d6             	mov    rsi,rdx
    cd20:	48 89 c7             	mov    rdi,rax
    cd23:	e8 7b d5 ff ff       	call   a2a3 <cljn_map_dissoc>
    cd28:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    cd2c:	48 89 42 10          	mov    QWORD PTR [rdx+0x10],rax
    cd30:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    cd34:	eb 13                	jmp    cd49 <cljn_dissoc_bang+0x65>
    cd36:	48 8d 05 db 36 00 00 	lea    rax,[rip+0x36db]        # 10418 <_IO_stdin_used+0x418>
    cd3d:	48 89 c7             	mov    rdi,rax
    cd40:	e8 49 a0 ff ff       	call   6d8e <die>
    cd45:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    cd49:	c9                   	leave
    cd4a:	c3                   	ret

000000000000cd4b <cljn_persistent_bang>:
    cd4b:	f3 0f 1e fa          	endbr64
    cd4f:	55                   	push   rbp
    cd50:	48 89 e5             	mov    rbp,rsp
    cd53:	48 83 ec 20          	sub    rsp,0x20
    cd57:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    cd5b:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    cd5f:	48 89 c7             	mov    rdi,rax
    cd62:	e8 63 a0 ff ff       	call   6dca <obj_type>
    cd67:	83 f8 12             	cmp    eax,0x12
    cd6a:	75 0d                	jne    cd79 <cljn_persistent_bang+0x2e>
    cd6c:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    cd70:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    cd74:	e9 e6 00 00 00       	jmp    ce5f <cljn_persistent_bang+0x114>
    cd79:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    cd7d:	48 89 c7             	mov    rdi,rax
    cd80:	e8 45 a0 ff ff       	call   6dca <obj_type>
    cd85:	83 f8 11             	cmp    eax,0x11
    cd88:	0f 85 be 00 00 00    	jne    ce4c <cljn_persistent_bang+0x101>
    cd8e:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    cd92:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    cd96:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    cd9a:	48 8d 15 9a 36 00 00 	lea    rdx,[rip+0x369a]        # 1043b <_IO_stdin_used+0x43b>
    cda1:	48 89 d6             	mov    rsi,rdx
    cda4:	48 89 c7             	mov    rdi,rax
    cda7:	e8 a9 fa ff ff       	call   c855 <tv_check>
    cdac:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    cdb0:	48 c7 40 38 02 00 00 	mov    QWORD PTR [rax+0x38],0x2
    cdb7:	00 
    cdb8:	e8 64 a1 ff ff       	call   6f21 <maybe_gc>
    cdbd:	8b 05 d5 72 00 02    	mov    eax,DWORD PTR [rip+0x20072d5]        # 2014098 <gc_disabled>
    cdc3:	83 c0 01             	add    eax,0x1
    cdc6:	89 05 cc 72 00 02    	mov    DWORD PTR [rip+0x20072cc],eax        # 2014098 <gc_disabled>
    cdcc:	be 05 00 00 00       	mov    esi,0x5
    cdd1:	bf 38 00 00 00       	mov    edi,0x38
    cdd6:	e8 98 a1 ff ff       	call   6f73 <obj_alloc>
    cddb:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    cddf:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    cde3:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    cde7:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    cdeb:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    cdef:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    cdf3:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    cdf7:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    cdfb:	48 8b 50 18          	mov    rdx,QWORD PTR [rax+0x18]
    cdff:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    ce03:	48 89 50 18          	mov    QWORD PTR [rax+0x18],rdx
    ce07:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    ce0b:	48 8b 50 20          	mov    rdx,QWORD PTR [rax+0x20]
    ce0f:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    ce13:	48 89 50 20          	mov    QWORD PTR [rax+0x20],rdx
    ce17:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    ce1b:	48 8b 50 28          	mov    rdx,QWORD PTR [rax+0x28]
    ce1f:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    ce23:	48 89 50 28          	mov    QWORD PTR [rax+0x28],rdx
    ce27:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    ce2b:	48 8b 50 30          	mov    rdx,QWORD PTR [rax+0x30]
    ce2f:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    ce33:	48 89 50 30          	mov    QWORD PTR [rax+0x30],rdx
    ce37:	8b 05 5b 72 00 02    	mov    eax,DWORD PTR [rip+0x200725b]        # 2014098 <gc_disabled>
    ce3d:	83 e8 01             	sub    eax,0x1
    ce40:	89 05 52 72 00 02    	mov    DWORD PTR [rip+0x2007252],eax        # 2014098 <gc_disabled>
    ce46:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    ce4a:	eb 13                	jmp    ce5f <cljn_persistent_bang+0x114>
    ce4c:	48 8d 05 f5 35 00 00 	lea    rax,[rip+0x35f5]        # 10448 <_IO_stdin_used+0x448>
    ce53:	48 89 c7             	mov    rdi,rax
    ce56:	e8 33 9f ff ff       	call   6d8e <die>
    ce5b:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    ce5f:	c9                   	leave
    ce60:	c3                   	ret

000000000000ce61 <need_fix>:
    ce61:	f3 0f 1e fa          	endbr64
    ce65:	55                   	push   rbp
    ce66:	48 89 e5             	mov    rbp,rsp
    ce69:	48 83 ec 10          	sub    rsp,0x10
    ce6d:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    ce71:	48 89 75 f0          	mov    QWORD PTR [rbp-0x10],rsi
    ce75:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    ce79:	83 e0 01             	and    eax,0x1
    ce7c:	48 85 c0             	test   rax,rax
    ce7f:	75 2c                	jne    cead <need_fix+0x4c>
    ce81:	48 8b 05 d8 71 00 00 	mov    rax,QWORD PTR [rip+0x71d8]        # 14060 <stderr@GLIBC_2.2.5>
    ce88:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    ce8c:	48 8d 0d dd 35 00 00 	lea    rcx,[rip+0x35dd]        # 10470 <_IO_stdin_used+0x470>
    ce93:	48 89 ce             	mov    rsi,rcx
    ce96:	48 89 c7             	mov    rdi,rax
    ce99:	b8 00 00 00 00       	mov    eax,0x0
    ce9e:	e8 0d 42 ff ff       	call   10b0 <fprintf@plt>
    cea3:	bf 01 00 00 00       	mov    edi,0x1
    cea8:	e8 53 42 ff ff       	call   1100 <exit@plt>
    cead:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    ceb1:	48 d1 f8             	sar    rax,1
    ceb4:	c9                   	leave
    ceb5:	c3                   	ret

000000000000ceb6 <mk_fix_checked>:
    ceb6:	f3 0f 1e fa          	endbr64
    ceba:	55                   	push   rbp
    cebb:	48 89 e5             	mov    rbp,rsp
    cebe:	48 83 ec 10          	sub    rsp,0x10
    cec2:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    cec6:	48 89 75 f0          	mov    QWORD PTR [rbp-0x10],rsi
    ceca:	48 b8 00 00 00 00 00 	movabs rax,0xc000000000000000
    ced1:	00 00 c0 
    ced4:	48 39 45 f8          	cmp    QWORD PTR [rbp-0x8],rax
    ced8:	7c 10                	jl     ceea <mk_fix_checked+0x34>
    ceda:	48 b8 ff ff ff ff ff 	movabs rax,0x3fffffffffffffff
    cee1:	ff ff 3f 
    cee4:	48 39 45 f8          	cmp    QWORD PTR [rbp-0x8],rax
    cee8:	7e 2c                	jle    cf16 <mk_fix_checked+0x60>
    ceea:	48 8b 05 6f 71 00 00 	mov    rax,QWORD PTR [rip+0x716f]        # 14060 <stderr@GLIBC_2.2.5>
    cef1:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    cef5:	48 8d 0d 9a 35 00 00 	lea    rcx,[rip+0x359a]        # 10496 <_IO_stdin_used+0x496>
    cefc:	48 89 ce             	mov    rsi,rcx
    ceff:	48 89 c7             	mov    rdi,rax
    cf02:	b8 00 00 00 00       	mov    eax,0x0
    cf07:	e8 a4 41 ff ff       	call   10b0 <fprintf@plt>
    cf0c:	bf 01 00 00 00       	mov    edi,0x1
    cf11:	e8 ea 41 ff ff       	call   1100 <exit@plt>
    cf16:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    cf1a:	48 01 c0             	add    rax,rax
    cf1d:	48 83 c8 01          	or     rax,0x1
    cf21:	c9                   	leave
    cf22:	c3                   	ret

000000000000cf23 <cljn_add>:
    cf23:	f3 0f 1e fa          	endbr64
    cf27:	55                   	push   rbp
    cf28:	48 89 e5             	mov    rbp,rsp
    cf2b:	53                   	push   rbx
    cf2c:	48 83 ec 28          	sub    rsp,0x28
    cf30:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    cf34:	48 89 75 d0          	mov    QWORD PTR [rbp-0x30],rsi
    cf38:	64 48 8b 04 25 28 00 	mov    rax,QWORD PTR fs:0x28
    cf3f:	00 00 
    cf41:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    cf45:	31 c0                	xor    eax,eax
    cf47:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    cf4b:	48 8d 15 5a 35 00 00 	lea    rdx,[rip+0x355a]        # 104ac <_IO_stdin_used+0x4ac>
    cf52:	48 89 d6             	mov    rsi,rdx
    cf55:	48 89 c7             	mov    rdi,rax
    cf58:	e8 04 ff ff ff       	call   ce61 <need_fix>
    cf5d:	48 89 c3             	mov    rbx,rax
    cf60:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    cf64:	48 8d 15 41 35 00 00 	lea    rdx,[rip+0x3541]        # 104ac <_IO_stdin_used+0x4ac>
    cf6b:	48 89 d6             	mov    rsi,rdx
    cf6e:	48 89 c7             	mov    rdi,rax
    cf71:	e8 eb fe ff ff       	call   ce61 <need_fix>
    cf76:	ba 00 00 00 00       	mov    edx,0x0
    cf7b:	48 01 d8             	add    rax,rbx
    cf7e:	71 05                	jno    cf85 <cljn_add+0x62>
    cf80:	ba 01 00 00 00       	mov    edx,0x1
    cf85:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    cf89:	48 89 d0             	mov    rax,rdx
    cf8c:	83 e0 01             	and    eax,0x1
    cf8f:	84 c0                	test   al,al
    cf91:	74 0f                	je     cfa2 <cljn_add+0x7f>
    cf93:	48 8d 05 14 35 00 00 	lea    rax,[rip+0x3514]        # 104ae <_IO_stdin_used+0x4ae>
    cf9a:	48 89 c7             	mov    rdi,rax
    cf9d:	e8 ec 9d ff ff       	call   6d8e <die>
    cfa2:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    cfa6:	48 8d 15 ff 34 00 00 	lea    rdx,[rip+0x34ff]        # 104ac <_IO_stdin_used+0x4ac>
    cfad:	48 89 d6             	mov    rsi,rdx
    cfb0:	48 89 c7             	mov    rdi,rax
    cfb3:	e8 fe fe ff ff       	call   ceb6 <mk_fix_checked>
    cfb8:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    cfbc:	64 48 2b 14 25 28 00 	sub    rdx,QWORD PTR fs:0x28
    cfc3:	00 00 
    cfc5:	74 05                	je     cfcc <cljn_add+0xa9>
    cfc7:	e8 94 40 ff ff       	call   1060 <__stack_chk_fail@plt>
    cfcc:	48 8b 5d f8          	mov    rbx,QWORD PTR [rbp-0x8]
    cfd0:	c9                   	leave
    cfd1:	c3                   	ret

000000000000cfd2 <cljn_sub>:
    cfd2:	f3 0f 1e fa          	endbr64
    cfd6:	55                   	push   rbp
    cfd7:	48 89 e5             	mov    rbp,rsp
    cfda:	53                   	push   rbx
    cfdb:	48 83 ec 28          	sub    rsp,0x28
    cfdf:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    cfe3:	48 89 75 d0          	mov    QWORD PTR [rbp-0x30],rsi
    cfe7:	64 48 8b 04 25 28 00 	mov    rax,QWORD PTR fs:0x28
    cfee:	00 00 
    cff0:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    cff4:	31 c0                	xor    eax,eax
    cff6:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    cffa:	48 8d 15 bb 34 00 00 	lea    rdx,[rip+0x34bb]        # 104bc <_IO_stdin_used+0x4bc>
    d001:	48 89 d6             	mov    rsi,rdx
    d004:	48 89 c7             	mov    rdi,rax
    d007:	e8 55 fe ff ff       	call   ce61 <need_fix>
    d00c:	48 89 c3             	mov    rbx,rax
    d00f:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    d013:	48 8d 15 a2 34 00 00 	lea    rdx,[rip+0x34a2]        # 104bc <_IO_stdin_used+0x4bc>
    d01a:	48 89 d6             	mov    rsi,rdx
    d01d:	48 89 c7             	mov    rdi,rax
    d020:	e8 3c fe ff ff       	call   ce61 <need_fix>
    d025:	b9 00 00 00 00       	mov    ecx,0x0
    d02a:	48 29 c3             	sub    rbx,rax
    d02d:	48 89 da             	mov    rdx,rbx
    d030:	71 05                	jno    d037 <cljn_sub+0x65>
    d032:	b9 01 00 00 00       	mov    ecx,0x1
    d037:	48 89 d0             	mov    rax,rdx
    d03a:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    d03e:	48 89 c8             	mov    rax,rcx
    d041:	83 e0 01             	and    eax,0x1
    d044:	84 c0                	test   al,al
    d046:	74 0f                	je     d057 <cljn_sub+0x85>
    d048:	48 8d 05 6f 34 00 00 	lea    rax,[rip+0x346f]        # 104be <_IO_stdin_used+0x4be>
    d04f:	48 89 c7             	mov    rdi,rax
    d052:	e8 37 9d ff ff       	call   6d8e <die>
    d057:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    d05b:	48 8d 15 5a 34 00 00 	lea    rdx,[rip+0x345a]        # 104bc <_IO_stdin_used+0x4bc>
    d062:	48 89 d6             	mov    rsi,rdx
    d065:	48 89 c7             	mov    rdi,rax
    d068:	e8 49 fe ff ff       	call   ceb6 <mk_fix_checked>
    d06d:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    d071:	64 48 2b 14 25 28 00 	sub    rdx,QWORD PTR fs:0x28
    d078:	00 00 
    d07a:	74 05                	je     d081 <cljn_sub+0xaf>
    d07c:	e8 df 3f ff ff       	call   1060 <__stack_chk_fail@plt>
    d081:	48 8b 5d f8          	mov    rbx,QWORD PTR [rbp-0x8]
    d085:	c9                   	leave
    d086:	c3                   	ret

000000000000d087 <cljn_mul>:
    d087:	f3 0f 1e fa          	endbr64
    d08b:	55                   	push   rbp
    d08c:	48 89 e5             	mov    rbp,rsp
    d08f:	53                   	push   rbx
    d090:	48 83 ec 28          	sub    rsp,0x28
    d094:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    d098:	48 89 75 d0          	mov    QWORD PTR [rbp-0x30],rsi
    d09c:	64 48 8b 04 25 28 00 	mov    rax,QWORD PTR fs:0x28
    d0a3:	00 00 
    d0a5:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    d0a9:	31 c0                	xor    eax,eax
    d0ab:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    d0af:	48 8d 15 16 34 00 00 	lea    rdx,[rip+0x3416]        # 104cc <_IO_stdin_used+0x4cc>
    d0b6:	48 89 d6             	mov    rsi,rdx
    d0b9:	48 89 c7             	mov    rdi,rax
    d0bc:	e8 a0 fd ff ff       	call   ce61 <need_fix>
    d0c1:	48 89 c3             	mov    rbx,rax
    d0c4:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    d0c8:	48 8d 15 fd 33 00 00 	lea    rdx,[rip+0x33fd]        # 104cc <_IO_stdin_used+0x4cc>
    d0cf:	48 89 d6             	mov    rsi,rdx
    d0d2:	48 89 c7             	mov    rdi,rax
    d0d5:	e8 87 fd ff ff       	call   ce61 <need_fix>
    d0da:	ba 00 00 00 00       	mov    edx,0x0
    d0df:	48 0f af c3          	imul   rax,rbx
    d0e3:	71 05                	jno    d0ea <cljn_mul+0x63>
    d0e5:	ba 01 00 00 00       	mov    edx,0x1
    d0ea:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    d0ee:	48 89 d0             	mov    rax,rdx
    d0f1:	83 e0 01             	and    eax,0x1
    d0f4:	84 c0                	test   al,al
    d0f6:	74 0f                	je     d107 <cljn_mul+0x80>
    d0f8:	48 8d 05 cf 33 00 00 	lea    rax,[rip+0x33cf]        # 104ce <_IO_stdin_used+0x4ce>
    d0ff:	48 89 c7             	mov    rdi,rax
    d102:	e8 87 9c ff ff       	call   6d8e <die>
    d107:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    d10b:	48 8d 15 ba 33 00 00 	lea    rdx,[rip+0x33ba]        # 104cc <_IO_stdin_used+0x4cc>
    d112:	48 89 d6             	mov    rsi,rdx
    d115:	48 89 c7             	mov    rdi,rax
    d118:	e8 99 fd ff ff       	call   ceb6 <mk_fix_checked>
    d11d:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    d121:	64 48 2b 14 25 28 00 	sub    rdx,QWORD PTR fs:0x28
    d128:	00 00 
    d12a:	74 05                	je     d131 <cljn_mul+0xaa>
    d12c:	e8 2f 3f ff ff       	call   1060 <__stack_chk_fail@plt>
    d131:	48 8b 5d f8          	mov    rbx,QWORD PTR [rbp-0x8]
    d135:	c9                   	leave
    d136:	c3                   	ret

000000000000d137 <cljn_quot>:
    d137:	f3 0f 1e fa          	endbr64
    d13b:	55                   	push   rbp
    d13c:	48 89 e5             	mov    rbp,rsp
    d13f:	48 83 ec 20          	sub    rsp,0x20
    d143:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    d147:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    d14b:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    d14f:	48 8d 15 86 33 00 00 	lea    rdx,[rip+0x3386]        # 104dc <_IO_stdin_used+0x4dc>
    d156:	48 89 d6             	mov    rsi,rdx
    d159:	48 89 c7             	mov    rdi,rax
    d15c:	e8 00 fd ff ff       	call   ce61 <need_fix>
    d161:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    d165:	48 83 7d f0 00       	cmp    QWORD PTR [rbp-0x10],0x0
    d16a:	75 0f                	jne    d17b <cljn_quot+0x44>
    d16c:	48 8d 05 6e 33 00 00 	lea    rax,[rip+0x336e]        # 104e1 <_IO_stdin_used+0x4e1>
    d173:	48 89 c7             	mov    rdi,rax
    d176:	e8 13 9c ff ff       	call   6d8e <die>
    d17b:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    d17f:	48 8d 15 56 33 00 00 	lea    rdx,[rip+0x3356]        # 104dc <_IO_stdin_used+0x4dc>
    d186:	48 89 d6             	mov    rsi,rdx
    d189:	48 89 c7             	mov    rdi,rax
    d18c:	e8 d0 fc ff ff       	call   ce61 <need_fix>
    d191:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    d195:	48 b8 00 00 00 00 00 	movabs rax,0xc000000000000000
    d19c:	00 00 c0 
    d19f:	48 39 45 f8          	cmp    QWORD PTR [rbp-0x8],rax
    d1a3:	75 16                	jne    d1bb <cljn_quot+0x84>
    d1a5:	48 83 7d f0 ff       	cmp    QWORD PTR [rbp-0x10],0xffffffffffffffff
    d1aa:	75 0f                	jne    d1bb <cljn_quot+0x84>
    d1ac:	48 8d 05 40 33 00 00 	lea    rax,[rip+0x3340]        # 104f3 <_IO_stdin_used+0x4f3>
    d1b3:	48 89 c7             	mov    rdi,rax
    d1b6:	e8 d3 9b ff ff       	call   6d8e <die>
    d1bb:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    d1bf:	48 99                	cqo
    d1c1:	48 f7 7d f0          	idiv   QWORD PTR [rbp-0x10]
    d1c5:	48 89 c2             	mov    rdx,rax
    d1c8:	48 8d 05 0d 33 00 00 	lea    rax,[rip+0x330d]        # 104dc <_IO_stdin_used+0x4dc>
    d1cf:	48 89 c6             	mov    rsi,rax
    d1d2:	48 89 d7             	mov    rdi,rdx
    d1d5:	e8 dc fc ff ff       	call   ceb6 <mk_fix_checked>
    d1da:	c9                   	leave
    d1db:	c3                   	ret

000000000000d1dc <cljn_mod>:
    d1dc:	f3 0f 1e fa          	endbr64
    d1e0:	55                   	push   rbp
    d1e1:	48 89 e5             	mov    rbp,rsp
    d1e4:	48 83 ec 30          	sub    rsp,0x30
    d1e8:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    d1ec:	48 89 75 d0          	mov    QWORD PTR [rbp-0x30],rsi
    d1f0:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    d1f4:	48 8d 15 09 33 00 00 	lea    rdx,[rip+0x3309]        # 10504 <_IO_stdin_used+0x504>
    d1fb:	48 89 d6             	mov    rsi,rdx
    d1fe:	48 89 c7             	mov    rdi,rax
    d201:	e8 5b fc ff ff       	call   ce61 <need_fix>
    d206:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    d20a:	48 83 7d f0 00       	cmp    QWORD PTR [rbp-0x10],0x0
    d20f:	75 0f                	jne    d220 <cljn_mod+0x44>
    d211:	48 8d 05 c9 32 00 00 	lea    rax,[rip+0x32c9]        # 104e1 <_IO_stdin_used+0x4e1>
    d218:	48 89 c7             	mov    rdi,rax
    d21b:	e8 6e 9b ff ff       	call   6d8e <die>
    d220:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    d224:	48 8d 15 d9 32 00 00 	lea    rdx,[rip+0x32d9]        # 10504 <_IO_stdin_used+0x504>
    d22b:	48 89 d6             	mov    rsi,rdx
    d22e:	48 89 c7             	mov    rdi,rax
    d231:	e8 2b fc ff ff       	call   ce61 <need_fix>
    d236:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    d23a:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    d23e:	48 99                	cqo
    d240:	48 f7 7d f0          	idiv   QWORD PTR [rbp-0x10]
    d244:	48 89 55 e8          	mov    QWORD PTR [rbp-0x18],rdx
    d248:	48 83 7d e8 00       	cmp    QWORD PTR [rbp-0x18],0x0
    d24d:	74 15                	je     d264 <cljn_mod+0x88>
    d24f:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    d253:	48 33 45 f0          	xor    rax,QWORD PTR [rbp-0x10]
    d257:	48 85 c0             	test   rax,rax
    d25a:	79 08                	jns    d264 <cljn_mod+0x88>
    d25c:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    d260:	48 01 45 e8          	add    QWORD PTR [rbp-0x18],rax
    d264:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    d268:	48 8d 15 95 32 00 00 	lea    rdx,[rip+0x3295]        # 10504 <_IO_stdin_used+0x504>
    d26f:	48 89 d6             	mov    rsi,rdx
    d272:	48 89 c7             	mov    rdi,rax
    d275:	e8 3c fc ff ff       	call   ceb6 <mk_fix_checked>
    d27a:	c9                   	leave
    d27b:	c3                   	ret

000000000000d27c <cljn_inc>:
    d27c:	f3 0f 1e fa          	endbr64
    d280:	55                   	push   rbp
    d281:	48 89 e5             	mov    rbp,rsp
    d284:	48 83 ec 20          	sub    rsp,0x20
    d288:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    d28c:	64 48 8b 04 25 28 00 	mov    rax,QWORD PTR fs:0x28
    d293:	00 00 
    d295:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    d299:	31 c0                	xor    eax,eax
    d29b:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    d29f:	48 8d 15 62 32 00 00 	lea    rdx,[rip+0x3262]        # 10508 <_IO_stdin_used+0x508>
    d2a6:	48 89 d6             	mov    rsi,rdx
    d2a9:	48 89 c7             	mov    rdi,rax
    d2ac:	e8 b0 fb ff ff       	call   ce61 <need_fix>
    d2b1:	ba 00 00 00 00       	mov    edx,0x0
    d2b6:	48 83 c0 01          	add    rax,0x1
    d2ba:	71 05                	jno    d2c1 <cljn_inc+0x45>
    d2bc:	ba 01 00 00 00       	mov    edx,0x1
    d2c1:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    d2c5:	48 89 d0             	mov    rax,rdx
    d2c8:	83 e0 01             	and    eax,0x1
    d2cb:	84 c0                	test   al,al
    d2cd:	74 0f                	je     d2de <cljn_inc+0x62>
    d2cf:	48 8d 05 36 32 00 00 	lea    rax,[rip+0x3236]        # 1050c <_IO_stdin_used+0x50c>
    d2d6:	48 89 c7             	mov    rdi,rax
    d2d9:	e8 b0 9a ff ff       	call   6d8e <die>
    d2de:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    d2e2:	48 8d 15 1f 32 00 00 	lea    rdx,[rip+0x321f]        # 10508 <_IO_stdin_used+0x508>
    d2e9:	48 89 d6             	mov    rsi,rdx
    d2ec:	48 89 c7             	mov    rdi,rax
    d2ef:	e8 c2 fb ff ff       	call   ceb6 <mk_fix_checked>
    d2f4:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    d2f8:	64 48 2b 14 25 28 00 	sub    rdx,QWORD PTR fs:0x28
    d2ff:	00 00 
    d301:	74 05                	je     d308 <cljn_inc+0x8c>
    d303:	e8 58 3d ff ff       	call   1060 <__stack_chk_fail@plt>
    d308:	c9                   	leave
    d309:	c3                   	ret

000000000000d30a <cljn_dec>:
    d30a:	f3 0f 1e fa          	endbr64
    d30e:	55                   	push   rbp
    d30f:	48 89 e5             	mov    rbp,rsp
    d312:	48 83 ec 20          	sub    rsp,0x20
    d316:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    d31a:	64 48 8b 04 25 28 00 	mov    rax,QWORD PTR fs:0x28
    d321:	00 00 
    d323:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    d327:	31 c0                	xor    eax,eax
    d329:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    d32d:	48 8d 15 e8 31 00 00 	lea    rdx,[rip+0x31e8]        # 1051c <_IO_stdin_used+0x51c>
    d334:	48 89 d6             	mov    rsi,rdx
    d337:	48 89 c7             	mov    rdi,rax
    d33a:	e8 22 fb ff ff       	call   ce61 <need_fix>
    d33f:	ba 00 00 00 00       	mov    edx,0x0
    d344:	48 83 e8 01          	sub    rax,0x1
    d348:	71 05                	jno    d34f <cljn_dec+0x45>
    d34a:	ba 01 00 00 00       	mov    edx,0x1
    d34f:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    d353:	48 89 d0             	mov    rax,rdx
    d356:	83 e0 01             	and    eax,0x1
    d359:	84 c0                	test   al,al
    d35b:	74 0f                	je     d36c <cljn_dec+0x62>
    d35d:	48 8d 05 bc 31 00 00 	lea    rax,[rip+0x31bc]        # 10520 <_IO_stdin_used+0x520>
    d364:	48 89 c7             	mov    rdi,rax
    d367:	e8 22 9a ff ff       	call   6d8e <die>
    d36c:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    d370:	48 8d 15 a5 31 00 00 	lea    rdx,[rip+0x31a5]        # 1051c <_IO_stdin_used+0x51c>
    d377:	48 89 d6             	mov    rsi,rdx
    d37a:	48 89 c7             	mov    rdi,rax
    d37d:	e8 34 fb ff ff       	call   ceb6 <mk_fix_checked>
    d382:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    d386:	64 48 2b 14 25 28 00 	sub    rdx,QWORD PTR fs:0x28
    d38d:	00 00 
    d38f:	74 05                	je     d396 <cljn_dec+0x8c>
    d391:	e8 ca 3c ff ff       	call   1060 <__stack_chk_fail@plt>
    d396:	c9                   	leave
    d397:	c3                   	ret

000000000000d398 <b2v>:
    d398:	f3 0f 1e fa          	endbr64
    d39c:	55                   	push   rbp
    d39d:	48 89 e5             	mov    rbp,rsp
    d3a0:	89 7d fc             	mov    DWORD PTR [rbp-0x4],edi
    d3a3:	83 7d fc 00          	cmp    DWORD PTR [rbp-0x4],0x0
    d3a7:	74 07                	je     d3b0 <b2v+0x18>
    d3a9:	b8 0a 00 00 00       	mov    eax,0xa
    d3ae:	eb 05                	jmp    d3b5 <b2v+0x1d>
    d3b0:	b8 06 00 00 00       	mov    eax,0x6
    d3b5:	5d                   	pop    rbp
    d3b6:	c3                   	ret

000000000000d3b7 <cljn_lt>:
    d3b7:	f3 0f 1e fa          	endbr64
    d3bb:	55                   	push   rbp
    d3bc:	48 89 e5             	mov    rbp,rsp
    d3bf:	53                   	push   rbx
    d3c0:	48 83 ec 18          	sub    rsp,0x18
    d3c4:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    d3c8:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    d3cc:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    d3d0:	48 8d 15 59 31 00 00 	lea    rdx,[rip+0x3159]        # 10530 <_IO_stdin_used+0x530>
    d3d7:	48 89 d6             	mov    rsi,rdx
    d3da:	48 89 c7             	mov    rdi,rax
    d3dd:	e8 7f fa ff ff       	call   ce61 <need_fix>
    d3e2:	48 89 c3             	mov    rbx,rax
    d3e5:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    d3e9:	48 8d 15 40 31 00 00 	lea    rdx,[rip+0x3140]        # 10530 <_IO_stdin_used+0x530>
    d3f0:	48 89 d6             	mov    rsi,rdx
    d3f3:	48 89 c7             	mov    rdi,rax
    d3f6:	e8 66 fa ff ff       	call   ce61 <need_fix>
    d3fb:	48 39 c3             	cmp    rbx,rax
    d3fe:	0f 9c c0             	setl   al
    d401:	0f b6 c0             	movzx  eax,al
    d404:	89 c7                	mov    edi,eax
    d406:	e8 8d ff ff ff       	call   d398 <b2v>
    d40b:	48 8b 5d f8          	mov    rbx,QWORD PTR [rbp-0x8]
    d40f:	c9                   	leave
    d410:	c3                   	ret

000000000000d411 <cljn_le>:
    d411:	f3 0f 1e fa          	endbr64
    d415:	55                   	push   rbp
    d416:	48 89 e5             	mov    rbp,rsp
    d419:	53                   	push   rbx
    d41a:	48 83 ec 18          	sub    rsp,0x18
    d41e:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    d422:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    d426:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    d42a:	48 8d 15 01 31 00 00 	lea    rdx,[rip+0x3101]        # 10532 <_IO_stdin_used+0x532>
    d431:	48 89 d6             	mov    rsi,rdx
    d434:	48 89 c7             	mov    rdi,rax
    d437:	e8 25 fa ff ff       	call   ce61 <need_fix>
    d43c:	48 89 c3             	mov    rbx,rax
    d43f:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    d443:	48 8d 15 e8 30 00 00 	lea    rdx,[rip+0x30e8]        # 10532 <_IO_stdin_used+0x532>
    d44a:	48 89 d6             	mov    rsi,rdx
    d44d:	48 89 c7             	mov    rdi,rax
    d450:	e8 0c fa ff ff       	call   ce61 <need_fix>
    d455:	48 39 c3             	cmp    rbx,rax
    d458:	0f 9e c0             	setle  al
    d45b:	0f b6 c0             	movzx  eax,al
    d45e:	89 c7                	mov    edi,eax
    d460:	e8 33 ff ff ff       	call   d398 <b2v>
    d465:	48 8b 5d f8          	mov    rbx,QWORD PTR [rbp-0x8]
    d469:	c9                   	leave
    d46a:	c3                   	ret

000000000000d46b <cljn_gt>:
    d46b:	f3 0f 1e fa          	endbr64
    d46f:	55                   	push   rbp
    d470:	48 89 e5             	mov    rbp,rsp
    d473:	53                   	push   rbx
    d474:	48 83 ec 18          	sub    rsp,0x18
    d478:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    d47c:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    d480:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    d484:	48 8d 15 aa 30 00 00 	lea    rdx,[rip+0x30aa]        # 10535 <_IO_stdin_used+0x535>
    d48b:	48 89 d6             	mov    rsi,rdx
    d48e:	48 89 c7             	mov    rdi,rax
    d491:	e8 cb f9 ff ff       	call   ce61 <need_fix>
    d496:	48 89 c3             	mov    rbx,rax
    d499:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    d49d:	48 8d 15 91 30 00 00 	lea    rdx,[rip+0x3091]        # 10535 <_IO_stdin_used+0x535>
    d4a4:	48 89 d6             	mov    rsi,rdx
    d4a7:	48 89 c7             	mov    rdi,rax
    d4aa:	e8 b2 f9 ff ff       	call   ce61 <need_fix>
    d4af:	48 39 c3             	cmp    rbx,rax
    d4b2:	0f 9f c0             	setg   al
    d4b5:	0f b6 c0             	movzx  eax,al
    d4b8:	89 c7                	mov    edi,eax
    d4ba:	e8 d9 fe ff ff       	call   d398 <b2v>
    d4bf:	48 8b 5d f8          	mov    rbx,QWORD PTR [rbp-0x8]
    d4c3:	c9                   	leave
    d4c4:	c3                   	ret

000000000000d4c5 <cljn_ge>:
    d4c5:	f3 0f 1e fa          	endbr64
    d4c9:	55                   	push   rbp
    d4ca:	48 89 e5             	mov    rbp,rsp
    d4cd:	53                   	push   rbx
    d4ce:	48 83 ec 18          	sub    rsp,0x18
    d4d2:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    d4d6:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    d4da:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    d4de:	48 8d 15 52 30 00 00 	lea    rdx,[rip+0x3052]        # 10537 <_IO_stdin_used+0x537>
    d4e5:	48 89 d6             	mov    rsi,rdx
    d4e8:	48 89 c7             	mov    rdi,rax
    d4eb:	e8 71 f9 ff ff       	call   ce61 <need_fix>
    d4f0:	48 89 c3             	mov    rbx,rax
    d4f3:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    d4f7:	48 8d 15 39 30 00 00 	lea    rdx,[rip+0x3039]        # 10537 <_IO_stdin_used+0x537>
    d4fe:	48 89 d6             	mov    rsi,rdx
    d501:	48 89 c7             	mov    rdi,rax
    d504:	e8 58 f9 ff ff       	call   ce61 <need_fix>
    d509:	48 39 c3             	cmp    rbx,rax
    d50c:	0f 9d c0             	setge  al
    d50f:	0f b6 c0             	movzx  eax,al
    d512:	89 c7                	mov    edi,eax
    d514:	e8 7f fe ff ff       	call   d398 <b2v>
    d519:	48 8b 5d f8          	mov    rbx,QWORD PTR [rbp-0x8]
    d51d:	c9                   	leave
    d51e:	c3                   	ret

000000000000d51f <is_seq>:
    d51f:	f3 0f 1e fa          	endbr64
    d523:	55                   	push   rbp
    d524:	48 89 e5             	mov    rbp,rsp
    d527:	89 7d fc             	mov    DWORD PTR [rbp-0x4],edi
    d52a:	83 7d fc 02          	cmp    DWORD PTR [rbp-0x4],0x2
    d52e:	74 06                	je     d536 <is_seq+0x17>
    d530:	83 7d fc 05          	cmp    DWORD PTR [rbp-0x4],0x5
    d534:	75 07                	jne    d53d <is_seq+0x1e>
    d536:	b8 01 00 00 00       	mov    eax,0x1
    d53b:	eb 05                	jmp    d542 <is_seq+0x23>
    d53d:	b8 00 00 00 00       	mov    eax,0x0
    d542:	5d                   	pop    rbp
    d543:	c3                   	ret

000000000000d544 <seq_nth>:
    d544:	f3 0f 1e fa          	endbr64
    d548:	55                   	push   rbp
    d549:	48 89 e5             	mov    rbp,rsp
    d54c:	48 83 ec 28          	sub    rsp,0x28
    d550:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    d554:	89 75 e4             	mov    DWORD PTR [rbp-0x1c],esi
    d557:	48 89 55 d8          	mov    QWORD PTR [rbp-0x28],rdx
    d55b:	83 7d e4 05          	cmp    DWORD PTR [rbp-0x1c],0x5
    d55f:	75 15                	jne    d576 <seq_nth+0x32>
    d561:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    d565:	48 8b 55 d8          	mov    rdx,QWORD PTR [rbp-0x28]
    d569:	48 89 d6             	mov    rsi,rdx
    d56c:	48 89 c7             	mov    rdi,rax
    d56f:	e8 6b aa ff ff       	call   7fdf <pv_nth>
    d574:	eb 2f                	jmp    d5a5 <seq_nth+0x61>
    d576:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    d57a:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    d57e:	eb 0c                	jmp    d58c <seq_nth+0x48>
    d580:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    d584:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    d588:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    d58c:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    d590:	48 8d 50 ff          	lea    rdx,[rax-0x1]
    d594:	48 89 55 d8          	mov    QWORD PTR [rbp-0x28],rdx
    d598:	48 85 c0             	test   rax,rax
    d59b:	7f e3                	jg     d580 <seq_nth+0x3c>
    d59d:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    d5a1:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    d5a5:	c9                   	leave
    d5a6:	c3                   	ret

000000000000d5a7 <seq_len>:
    d5a7:	f3 0f 1e fa          	endbr64
    d5ab:	55                   	push   rbp
    d5ac:	48 89 e5             	mov    rbp,rsp
    d5af:	48 83 ec 20          	sub    rsp,0x20
    d5b3:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    d5b7:	89 75 e4             	mov    DWORD PTR [rbp-0x1c],esi
    d5ba:	83 7d e4 05          	cmp    DWORD PTR [rbp-0x1c],0x5
    d5be:	75 0a                	jne    d5ca <seq_len+0x23>
    d5c0:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    d5c4:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    d5c8:	eb 38                	jmp    d602 <seq_len+0x5b>
    d5ca:	48 c7 45 f0 00 00 00 	mov    QWORD PTR [rbp-0x10],0x0
    d5d1:	00 
    d5d2:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    d5d6:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    d5da:	eb 11                	jmp    d5ed <seq_len+0x46>
    d5dc:	48 83 45 f0 01       	add    QWORD PTR [rbp-0x10],0x1
    d5e1:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    d5e5:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    d5e9:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    d5ed:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    d5f1:	48 89 c7             	mov    rdi,rax
    d5f4:	e8 d1 97 ff ff       	call   6dca <obj_type>
    d5f9:	83 f8 02             	cmp    eax,0x2
    d5fc:	74 de                	je     d5dc <seq_len+0x35>
    d5fe:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    d602:	c9                   	leave
    d603:	c3                   	ret

000000000000d604 <cljn_equal_raw>:
    d604:	f3 0f 1e fa          	endbr64
    d608:	55                   	push   rbp
    d609:	48 89 e5             	mov    rbp,rsp
    d60c:	53                   	push   rbx
    d60d:	48 81 ec b8 00 00 00 	sub    rsp,0xb8
    d614:	48 89 bd 48 ff ff ff 	mov    QWORD PTR [rbp-0xb8],rdi
    d61b:	48 89 b5 40 ff ff ff 	mov    QWORD PTR [rbp-0xc0],rsi
    d622:	48 8b 85 48 ff ff ff 	mov    rax,QWORD PTR [rbp-0xb8]
    d629:	48 3b 85 40 ff ff ff 	cmp    rax,QWORD PTR [rbp-0xc0]
    d630:	75 0a                	jne    d63c <cljn_equal_raw+0x38>
    d632:	b8 01 00 00 00       	mov    eax,0x1
    d637:	e9 0d 06 00 00       	jmp    dc49 <cljn_equal_raw+0x645>
    d63c:	48 8b 85 48 ff ff ff 	mov    rax,QWORD PTR [rbp-0xb8]
    d643:	48 89 c7             	mov    rdi,rax
    d646:	e8 7f 97 ff ff       	call   6dca <obj_type>
    d64b:	89 85 58 ff ff ff    	mov    DWORD PTR [rbp-0xa8],eax
    d651:	48 8b 85 40 ff ff ff 	mov    rax,QWORD PTR [rbp-0xc0]
    d658:	48 89 c7             	mov    rdi,rax
    d65b:	e8 6a 97 ff ff       	call   6dca <obj_type>
    d660:	89 85 5c ff ff ff    	mov    DWORD PTR [rbp-0xa4],eax
    d666:	83 bd 58 ff ff ff 01 	cmp    DWORD PTR [rbp-0xa8],0x1
    d66d:	75 09                	jne    d678 <cljn_equal_raw+0x74>
    d66f:	83 bd 5c ff ff ff 01 	cmp    DWORD PTR [rbp-0xa4],0x1
    d676:	74 12                	je     d68a <cljn_equal_raw+0x86>
    d678:	83 bd 58 ff ff ff 04 	cmp    DWORD PTR [rbp-0xa8],0x4
    d67f:	75 7d                	jne    d6fe <cljn_equal_raw+0xfa>
    d681:	83 bd 5c ff ff ff 04 	cmp    DWORD PTR [rbp-0xa4],0x4
    d688:	75 74                	jne    d6fe <cljn_equal_raw+0xfa>
    d68a:	48 8b 85 48 ff ff ff 	mov    rax,QWORD PTR [rbp-0xb8]
    d691:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    d695:	48 8b 85 40 ff ff ff 	mov    rax,QWORD PTR [rbp-0xc0]
    d69c:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    d6a0:	8b 85 58 ff ff ff    	mov    eax,DWORD PTR [rbp-0xa8]
    d6a6:	3b 85 5c ff ff ff    	cmp    eax,DWORD PTR [rbp-0xa4]
    d6ac:	75 46                	jne    d6f4 <cljn_equal_raw+0xf0>
    d6ae:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    d6b2:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    d6b6:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    d6ba:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    d6be:	48 39 c2             	cmp    rdx,rax
    d6c1:	75 31                	jne    d6f4 <cljn_equal_raw+0xf0>
    d6c3:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    d6c7:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    d6cb:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    d6cf:	48 8b 48 18          	mov    rcx,QWORD PTR [rax+0x18]
    d6d3:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    d6d7:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    d6db:	48 89 ce             	mov    rsi,rcx
    d6de:	48 89 c7             	mov    rdi,rax
    d6e1:	e8 aa 39 ff ff       	call   1090 <memcmp@plt>
    d6e6:	85 c0                	test   eax,eax
    d6e8:	75 0a                	jne    d6f4 <cljn_equal_raw+0xf0>
    d6ea:	b8 01 00 00 00       	mov    eax,0x1
    d6ef:	e9 55 05 00 00       	jmp    dc49 <cljn_equal_raw+0x645>
    d6f4:	b8 00 00 00 00       	mov    eax,0x0
    d6f9:	e9 4b 05 00 00       	jmp    dc49 <cljn_equal_raw+0x645>
    d6fe:	8b 85 58 ff ff ff    	mov    eax,DWORD PTR [rbp-0xa8]
    d704:	89 c7                	mov    edi,eax
    d706:	e8 14 fe ff ff       	call   d51f <is_seq>
    d70b:	85 c0                	test   eax,eax
    d70d:	0f 84 e3 00 00 00    	je     d7f6 <cljn_equal_raw+0x1f2>
    d713:	8b 85 5c ff ff ff    	mov    eax,DWORD PTR [rbp-0xa4]
    d719:	89 c7                	mov    edi,eax
    d71b:	e8 ff fd ff ff       	call   d51f <is_seq>
    d720:	85 c0                	test   eax,eax
    d722:	0f 84 ce 00 00 00    	je     d7f6 <cljn_equal_raw+0x1f2>
    d728:	8b 95 58 ff ff ff    	mov    edx,DWORD PTR [rbp-0xa8]
    d72e:	48 8b 85 48 ff ff ff 	mov    rax,QWORD PTR [rbp-0xb8]
    d735:	89 d6                	mov    esi,edx
    d737:	48 89 c7             	mov    rdi,rax
    d73a:	e8 68 fe ff ff       	call   d5a7 <seq_len>
    d73f:	48 89 45 88          	mov    QWORD PTR [rbp-0x78],rax
    d743:	8b 95 5c ff ff ff    	mov    edx,DWORD PTR [rbp-0xa4]
    d749:	48 8b 85 40 ff ff ff 	mov    rax,QWORD PTR [rbp-0xc0]
    d750:	89 d6                	mov    esi,edx
    d752:	48 89 c7             	mov    rdi,rax
    d755:	e8 4d fe ff ff       	call   d5a7 <seq_len>
    d75a:	48 89 45 90          	mov    QWORD PTR [rbp-0x70],rax
    d75e:	48 8b 45 88          	mov    rax,QWORD PTR [rbp-0x78]
    d762:	48 3b 45 90          	cmp    rax,QWORD PTR [rbp-0x70]
    d766:	74 0a                	je     d772 <cljn_equal_raw+0x16e>
    d768:	b8 00 00 00 00       	mov    eax,0x0
    d76d:	e9 d7 04 00 00       	jmp    dc49 <cljn_equal_raw+0x645>
    d772:	48 c7 85 70 ff ff ff 	mov    QWORD PTR [rbp-0x90],0x0
    d779:	00 00 00 00 
    d77d:	eb 60                	jmp    d7df <cljn_equal_raw+0x1db>
    d77f:	48 8b 95 70 ff ff ff 	mov    rdx,QWORD PTR [rbp-0x90]
    d786:	8b 8d 5c ff ff ff    	mov    ecx,DWORD PTR [rbp-0xa4]
    d78c:	48 8b 85 40 ff ff ff 	mov    rax,QWORD PTR [rbp-0xc0]
    d793:	89 ce                	mov    esi,ecx
    d795:	48 89 c7             	mov    rdi,rax
    d798:	e8 a7 fd ff ff       	call   d544 <seq_nth>
    d79d:	48 89 c3             	mov    rbx,rax
    d7a0:	48 8b 95 70 ff ff ff 	mov    rdx,QWORD PTR [rbp-0x90]
    d7a7:	8b 8d 58 ff ff ff    	mov    ecx,DWORD PTR [rbp-0xa8]
    d7ad:	48 8b 85 48 ff ff ff 	mov    rax,QWORD PTR [rbp-0xb8]
    d7b4:	89 ce                	mov    esi,ecx
    d7b6:	48 89 c7             	mov    rdi,rax
    d7b9:	e8 86 fd ff ff       	call   d544 <seq_nth>
    d7be:	48 89 de             	mov    rsi,rbx
    d7c1:	48 89 c7             	mov    rdi,rax
    d7c4:	e8 3b fe ff ff       	call   d604 <cljn_equal_raw>
    d7c9:	85 c0                	test   eax,eax
    d7cb:	75 0a                	jne    d7d7 <cljn_equal_raw+0x1d3>
    d7cd:	b8 00 00 00 00       	mov    eax,0x0
    d7d2:	e9 72 04 00 00       	jmp    dc49 <cljn_equal_raw+0x645>
    d7d7:	48 83 85 70 ff ff ff 	add    QWORD PTR [rbp-0x90],0x1
    d7de:	01 
    d7df:	48 8b 85 70 ff ff ff 	mov    rax,QWORD PTR [rbp-0x90]
    d7e6:	48 3b 45 88          	cmp    rax,QWORD PTR [rbp-0x78]
    d7ea:	7c 93                	jl     d77f <cljn_equal_raw+0x17b>
    d7ec:	b8 01 00 00 00       	mov    eax,0x1
    d7f1:	e9 53 04 00 00       	jmp    dc49 <cljn_equal_raw+0x645>
    d7f6:	83 bd 58 ff ff ff 07 	cmp    DWORD PTR [rbp-0xa8],0x7
    d7fd:	74 12                	je     d811 <cljn_equal_raw+0x20d>
    d7ff:	83 bd 58 ff ff ff 0d 	cmp    DWORD PTR [rbp-0xa8],0xd
    d806:	74 09                	je     d811 <cljn_equal_raw+0x20d>
    d808:	83 bd 58 ff ff ff 10 	cmp    DWORD PTR [rbp-0xa8],0x10
    d80f:	75 07                	jne    d818 <cljn_equal_raw+0x214>
    d811:	b8 01 00 00 00       	mov    eax,0x1
    d816:	eb 05                	jmp    d81d <cljn_equal_raw+0x219>
    d818:	b8 00 00 00 00       	mov    eax,0x0
    d81d:	89 85 60 ff ff ff    	mov    DWORD PTR [rbp-0xa0],eax
    d823:	83 bd 5c ff ff ff 07 	cmp    DWORD PTR [rbp-0xa4],0x7
    d82a:	74 12                	je     d83e <cljn_equal_raw+0x23a>
    d82c:	83 bd 5c ff ff ff 0d 	cmp    DWORD PTR [rbp-0xa4],0xd
    d833:	74 09                	je     d83e <cljn_equal_raw+0x23a>
    d835:	83 bd 5c ff ff ff 10 	cmp    DWORD PTR [rbp-0xa4],0x10
    d83c:	75 07                	jne    d845 <cljn_equal_raw+0x241>
    d83e:	b8 01 00 00 00       	mov    eax,0x1
    d843:	eb 05                	jmp    d84a <cljn_equal_raw+0x246>
    d845:	b8 00 00 00 00       	mov    eax,0x0
    d84a:	89 85 64 ff ff ff    	mov    DWORD PTR [rbp-0x9c],eax
    d850:	83 bd 60 ff ff ff 00 	cmp    DWORD PTR [rbp-0xa0],0x0
    d857:	0f 84 64 01 00 00    	je     d9c1 <cljn_equal_raw+0x3bd>
    d85d:	83 bd 64 ff ff ff 00 	cmp    DWORD PTR [rbp-0x9c],0x0
    d864:	0f 84 57 01 00 00    	je     d9c1 <cljn_equal_raw+0x3bd>
    d86a:	83 bd 58 ff ff ff 0d 	cmp    DWORD PTR [rbp-0xa8],0xd
    d871:	75 0d                	jne    d880 <cljn_equal_raw+0x27c>
    d873:	48 8b 85 48 ff ff ff 	mov    rax,QWORD PTR [rbp-0xb8]
    d87a:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    d87e:	eb 21                	jmp    d8a1 <cljn_equal_raw+0x29d>
    d880:	83 bd 58 ff ff ff 10 	cmp    DWORD PTR [rbp-0xa8],0x10
    d887:	75 0d                	jne    d896 <cljn_equal_raw+0x292>
    d889:	48 8b 85 48 ff ff ff 	mov    rax,QWORD PTR [rbp-0xb8]
    d890:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    d894:	eb 0b                	jmp    d8a1 <cljn_equal_raw+0x29d>
    d896:	48 8b 85 48 ff ff ff 	mov    rax,QWORD PTR [rbp-0xb8]
    d89d:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    d8a1:	48 89 45 98          	mov    QWORD PTR [rbp-0x68],rax
    d8a5:	83 bd 5c ff ff ff 0d 	cmp    DWORD PTR [rbp-0xa4],0xd
    d8ac:	75 0d                	jne    d8bb <cljn_equal_raw+0x2b7>
    d8ae:	48 8b 85 40 ff ff ff 	mov    rax,QWORD PTR [rbp-0xc0]
    d8b5:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    d8b9:	eb 21                	jmp    d8dc <cljn_equal_raw+0x2d8>
    d8bb:	83 bd 5c ff ff ff 10 	cmp    DWORD PTR [rbp-0xa4],0x10
    d8c2:	75 0d                	jne    d8d1 <cljn_equal_raw+0x2cd>
    d8c4:	48 8b 85 40 ff ff ff 	mov    rax,QWORD PTR [rbp-0xc0]
    d8cb:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    d8cf:	eb 0b                	jmp    d8dc <cljn_equal_raw+0x2d8>
    d8d1:	48 8b 85 40 ff ff ff 	mov    rax,QWORD PTR [rbp-0xc0]
    d8d8:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    d8dc:	48 89 45 a0          	mov    QWORD PTR [rbp-0x60],rax
    d8e0:	48 8b 45 98          	mov    rax,QWORD PTR [rbp-0x68]
    d8e4:	48 3b 45 a0          	cmp    rax,QWORD PTR [rbp-0x60]
    d8e8:	74 0a                	je     d8f4 <cljn_equal_raw+0x2f0>
    d8ea:	b8 00 00 00 00       	mov    eax,0x0
    d8ef:	e9 55 03 00 00       	jmp    dc49 <cljn_equal_raw+0x645>
    d8f4:	83 bd 58 ff ff ff 0d 	cmp    DWORD PTR [rbp-0xa8],0xd
    d8fb:	75 22                	jne    d91f <cljn_equal_raw+0x31b>
    d8fd:	48 8b 85 48 ff ff ff 	mov    rax,QWORD PTR [rbp-0xb8]
    d904:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    d908:	48 8b 95 40 ff ff ff 	mov    rdx,QWORD PTR [rbp-0xc0]
    d90f:	48 89 d6             	mov    rsi,rdx
    d912:	48 89 c7             	mov    rdi,rax
    d915:	e8 14 bf ff ff       	call   982e <hnode_all_in>
    d91a:	e9 2a 03 00 00       	jmp    dc49 <cljn_equal_raw+0x645>
    d91f:	83 bd 58 ff ff ff 10 	cmp    DWORD PTR [rbp-0xa8],0x10
    d926:	75 22                	jne    d94a <cljn_equal_raw+0x346>
    d928:	48 8b 85 48 ff ff ff 	mov    rax,QWORD PTR [rbp-0xb8]
    d92f:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    d933:	48 8b 95 40 ff ff ff 	mov    rdx,QWORD PTR [rbp-0xc0]
    d93a:	48 89 d6             	mov    rsi,rdx
    d93d:	48 89 c7             	mov    rdi,rax
    d940:	e8 d6 dd ff ff       	call   b71b <tn_all_in>
    d945:	e9 ff 02 00 00       	jmp    dc49 <cljn_equal_raw+0x645>
    d94a:	48 8b 85 48 ff ff ff 	mov    rax,QWORD PTR [rbp-0xb8]
    d951:	48 89 45 a8          	mov    QWORD PTR [rbp-0x58],rax
    d955:	48 c7 85 78 ff ff ff 	mov    QWORD PTR [rbp-0x88],0x0
    d95c:	00 00 00 00 
    d960:	eb 44                	jmp    d9a6 <cljn_equal_raw+0x3a2>
    d962:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    d966:	48 8b 95 78 ff ff ff 	mov    rdx,QWORD PTR [rbp-0x88]
    d96d:	48 83 c2 02          	add    rdx,0x2
    d971:	48 8b 54 d0 08       	mov    rdx,QWORD PTR [rax+rdx*8+0x8]
    d976:	48 8b 85 40 ff ff ff 	mov    rax,QWORD PTR [rbp-0xc0]
    d97d:	48 89 d6             	mov    rsi,rdx
    d980:	48 89 c7             	mov    rdi,rax
    d983:	e8 fe e5 ff ff       	call   bf86 <cljn_contains>
    d988:	48 89 c7             	mov    rdi,rax
    d98b:	e8 ef 02 00 00       	call   dc7f <cljn_truthy>
    d990:	85 c0                	test   eax,eax
    d992:	75 0a                	jne    d99e <cljn_equal_raw+0x39a>
    d994:	b8 00 00 00 00       	mov    eax,0x0
    d999:	e9 ab 02 00 00       	jmp    dc49 <cljn_equal_raw+0x645>
    d99e:	48 83 85 78 ff ff ff 	add    QWORD PTR [rbp-0x88],0x1
    d9a5:	01 
    d9a6:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    d9aa:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    d9ae:	48 39 85 78 ff ff ff 	cmp    QWORD PTR [rbp-0x88],rax
    d9b5:	7c ab                	jl     d962 <cljn_equal_raw+0x35e>
    d9b7:	b8 01 00 00 00       	mov    eax,0x1
    d9bc:	e9 88 02 00 00       	jmp    dc49 <cljn_equal_raw+0x645>
    d9c1:	83 bd 58 ff ff ff 06 	cmp    DWORD PTR [rbp-0xa8],0x6
    d9c8:	74 12                	je     d9dc <cljn_equal_raw+0x3d8>
    d9ca:	83 bd 58 ff ff ff 0a 	cmp    DWORD PTR [rbp-0xa8],0xa
    d9d1:	74 09                	je     d9dc <cljn_equal_raw+0x3d8>
    d9d3:	83 bd 58 ff ff ff 0f 	cmp    DWORD PTR [rbp-0xa8],0xf
    d9da:	75 07                	jne    d9e3 <cljn_equal_raw+0x3df>
    d9dc:	b8 01 00 00 00       	mov    eax,0x1
    d9e1:	eb 05                	jmp    d9e8 <cljn_equal_raw+0x3e4>
    d9e3:	b8 00 00 00 00       	mov    eax,0x0
    d9e8:	89 85 68 ff ff ff    	mov    DWORD PTR [rbp-0x98],eax
    d9ee:	83 bd 5c ff ff ff 06 	cmp    DWORD PTR [rbp-0xa4],0x6
    d9f5:	74 12                	je     da09 <cljn_equal_raw+0x405>
    d9f7:	83 bd 5c ff ff ff 0a 	cmp    DWORD PTR [rbp-0xa4],0xa
    d9fe:	74 09                	je     da09 <cljn_equal_raw+0x405>
    da00:	83 bd 5c ff ff ff 0f 	cmp    DWORD PTR [rbp-0xa4],0xf
    da07:	75 07                	jne    da10 <cljn_equal_raw+0x40c>
    da09:	b8 01 00 00 00       	mov    eax,0x1
    da0e:	eb 05                	jmp    da15 <cljn_equal_raw+0x411>
    da10:	b8 00 00 00 00       	mov    eax,0x0
    da15:	89 85 6c ff ff ff    	mov    DWORD PTR [rbp-0x94],eax
    da1b:	83 bd 68 ff ff ff 00 	cmp    DWORD PTR [rbp-0x98],0x0
    da22:	0f 84 a8 01 00 00    	je     dbd0 <cljn_equal_raw+0x5cc>
    da28:	83 bd 6c ff ff ff 00 	cmp    DWORD PTR [rbp-0x94],0x0
    da2f:	0f 84 9b 01 00 00    	je     dbd0 <cljn_equal_raw+0x5cc>
    da35:	83 bd 58 ff ff ff 0a 	cmp    DWORD PTR [rbp-0xa8],0xa
    da3c:	75 0d                	jne    da4b <cljn_equal_raw+0x447>
    da3e:	48 8b 85 48 ff ff ff 	mov    rax,QWORD PTR [rbp-0xb8]
    da45:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    da49:	eb 21                	jmp    da6c <cljn_equal_raw+0x468>
    da4b:	83 bd 58 ff ff ff 0f 	cmp    DWORD PTR [rbp-0xa8],0xf
    da52:	75 0d                	jne    da61 <cljn_equal_raw+0x45d>
    da54:	48 8b 85 48 ff ff ff 	mov    rax,QWORD PTR [rbp-0xb8]
    da5b:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    da5f:	eb 0b                	jmp    da6c <cljn_equal_raw+0x468>
    da61:	48 8b 85 48 ff ff ff 	mov    rax,QWORD PTR [rbp-0xb8]
    da68:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    da6c:	48 89 45 b0          	mov    QWORD PTR [rbp-0x50],rax
    da70:	83 bd 5c ff ff ff 0a 	cmp    DWORD PTR [rbp-0xa4],0xa
    da77:	75 0d                	jne    da86 <cljn_equal_raw+0x482>
    da79:	48 8b 85 40 ff ff ff 	mov    rax,QWORD PTR [rbp-0xc0]
    da80:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    da84:	eb 21                	jmp    daa7 <cljn_equal_raw+0x4a3>
    da86:	83 bd 5c ff ff ff 0f 	cmp    DWORD PTR [rbp-0xa4],0xf
    da8d:	75 0d                	jne    da9c <cljn_equal_raw+0x498>
    da8f:	48 8b 85 40 ff ff ff 	mov    rax,QWORD PTR [rbp-0xc0]
    da96:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    da9a:	eb 0b                	jmp    daa7 <cljn_equal_raw+0x4a3>
    da9c:	48 8b 85 40 ff ff ff 	mov    rax,QWORD PTR [rbp-0xc0]
    daa3:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    daa7:	48 89 45 b8          	mov    QWORD PTR [rbp-0x48],rax
    daab:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    daaf:	48 3b 45 b8          	cmp    rax,QWORD PTR [rbp-0x48]
    dab3:	74 0a                	je     dabf <cljn_equal_raw+0x4bb>
    dab5:	b8 00 00 00 00       	mov    eax,0x0
    daba:	e9 8a 01 00 00       	jmp    dc49 <cljn_equal_raw+0x645>
    dabf:	83 bd 58 ff ff ff 0a 	cmp    DWORD PTR [rbp-0xa8],0xa
    dac6:	75 22                	jne    daea <cljn_equal_raw+0x4e6>
    dac8:	48 8b 85 48 ff ff ff 	mov    rax,QWORD PTR [rbp-0xb8]
    dacf:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    dad3:	48 8b 95 40 ff ff ff 	mov    rdx,QWORD PTR [rbp-0xc0]
    dada:	48 89 d6             	mov    rsi,rdx
    dadd:	48 89 c7             	mov    rdi,rax
    dae0:	e8 18 cd ff ff       	call   a7fd <hmap_node_subset>
    dae5:	e9 5f 01 00 00       	jmp    dc49 <cljn_equal_raw+0x645>
    daea:	83 bd 58 ff ff ff 0f 	cmp    DWORD PTR [rbp-0xa8],0xf
    daf1:	75 22                	jne    db15 <cljn_equal_raw+0x511>
    daf3:	48 8b 85 48 ff ff ff 	mov    rax,QWORD PTR [rbp-0xb8]
    dafa:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    dafe:	48 8b 95 40 ff ff ff 	mov    rdx,QWORD PTR [rbp-0xc0]
    db05:	48 89 d6             	mov    rsi,rdx
    db08:	48 89 c7             	mov    rdi,rax
    db0b:	e8 9a dc ff ff       	call   b7aa <tn_map_subset>
    db10:	e9 34 01 00 00       	jmp    dc49 <cljn_equal_raw+0x645>
    db15:	48 8b 85 48 ff ff ff 	mov    rax,QWORD PTR [rbp-0xb8]
    db1c:	48 89 45 c0          	mov    QWORD PTR [rbp-0x40],rax
    db20:	48 c7 45 80 00 00 00 	mov    QWORD PTR [rbp-0x80],0x0
    db27:	00 
    db28:	e9 8a 00 00 00       	jmp    dbb7 <cljn_equal_raw+0x5b3>
    db2d:	48 8b 45 80          	mov    rax,QWORD PTR [rbp-0x80]
    db31:	48 8d 14 00          	lea    rdx,[rax+rax*1]
    db35:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    db39:	48 83 c2 02          	add    rdx,0x2
    db3d:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    db42:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
    db46:	48 8b 55 c8          	mov    rdx,QWORD PTR [rbp-0x38]
    db4a:	48 8b 85 40 ff ff ff 	mov    rax,QWORD PTR [rbp-0xc0]
    db51:	48 89 d6             	mov    rsi,rdx
    db54:	48 89 c7             	mov    rdi,rax
    db57:	e8 0c c4 ff ff       	call   9f68 <cljn_map_contains>
    db5c:	48 89 c7             	mov    rdi,rax
    db5f:	e8 1b 01 00 00       	call   dc7f <cljn_truthy>
    db64:	85 c0                	test   eax,eax
    db66:	74 40                	je     dba8 <cljn_equal_raw+0x5a4>
    db68:	48 8b 55 c8          	mov    rdx,QWORD PTR [rbp-0x38]
    db6c:	48 8b 85 40 ff ff ff 	mov    rax,QWORD PTR [rbp-0xc0]
    db73:	48 89 d6             	mov    rsi,rdx
    db76:	48 89 c7             	mov    rdi,rax
    db79:	e8 f6 c2 ff ff       	call   9e74 <cljn_map_get>
    db7e:	48 89 c2             	mov    rdx,rax
    db81:	48 8b 45 80          	mov    rax,QWORD PTR [rbp-0x80]
    db85:	48 01 c0             	add    rax,rax
    db88:	48 8d 48 01          	lea    rcx,[rax+0x1]
    db8c:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    db90:	48 83 c1 02          	add    rcx,0x2
    db94:	48 8b 44 c8 08       	mov    rax,QWORD PTR [rax+rcx*8+0x8]
    db99:	48 89 d6             	mov    rsi,rdx
    db9c:	48 89 c7             	mov    rdi,rax
    db9f:	e8 60 fa ff ff       	call   d604 <cljn_equal_raw>
    dba4:	85 c0                	test   eax,eax
    dba6:	75 0a                	jne    dbb2 <cljn_equal_raw+0x5ae>
    dba8:	b8 00 00 00 00       	mov    eax,0x0
    dbad:	e9 97 00 00 00       	jmp    dc49 <cljn_equal_raw+0x645>
    dbb2:	48 83 45 80 01       	add    QWORD PTR [rbp-0x80],0x1
    dbb7:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    dbbb:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    dbbf:	48 39 45 80          	cmp    QWORD PTR [rbp-0x80],rax
    dbc3:	0f 8c 64 ff ff ff    	jl     db2d <cljn_equal_raw+0x529>
    dbc9:	b8 01 00 00 00       	mov    eax,0x1
    dbce:	eb 79                	jmp    dc49 <cljn_equal_raw+0x645>
    dbd0:	83 bd 58 ff ff ff 08 	cmp    DWORD PTR [rbp-0xa8],0x8
    dbd7:	75 6b                	jne    dc44 <cljn_equal_raw+0x640>
    dbd9:	83 bd 5c ff ff ff 08 	cmp    DWORD PTR [rbp-0xa4],0x8
    dbe0:	75 62                	jne    dc44 <cljn_equal_raw+0x640>
    dbe2:	48 8b 85 48 ff ff ff 	mov    rax,QWORD PTR [rbp-0xb8]
    dbe9:	48 89 45 d0          	mov    QWORD PTR [rbp-0x30],rax
    dbed:	48 8b 85 40 ff ff ff 	mov    rax,QWORD PTR [rbp-0xc0]
    dbf4:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    dbf8:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    dbfc:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    dc00:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    dc04:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    dc08:	48 89 d6             	mov    rsi,rdx
    dc0b:	48 89 c7             	mov    rdi,rax
    dc0e:	e8 f1 f9 ff ff       	call   d604 <cljn_equal_raw>
    dc13:	85 c0                	test   eax,eax
    dc15:	74 26                	je     dc3d <cljn_equal_raw+0x639>
    dc17:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    dc1b:	48 8b 50 18          	mov    rdx,QWORD PTR [rax+0x18]
    dc1f:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    dc23:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    dc27:	48 89 d6             	mov    rsi,rdx
    dc2a:	48 89 c7             	mov    rdi,rax
    dc2d:	e8 d2 f9 ff ff       	call   d604 <cljn_equal_raw>
    dc32:	85 c0                	test   eax,eax
    dc34:	74 07                	je     dc3d <cljn_equal_raw+0x639>
    dc36:	b8 01 00 00 00       	mov    eax,0x1
    dc3b:	eb 0c                	jmp    dc49 <cljn_equal_raw+0x645>
    dc3d:	b8 00 00 00 00       	mov    eax,0x0
    dc42:	eb 05                	jmp    dc49 <cljn_equal_raw+0x645>
    dc44:	b8 00 00 00 00       	mov    eax,0x0
    dc49:	48 8b 5d f8          	mov    rbx,QWORD PTR [rbp-0x8]
    dc4d:	c9                   	leave
    dc4e:	c3                   	ret

000000000000dc4f <cljn_eq>:
    dc4f:	f3 0f 1e fa          	endbr64
    dc53:	55                   	push   rbp
    dc54:	48 89 e5             	mov    rbp,rsp
    dc57:	48 83 ec 10          	sub    rsp,0x10
    dc5b:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    dc5f:	48 89 75 f0          	mov    QWORD PTR [rbp-0x10],rsi
    dc63:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    dc67:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    dc6b:	48 89 d6             	mov    rsi,rdx
    dc6e:	48 89 c7             	mov    rdi,rax
    dc71:	e8 8e f9 ff ff       	call   d604 <cljn_equal_raw>
    dc76:	89 c7                	mov    edi,eax
    dc78:	e8 1b f7 ff ff       	call   d398 <b2v>
    dc7d:	c9                   	leave
    dc7e:	c3                   	ret

000000000000dc7f <cljn_truthy>:
    dc7f:	f3 0f 1e fa          	endbr64
    dc83:	55                   	push   rbp
    dc84:	48 89 e5             	mov    rbp,rsp
    dc87:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    dc8b:	48 83 7d f8 02       	cmp    QWORD PTR [rbp-0x8],0x2
    dc90:	74 0e                	je     dca0 <cljn_truthy+0x21>
    dc92:	48 83 7d f8 06       	cmp    QWORD PTR [rbp-0x8],0x6
    dc97:	74 07                	je     dca0 <cljn_truthy+0x21>
    dc99:	b8 01 00 00 00       	mov    eax,0x1
    dc9e:	eb 05                	jmp    dca5 <cljn_truthy+0x26>
    dca0:	b8 00 00 00 00       	mov    eax,0x0
    dca5:	5d                   	pop    rbp
    dca6:	c3                   	ret

000000000000dca7 <cljn_not>:
    dca7:	f3 0f 1e fa          	endbr64
    dcab:	55                   	push   rbp
    dcac:	48 89 e5             	mov    rbp,rsp
    dcaf:	48 83 ec 08          	sub    rsp,0x8
    dcb3:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    dcb7:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    dcbb:	48 89 c7             	mov    rdi,rax
    dcbe:	e8 bc ff ff ff       	call   dc7f <cljn_truthy>
    dcc3:	85 c0                	test   eax,eax
    dcc5:	0f 94 c0             	sete   al
    dcc8:	0f b6 c0             	movzx  eax,al
    dccb:	89 c7                	mov    edi,eax
    dccd:	e8 c6 f6 ff ff       	call   d398 <b2v>
    dcd2:	c9                   	leave
    dcd3:	c3                   	ret

000000000000dcd4 <cljn_nilp>:
    dcd4:	f3 0f 1e fa          	endbr64
    dcd8:	55                   	push   rbp
    dcd9:	48 89 e5             	mov    rbp,rsp
    dcdc:	48 83 ec 08          	sub    rsp,0x8
    dce0:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    dce4:	48 83 7d f8 02       	cmp    QWORD PTR [rbp-0x8],0x2
    dce9:	0f 94 c0             	sete   al
    dcec:	0f b6 c0             	movzx  eax,al
    dcef:	89 c7                	mov    edi,eax
    dcf1:	e8 a2 f6 ff ff       	call   d398 <b2v>
    dcf6:	c9                   	leave
    dcf7:	c3                   	ret

000000000000dcf8 <cljn_emptyp>:
    dcf8:	f3 0f 1e fa          	endbr64
    dcfc:	55                   	push   rbp
    dcfd:	48 89 e5             	mov    rbp,rsp
    dd00:	48 83 ec 08          	sub    rsp,0x8
    dd04:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    dd08:	48 83 7d f8 12       	cmp    QWORD PTR [rbp-0x8],0x12
    dd0d:	74 07                	je     dd16 <cljn_emptyp+0x1e>
    dd0f:	48 83 7d f8 02       	cmp    QWORD PTR [rbp-0x8],0x2
    dd14:	75 0a                	jne    dd20 <cljn_emptyp+0x28>
    dd16:	b8 0a 00 00 00       	mov    eax,0xa
    dd1b:	e9 f9 00 00 00       	jmp    de19 <cljn_emptyp+0x121>
    dd20:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    dd24:	48 89 c7             	mov    rdi,rax
    dd27:	e8 9e 90 ff ff       	call   6dca <obj_type>
    dd2c:	83 f8 10             	cmp    eax,0x10
    dd2f:	0f 87 df 00 00 00    	ja     de14 <cljn_emptyp+0x11c>
    dd35:	89 c0                	mov    eax,eax
    dd37:	48 8d 14 85 00 00 00 	lea    rdx,[rax*4+0x0]
    dd3e:	00 
    dd3f:	48 8d 05 f6 27 00 00 	lea    rax,[rip+0x27f6]        # 1053c <_IO_stdin_used+0x53c>
    dd46:	8b 04 02             	mov    eax,DWORD PTR [rdx+rax*1]
    dd49:	48 98                	cdqe
    dd4b:	48 8d 15 ea 27 00 00 	lea    rdx,[rip+0x27ea]        # 1053c <_IO_stdin_used+0x53c>
    dd52:	48 01 d0             	add    rax,rdx
    dd55:	3e ff e0             	notrack jmp rax
    dd58:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    dd5c:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    dd60:	48 85 c0             	test   rax,rax
    dd63:	0f 94 c0             	sete   al
    dd66:	0f b6 c0             	movzx  eax,al
    dd69:	89 c7                	mov    edi,eax
    dd6b:	e8 28 f6 ff ff       	call   d398 <b2v>
    dd70:	e9 a4 00 00 00       	jmp    de19 <cljn_emptyp+0x121>
    dd75:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    dd79:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    dd7d:	48 85 c0             	test   rax,rax
    dd80:	0f 94 c0             	sete   al
    dd83:	0f b6 c0             	movzx  eax,al
    dd86:	89 c7                	mov    edi,eax
    dd88:	e8 0b f6 ff ff       	call   d398 <b2v>
    dd8d:	e9 87 00 00 00       	jmp    de19 <cljn_emptyp+0x121>
    dd92:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    dd96:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    dd9a:	48 85 c0             	test   rax,rax
    dd9d:	0f 94 c0             	sete   al
    dda0:	0f b6 c0             	movzx  eax,al
    dda3:	89 c7                	mov    edi,eax
    dda5:	e8 ee f5 ff ff       	call   d398 <b2v>
    ddaa:	eb 6d                	jmp    de19 <cljn_emptyp+0x121>
    ddac:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    ddb0:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    ddb4:	48 85 c0             	test   rax,rax
    ddb7:	0f 94 c0             	sete   al
    ddba:	0f b6 c0             	movzx  eax,al
    ddbd:	89 c7                	mov    edi,eax
    ddbf:	e8 d4 f5 ff ff       	call   d398 <b2v>
    ddc4:	eb 53                	jmp    de19 <cljn_emptyp+0x121>
    ddc6:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    ddca:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    ddce:	48 85 c0             	test   rax,rax
    ddd1:	0f 94 c0             	sete   al
    ddd4:	0f b6 c0             	movzx  eax,al
    ddd7:	89 c7                	mov    edi,eax
    ddd9:	e8 ba f5 ff ff       	call   d398 <b2v>
    ddde:	eb 39                	jmp    de19 <cljn_emptyp+0x121>
    dde0:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    dde4:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    dde8:	48 85 c0             	test   rax,rax
    ddeb:	0f 94 c0             	sete   al
    ddee:	0f b6 c0             	movzx  eax,al
    ddf1:	89 c7                	mov    edi,eax
    ddf3:	e8 a0 f5 ff ff       	call   d398 <b2v>
    ddf8:	eb 1f                	jmp    de19 <cljn_emptyp+0x121>
    ddfa:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    ddfe:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    de02:	48 85 c0             	test   rax,rax
    de05:	0f 94 c0             	sete   al
    de08:	0f b6 c0             	movzx  eax,al
    de0b:	89 c7                	mov    edi,eax
    de0d:	e8 86 f5 ff ff       	call   d398 <b2v>
    de12:	eb 05                	jmp    de19 <cljn_emptyp+0x121>
    de14:	b8 06 00 00 00       	mov    eax,0x6
    de19:	c9                   	leave
    de1a:	c3                   	ret

000000000000de1b <cljn_first>:
    de1b:	f3 0f 1e fa          	endbr64
    de1f:	55                   	push   rbp
    de20:	48 89 e5             	mov    rbp,rsp
    de23:	48 83 ec 30          	sub    rsp,0x30
    de27:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    de2b:	48 83 7d d8 12       	cmp    QWORD PTR [rbp-0x28],0x12
    de30:	74 07                	je     de39 <cljn_first+0x1e>
    de32:	48 83 7d d8 02       	cmp    QWORD PTR [rbp-0x28],0x2
    de37:	75 0a                	jne    de43 <cljn_first+0x28>
    de39:	b8 02 00 00 00       	mov    eax,0x2
    de3e:	e9 25 01 00 00       	jmp    df68 <cljn_first+0x14d>
    de43:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    de47:	48 89 c7             	mov    rdi,rax
    de4a:	e8 7b 8f ff ff       	call   6dca <obj_type>
    de4f:	83 f8 10             	cmp    eax,0x10
    de52:	0f 87 fc 00 00 00    	ja     df54 <cljn_first+0x139>
    de58:	89 c0                	mov    eax,eax
    de5a:	48 8d 14 85 00 00 00 	lea    rdx,[rax*4+0x0]
    de61:	00 
    de62:	48 8d 05 37 27 00 00 	lea    rax,[rip+0x2737]        # 105a0 <_IO_stdin_used+0x5a0>
    de69:	8b 04 02             	mov    eax,DWORD PTR [rdx+rax*1]
    de6c:	48 98                	cdqe
    de6e:	48 8d 15 2b 27 00 00 	lea    rdx,[rip+0x272b]        # 105a0 <_IO_stdin_used+0x5a0>
    de75:	48 01 d0             	add    rax,rdx
    de78:	3e ff e0             	notrack jmp rax
    de7b:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    de7f:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    de83:	e9 e0 00 00 00       	jmp    df68 <cljn_first+0x14d>
    de88:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    de8c:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    de90:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    de94:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    de98:	48 85 c0             	test   rax,rax
    de9b:	7e 0d                	jle    deaa <cljn_first+0x8f>
    de9d:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    dea1:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    dea5:	e9 be 00 00 00       	jmp    df68 <cljn_first+0x14d>
    deaa:	b8 02 00 00 00       	mov    eax,0x2
    deaf:	e9 b4 00 00 00       	jmp    df68 <cljn_first+0x14d>
    deb4:	bf 12 00 00 00       	mov    edi,0x12
    deb9:	e8 a6 8d ff ff       	call   6c64 <cljn_gc_push>
    debe:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    dec2:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    dec6:	be 00 00 00 00       	mov    esi,0x0
    decb:	48 89 c7             	mov    rdi,rax
    dece:	e8 a9 b7 ff ff       	call   967c <hmap_cons_walk>
    ded3:	48 8b 05 a6 61 00 02 	mov    rax,QWORD PTR [rip+0x20061a6]        # 2014080 <gc_sp>
    deda:	48 83 e8 01          	sub    rax,0x1
    dede:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    dee5:	00 
    dee6:	48 8d 05 93 61 00 00 	lea    rax,[rip+0x6193]        # 14080 <gc_stack>
    deed:	48 8b 04 02          	mov    rax,QWORD PTR [rdx+rax*1]
    def1:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    def5:	bf 01 00 00 00       	mov    edi,0x1
    defa:	e8 dd 8d ff ff       	call   6cdc <cljn_gc_popn>
    deff:	48 83 7d e8 12       	cmp    QWORD PTR [rbp-0x18],0x12
    df04:	74 0a                	je     df10 <cljn_first+0xf5>
    df06:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    df0a:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    df0e:	eb 58                	jmp    df68 <cljn_first+0x14d>
    df10:	b8 02 00 00 00       	mov    eax,0x2
    df15:	eb 51                	jmp    df68 <cljn_first+0x14d>
    df17:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    df1b:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    df1f:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    df23:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    df27:	48 85 c0             	test   rax,rax
    df2a:	7e 13                	jle    df3f <cljn_first+0x124>
    df2c:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    df30:	be 00 00 00 00       	mov    esi,0x0
    df35:	48 89 c7             	mov    rdi,rax
    df38:	e8 a2 a0 ff ff       	call   7fdf <pv_nth>
    df3d:	eb 29                	jmp    df68 <cljn_first+0x14d>
    df3f:	b8 02 00 00 00       	mov    eax,0x2
    df44:	eb 22                	jmp    df68 <cljn_first+0x14d>
    df46:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    df4a:	48 89 c7             	mov    rdi,rax
    df4d:	e8 df d6 ff ff       	call   b631 <cljn_sorted_first>
    df52:	eb 14                	jmp    df68 <cljn_first+0x14d>
    df54:	48 8d 05 25 26 00 00 	lea    rax,[rip+0x2625]        # 10580 <_IO_stdin_used+0x580>
    df5b:	48 89 c7             	mov    rdi,rax
    df5e:	e8 2b 8e ff ff       	call   6d8e <die>
    df63:	b8 02 00 00 00       	mov    eax,0x2
    df68:	c9                   	leave
    df69:	c3                   	ret

000000000000df6a <cljn_rest>:
    df6a:	f3 0f 1e fa          	endbr64
    df6e:	55                   	push   rbp
    df6f:	48 89 e5             	mov    rbp,rsp
    df72:	48 83 ec 50          	sub    rsp,0x50
    df76:	48 89 7d b8          	mov    QWORD PTR [rbp-0x48],rdi
    df7a:	48 83 7d b8 12       	cmp    QWORD PTR [rbp-0x48],0x12
    df7f:	74 07                	je     df88 <cljn_rest+0x1e>
    df81:	48 83 7d b8 02       	cmp    QWORD PTR [rbp-0x48],0x2
    df86:	75 0a                	jne    df92 <cljn_rest+0x28>
    df88:	b8 12 00 00 00       	mov    eax,0x12
    df8d:	e9 1c 02 00 00       	jmp    e1ae <cljn_rest+0x244>
    df92:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    df96:	48 89 c7             	mov    rdi,rax
    df99:	e8 2c 8e ff ff       	call   6dca <obj_type>
    df9e:	83 f8 02             	cmp    eax,0x2
    dfa1:	75 0d                	jne    dfb0 <cljn_rest+0x46>
    dfa3:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    dfa7:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    dfab:	e9 fe 01 00 00       	jmp    e1ae <cljn_rest+0x244>
    dfb0:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    dfb4:	48 89 c7             	mov    rdi,rax
    dfb7:	e8 0e 8e ff ff       	call   6dca <obj_type>
    dfbc:	83 f8 0d             	cmp    eax,0xd
    dfbf:	75 69                	jne    e02a <cljn_rest+0xc0>
    dfc1:	bf 12 00 00 00       	mov    edi,0x12
    dfc6:	e8 99 8c ff ff       	call   6c64 <cljn_gc_push>
    dfcb:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    dfcf:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    dfd3:	be 00 00 00 00       	mov    esi,0x0
    dfd8:	48 89 c7             	mov    rdi,rax
    dfdb:	e8 9c b6 ff ff       	call   967c <hmap_cons_walk>
    dfe0:	48 8b 05 99 60 00 02 	mov    rax,QWORD PTR [rip+0x2006099]        # 2014080 <gc_sp>
    dfe7:	48 83 e8 01          	sub    rax,0x1
    dfeb:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    dff2:	00 
    dff3:	48 8d 05 86 60 00 00 	lea    rax,[rip+0x6086]        # 14080 <gc_stack>
    dffa:	48 8b 04 02          	mov    rax,QWORD PTR [rdx+rax*1]
    dffe:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    e002:	bf 01 00 00 00       	mov    edi,0x1
    e007:	e8 d0 8c ff ff       	call   6cdc <cljn_gc_popn>
    e00c:	48 83 7d f8 12       	cmp    QWORD PTR [rbp-0x8],0x12
    e011:	74 0d                	je     e020 <cljn_rest+0xb6>
    e013:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e017:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    e01b:	e9 8e 01 00 00       	jmp    e1ae <cljn_rest+0x244>
    e020:	b8 12 00 00 00       	mov    eax,0x12
    e025:	e9 84 01 00 00       	jmp    e1ae <cljn_rest+0x244>
    e02a:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    e02e:	48 89 c7             	mov    rdi,rax
    e031:	e8 94 8d ff ff       	call   6dca <obj_type>
    e036:	83 f8 0f             	cmp    eax,0xf
    e039:	74 11                	je     e04c <cljn_rest+0xe2>
    e03b:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    e03f:	48 89 c7             	mov    rdi,rax
    e042:	e8 83 8d ff ff       	call   6dca <obj_type>
    e047:	83 f8 10             	cmp    eax,0x10
    e04a:	75 4d                	jne    e099 <cljn_rest+0x12f>
    e04c:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    e050:	48 89 c7             	mov    rdi,rax
    e053:	e8 72 8d ff ff       	call   6dca <obj_type>
    e058:	83 f8 10             	cmp    eax,0x10
    e05b:	75 07                	jne    e064 <cljn_rest+0xfa>
    e05d:	ba 00 00 00 00       	mov    edx,0x0
    e062:	eb 05                	jmp    e069 <cljn_rest+0xff>
    e064:	ba 02 00 00 00       	mov    edx,0x2
    e069:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    e06d:	89 d6                	mov    esi,edx
    e06f:	48 89 c7             	mov    rdi,rax
    e072:	e8 40 d6 ff ff       	call   b6b7 <sorted_seq>
    e077:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    e07b:	48 83 7d f0 12       	cmp    QWORD PTR [rbp-0x10],0x12
    e080:	74 0d                	je     e08f <cljn_rest+0x125>
    e082:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    e086:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    e08a:	e9 1f 01 00 00       	jmp    e1ae <cljn_rest+0x244>
    e08f:	b8 12 00 00 00       	mov    eax,0x12
    e094:	e9 15 01 00 00       	jmp    e1ae <cljn_rest+0x244>
    e099:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    e09d:	48 89 c7             	mov    rdi,rax
    e0a0:	e8 25 8d ff ff       	call   6dca <obj_type>
    e0a5:	83 f8 07             	cmp    eax,0x7
    e0a8:	74 15                	je     e0bf <cljn_rest+0x155>
    e0aa:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    e0ae:	48 89 c7             	mov    rdi,rax
    e0b1:	e8 14 8d ff ff       	call   6dca <obj_type>
    e0b6:	83 f8 05             	cmp    eax,0x5
    e0b9:	0f 85 db 00 00 00    	jne    e19a <cljn_rest+0x230>
    e0bf:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    e0c3:	48 89 c7             	mov    rdi,rax
    e0c6:	e8 ff 8c ff ff       	call   6dca <obj_type>
    e0cb:	83 f8 05             	cmp    eax,0x5
    e0ce:	0f 94 c0             	sete   al
    e0d1:	0f b6 c0             	movzx  eax,al
    e0d4:	89 45 cc             	mov    DWORD PTR [rbp-0x34],eax
    e0d7:	83 7d cc 00          	cmp    DWORD PTR [rbp-0x34],0x0
    e0db:	74 0a                	je     e0e7 <cljn_rest+0x17d>
    e0dd:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    e0e1:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    e0e5:	eb 08                	jmp    e0ef <cljn_rest+0x185>
    e0e7:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    e0eb:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    e0ef:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    e0f3:	48 c7 45 d0 12 00 00 	mov    QWORD PTR [rbp-0x30],0x12
    e0fa:	00 
    e0fb:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    e0ff:	48 89 c7             	mov    rdi,rax
    e102:	e8 5d 8b ff ff       	call   6c64 <cljn_gc_push>
    e107:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    e10b:	48 83 e8 01          	sub    rax,0x1
    e10f:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    e113:	eb 6e                	jmp    e183 <cljn_rest+0x219>
    e115:	83 7d cc 00          	cmp    DWORD PTR [rbp-0x34],0x0
    e119:	74 15                	je     e130 <cljn_rest+0x1c6>
    e11b:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    e11f:	48 8b 55 d8          	mov    rdx,QWORD PTR [rbp-0x28]
    e123:	48 89 d6             	mov    rsi,rdx
    e126:	48 89 c7             	mov    rdi,rax
    e129:	e8 b1 9e ff ff       	call   7fdf <pv_nth>
    e12e:	eb 11                	jmp    e141 <cljn_rest+0x1d7>
    e130:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    e134:	48 8b 55 d8          	mov    rdx,QWORD PTR [rbp-0x28]
    e138:	48 83 c2 02          	add    rdx,0x2
    e13c:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    e141:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    e145:	48 8b 55 d0          	mov    rdx,QWORD PTR [rbp-0x30]
    e149:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e14d:	48 89 d6             	mov    rsi,rdx
    e150:	48 89 c7             	mov    rdi,rax
    e153:	e8 89 95 ff ff       	call   76e1 <cljn_cons>
    e158:	48 89 45 d0          	mov    QWORD PTR [rbp-0x30],rax
    e15c:	48 8b 05 1d 5f 00 02 	mov    rax,QWORD PTR [rip+0x2005f1d]        # 2014080 <gc_sp>
    e163:	48 83 e8 01          	sub    rax,0x1
    e167:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    e16e:	00 
    e16f:	48 8d 15 0a 5f 00 00 	lea    rdx,[rip+0x5f0a]        # 14080 <gc_stack>
    e176:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    e17a:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    e17e:	48 83 6d d8 01       	sub    QWORD PTR [rbp-0x28],0x1
    e183:	48 83 7d d8 00       	cmp    QWORD PTR [rbp-0x28],0x0
    e188:	7f 8b                	jg     e115 <cljn_rest+0x1ab>
    e18a:	bf 01 00 00 00       	mov    edi,0x1
    e18f:	e8 48 8b ff ff       	call   6cdc <cljn_gc_popn>
    e194:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    e198:	eb 14                	jmp    e1ae <cljn_rest+0x244>
    e19a:	48 8d 05 43 24 00 00 	lea    rax,[rip+0x2443]        # 105e4 <_IO_stdin_used+0x5e4>
    e1a1:	48 89 c7             	mov    rdi,rax
    e1a4:	e8 e5 8b ff ff       	call   6d8e <die>
    e1a9:	b8 12 00 00 00       	mov    eax,0x12
    e1ae:	c9                   	leave
    e1af:	c3                   	ret

000000000000e1b0 <cljn_count>:
    e1b0:	f3 0f 1e fa          	endbr64
    e1b4:	55                   	push   rbp
    e1b5:	48 89 e5             	mov    rbp,rsp
    e1b8:	48 83 ec 20          	sub    rsp,0x20
    e1bc:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    e1c0:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e1c4:	48 89 c7             	mov    rdi,rax
    e1c7:	e8 fe 8b ff ff       	call   6dca <obj_type>
    e1cc:	83 f8 12             	cmp    eax,0x12
    e1cf:	0f 87 e4 00 00 00    	ja     e2b9 <cljn_count+0x109>
    e1d5:	89 c0                	mov    eax,eax
    e1d7:	48 8d 14 85 00 00 00 	lea    rdx,[rax*4+0x0]
    e1de:	00 
    e1df:	48 8d 05 1e 24 00 00 	lea    rax,[rip+0x241e]        # 10604 <_IO_stdin_used+0x604>
    e1e6:	8b 04 02             	mov    eax,DWORD PTR [rdx+rax*1]
    e1e9:	48 98                	cdqe
    e1eb:	48 8d 15 12 24 00 00 	lea    rdx,[rip+0x2412]        # 10604 <_IO_stdin_used+0x604>
    e1f2:	48 01 d0             	add    rax,rdx
    e1f5:	3e ff e0             	notrack jmp rax
    e1f8:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e1fc:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    e200:	48 01 c0             	add    rax,rax
    e203:	48 83 c8 01          	or     rax,0x1
    e207:	e9 f2 00 00 00       	jmp    e2fe <cljn_count+0x14e>
    e20c:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e210:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    e214:	48 01 c0             	add    rax,rax
    e217:	48 83 c8 01          	or     rax,0x1
    e21b:	e9 de 00 00 00       	jmp    e2fe <cljn_count+0x14e>
    e220:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e224:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    e228:	48 01 c0             	add    rax,rax
    e22b:	48 83 c8 01          	or     rax,0x1
    e22f:	e9 ca 00 00 00       	jmp    e2fe <cljn_count+0x14e>
    e234:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e238:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    e23c:	48 01 c0             	add    rax,rax
    e23f:	48 83 c8 01          	or     rax,0x1
    e243:	e9 b6 00 00 00       	jmp    e2fe <cljn_count+0x14e>
    e248:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e24c:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    e250:	48 01 c0             	add    rax,rax
    e253:	48 83 c8 01          	or     rax,0x1
    e257:	e9 a2 00 00 00       	jmp    e2fe <cljn_count+0x14e>
    e25c:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e260:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    e264:	48 01 c0             	add    rax,rax
    e267:	48 83 c8 01          	or     rax,0x1
    e26b:	e9 8e 00 00 00       	jmp    e2fe <cljn_count+0x14e>
    e270:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e274:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    e278:	48 01 c0             	add    rax,rax
    e27b:	48 83 c8 01          	or     rax,0x1
    e27f:	eb 7d                	jmp    e2fe <cljn_count+0x14e>
    e281:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e285:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    e289:	48 01 c0             	add    rax,rax
    e28c:	48 83 c8 01          	or     rax,0x1
    e290:	eb 6c                	jmp    e2fe <cljn_count+0x14e>
    e292:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e296:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    e29a:	48 89 c7             	mov    rdi,rax
    e29d:	e8 0e ff ff ff       	call   e1b0 <cljn_count>
    e2a2:	eb 5a                	jmp    e2fe <cljn_count+0x14e>
    e2a4:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e2a8:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    e2ac:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    e2b0:	48 01 c0             	add    rax,rax
    e2b3:	48 83 c8 01          	or     rax,0x1
    e2b7:	eb 45                	jmp    e2fe <cljn_count+0x14e>
    e2b9:	48 c7 45 f8 00 00 00 	mov    QWORD PTR [rbp-0x8],0x0
    e2c0:	00 
    e2c1:	eb 11                	jmp    e2d4 <cljn_count+0x124>
    e2c3:	48 83 45 f8 01       	add    QWORD PTR [rbp-0x8],0x1
    e2c8:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e2cc:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    e2d0:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    e2d4:	48 83 7d e8 12       	cmp    QWORD PTR [rbp-0x18],0x12
    e2d9:	74 18                	je     e2f3 <cljn_count+0x143>
    e2db:	48 83 7d e8 02       	cmp    QWORD PTR [rbp-0x18],0x2
    e2e0:	74 11                	je     e2f3 <cljn_count+0x143>
    e2e2:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e2e6:	48 89 c7             	mov    rdi,rax
    e2e9:	e8 dc 8a ff ff       	call   6dca <obj_type>
    e2ee:	83 f8 02             	cmp    eax,0x2
    e2f1:	74 d0                	je     e2c3 <cljn_count+0x113>
    e2f3:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e2f7:	48 01 c0             	add    rax,rax
    e2fa:	48 83 c8 01          	or     rax,0x1
    e2fe:	c9                   	leave
    e2ff:	c3                   	ret

000000000000e300 <sb_init>:
    e300:	f3 0f 1e fa          	endbr64
    e304:	55                   	push   rbp
    e305:	48 89 e5             	mov    rbp,rsp
    e308:	48 83 ec 10          	sub    rsp,0x10
    e30c:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    e310:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e314:	48 c7 40 10 20 00 00 	mov    QWORD PTR [rax+0x10],0x20
    e31b:	00 
    e31c:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e320:	48 c7 40 08 00 00 00 	mov    QWORD PTR [rax+0x8],0x0
    e327:	00 
    e328:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e32c:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    e330:	48 89 c7             	mov    rdi,rax
    e333:	e8 fc 89 ff ff       	call   6d34 <xalloc>
    e338:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    e33c:	48 89 02             	mov    QWORD PTR [rdx],rax
    e33f:	90                   	nop
    e340:	c9                   	leave
    e341:	c3                   	ret

000000000000e342 <sb_putc>:
    e342:	f3 0f 1e fa          	endbr64
    e346:	55                   	push   rbp
    e347:	48 89 e5             	mov    rbp,rsp
    e34a:	48 83 ec 10          	sub    rsp,0x10
    e34e:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    e352:	89 f0                	mov    eax,esi
    e354:	88 45 f4             	mov    BYTE PTR [rbp-0xc],al
    e357:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e35b:	48 8b 40 08          	mov    rax,QWORD PTR [rax+0x8]
    e35f:	48 8d 50 01          	lea    rdx,[rax+0x1]
    e363:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e367:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    e36b:	48 39 d0             	cmp    rax,rdx
    e36e:	73 50                	jae    e3c0 <sb_putc+0x7e>
    e370:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e374:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    e378:	48 8d 14 00          	lea    rdx,[rax+rax*1]
    e37c:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e380:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    e384:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e388:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    e38c:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e390:	48 8b 00             	mov    rax,QWORD PTR [rax]
    e393:	48 89 d6             	mov    rsi,rdx
    e396:	48 89 c7             	mov    rdi,rax
    e399:	e8 42 2d ff ff       	call   10e0 <realloc@plt>
    e39e:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    e3a2:	48 89 02             	mov    QWORD PTR [rdx],rax
    e3a5:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e3a9:	48 8b 00             	mov    rax,QWORD PTR [rax]
    e3ac:	48 85 c0             	test   rax,rax
    e3af:	75 0f                	jne    e3c0 <sb_putc+0x7e>
    e3b1:	48 8d 05 98 22 00 00 	lea    rax,[rip+0x2298]        # 10650 <_IO_stdin_used+0x650>
    e3b8:	48 89 c7             	mov    rdi,rax
    e3bb:	e8 ce 89 ff ff       	call   6d8e <die>
    e3c0:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e3c4:	48 8b 30             	mov    rsi,QWORD PTR [rax]
    e3c7:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e3cb:	48 8b 40 08          	mov    rax,QWORD PTR [rax+0x8]
    e3cf:	48 8d 48 01          	lea    rcx,[rax+0x1]
    e3d3:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    e3d7:	48 89 4a 08          	mov    QWORD PTR [rdx+0x8],rcx
    e3db:	48 8d 14 06          	lea    rdx,[rsi+rax*1]
    e3df:	0f b6 45 f4          	movzx  eax,BYTE PTR [rbp-0xc]
    e3e3:	88 02                	mov    BYTE PTR [rdx],al
    e3e5:	90                   	nop
    e3e6:	c9                   	leave
    e3e7:	c3                   	ret

000000000000e3e8 <sb_write>:
    e3e8:	f3 0f 1e fa          	endbr64
    e3ec:	55                   	push   rbp
    e3ed:	48 89 e5             	mov    rbp,rsp
    e3f0:	48 83 ec 30          	sub    rsp,0x30
    e3f4:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    e3f8:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    e3fc:	48 89 55 d8          	mov    QWORD PTR [rbp-0x28],rdx
    e400:	48 c7 45 f8 00 00 00 	mov    QWORD PTR [rbp-0x8],0x0
    e407:	00 
    e408:	eb 24                	jmp    e42e <sb_write+0x46>
    e40a:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    e40e:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e412:	48 01 d0             	add    rax,rdx
    e415:	0f b6 00             	movzx  eax,BYTE PTR [rax]
    e418:	0f be d0             	movsx  edx,al
    e41b:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e41f:	89 d6                	mov    esi,edx
    e421:	48 89 c7             	mov    rdi,rax
    e424:	e8 19 ff ff ff       	call   e342 <sb_putc>
    e429:	48 83 45 f8 01       	add    QWORD PTR [rbp-0x8],0x1
    e42e:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e432:	48 3b 45 d8          	cmp    rax,QWORD PTR [rbp-0x28]
    e436:	72 d2                	jb     e40a <sb_write+0x22>
    e438:	90                   	nop
    e439:	90                   	nop
    e43a:	c9                   	leave
    e43b:	c3                   	ret

000000000000e43c <sb_str>:
    e43c:	f3 0f 1e fa          	endbr64
    e440:	55                   	push   rbp
    e441:	48 89 e5             	mov    rbp,rsp
    e444:	48 83 ec 10          	sub    rsp,0x10
    e448:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    e44c:	48 89 75 f0          	mov    QWORD PTR [rbp-0x10],rsi
    e450:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    e454:	48 89 c7             	mov    rdi,rax
    e457:	e8 f4 2b ff ff       	call   1050 <strlen@plt>
    e45c:	48 89 c2             	mov    rdx,rax
    e45f:	48 8b 4d f0          	mov    rcx,QWORD PTR [rbp-0x10]
    e463:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e467:	48 89 ce             	mov    rsi,rcx
    e46a:	48 89 c7             	mov    rdi,rax
    e46d:	e8 76 ff ff ff       	call   e3e8 <sb_write>
    e472:	90                   	nop
    e473:	c9                   	leave
    e474:	c3                   	ret

000000000000e475 <sb_write_hmap>:
    e475:	f3 0f 1e fa          	endbr64
    e479:	55                   	push   rbp
    e47a:	48 89 e5             	mov    rbp,rsp
    e47d:	48 83 ec 50          	sub    rsp,0x50
    e481:	48 89 7d c8          	mov    QWORD PTR [rbp-0x38],rdi
    e485:	48 89 75 c0          	mov    QWORD PTR [rbp-0x40],rsi
    e489:	89 55 bc             	mov    DWORD PTR [rbp-0x44],edx
    e48c:	48 89 4d b0          	mov    QWORD PTR [rbp-0x50],rcx
    e490:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    e494:	48 89 c7             	mov    rdi,rax
    e497:	e8 2e 89 ff ff       	call   6dca <obj_type>
    e49c:	83 f8 0c             	cmp    eax,0xc
    e49f:	0f 85 bb 00 00 00    	jne    e560 <sb_write_hmap+0xeb>
    e4a5:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    e4a9:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    e4ad:	48 c7 45 e0 00 00 00 	mov    QWORD PTR [rbp-0x20],0x0
    e4b4:	00 
    e4b5:	e9 8f 00 00 00       	jmp    e549 <sb_write_hmap+0xd4>
    e4ba:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    e4be:	8b 00                	mov    eax,DWORD PTR [rax]
    e4c0:	85 c0                	test   eax,eax
    e4c2:	75 16                	jne    e4da <sb_write_hmap+0x65>
    e4c4:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    e4c8:	48 8d 15 8e 21 00 00 	lea    rdx,[rip+0x218e]        # 1065d <_IO_stdin_used+0x65d>
    e4cf:	48 89 d6             	mov    rsi,rdx
    e4d2:	48 89 c7             	mov    rdi,rax
    e4d5:	e8 62 ff ff ff       	call   e43c <sb_str>
    e4da:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    e4de:	c7 00 00 00 00 00    	mov    DWORD PTR [rax],0x0
    e4e4:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    e4e8:	48 8d 14 00          	lea    rdx,[rax+rax*1]
    e4ec:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e4f0:	48 83 c2 04          	add    rdx,0x4
    e4f4:	48 8b 0c d0          	mov    rcx,QWORD PTR [rax+rdx*8]
    e4f8:	8b 55 bc             	mov    edx,DWORD PTR [rbp-0x44]
    e4fb:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    e4ff:	48 89 ce             	mov    rsi,rcx
    e502:	48 89 c7             	mov    rdi,rax
    e505:	e8 b9 03 00 00       	call   e8c3 <write_val>
    e50a:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    e50e:	be 20 00 00 00       	mov    esi,0x20
    e513:	48 89 c7             	mov    rdi,rax
    e516:	e8 27 fe ff ff       	call   e342 <sb_putc>
    e51b:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    e51f:	48 01 c0             	add    rax,rax
    e522:	48 8d 50 01          	lea    rdx,[rax+0x1]
    e526:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e52a:	48 83 c2 04          	add    rdx,0x4
    e52e:	48 8b 0c d0          	mov    rcx,QWORD PTR [rax+rdx*8]
    e532:	8b 55 bc             	mov    edx,DWORD PTR [rbp-0x44]
    e535:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    e539:	48 89 ce             	mov    rsi,rcx
    e53c:	48 89 c7             	mov    rdi,rax
    e53f:	e8 7f 03 00 00       	call   e8c3 <write_val>
    e544:	48 83 45 e0 01       	add    QWORD PTR [rbp-0x20],0x1
    e549:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e54d:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    e551:	48 39 45 e0          	cmp    QWORD PTR [rbp-0x20],rax
    e555:	0f 8c 5f ff ff ff    	jl     e4ba <sb_write_hmap+0x45>
    e55b:	e9 01 01 00 00       	jmp    e661 <sb_write_hmap+0x1ec>
    e560:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    e564:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    e568:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e56c:	8b 40 10             	mov    eax,DWORD PTR [rax+0x10]
    e56f:	89 c0                	mov    eax,eax
    e571:	48 89 c7             	mov    rdi,rax
    e574:	e8 37 15 00 00       	call   fab0 <__popcountdi2>
    e579:	89 45 dc             	mov    DWORD PTR [rbp-0x24],eax
    e57c:	c7 45 d8 00 00 00 00 	mov    DWORD PTR [rbp-0x28],0x0
    e583:	e9 cd 00 00 00       	jmp    e655 <sb_write_hmap+0x1e0>
    e588:	8b 45 d8             	mov    eax,DWORD PTR [rbp-0x28]
    e58b:	8d 14 00             	lea    edx,[rax+rax*1]
    e58e:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e592:	48 63 d2             	movsxd rdx,edx
    e595:	48 83 c2 02          	add    rdx,0x2
    e599:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    e59e:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    e5a2:	48 83 7d f0 1a       	cmp    QWORD PTR [rbp-0x10],0x1a
    e5a7:	75 2d                	jne    e5d6 <sb_write_hmap+0x161>
    e5a9:	8b 45 d8             	mov    eax,DWORD PTR [rbp-0x28]
    e5ac:	01 c0                	add    eax,eax
    e5ae:	8d 50 01             	lea    edx,[rax+0x1]
    e5b1:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e5b5:	48 63 d2             	movsxd rdx,edx
    e5b8:	48 83 c2 02          	add    rdx,0x2
    e5bc:	48 8b 74 d0 08       	mov    rsi,QWORD PTR [rax+rdx*8+0x8]
    e5c1:	48 8b 4d b0          	mov    rcx,QWORD PTR [rbp-0x50]
    e5c5:	8b 55 bc             	mov    edx,DWORD PTR [rbp-0x44]
    e5c8:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    e5cc:	48 89 c7             	mov    rdi,rax
    e5cf:	e8 a1 fe ff ff       	call   e475 <sb_write_hmap>
    e5d4:	eb 7b                	jmp    e651 <sb_write_hmap+0x1dc>
    e5d6:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    e5da:	8b 00                	mov    eax,DWORD PTR [rax]
    e5dc:	85 c0                	test   eax,eax
    e5de:	75 16                	jne    e5f6 <sb_write_hmap+0x181>
    e5e0:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    e5e4:	48 8d 15 72 20 00 00 	lea    rdx,[rip+0x2072]        # 1065d <_IO_stdin_used+0x65d>
    e5eb:	48 89 d6             	mov    rsi,rdx
    e5ee:	48 89 c7             	mov    rdi,rax
    e5f1:	e8 46 fe ff ff       	call   e43c <sb_str>
    e5f6:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    e5fa:	c7 00 00 00 00 00    	mov    DWORD PTR [rax],0x0
    e600:	8b 55 bc             	mov    edx,DWORD PTR [rbp-0x44]
    e603:	48 8b 4d f0          	mov    rcx,QWORD PTR [rbp-0x10]
    e607:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    e60b:	48 89 ce             	mov    rsi,rcx
    e60e:	48 89 c7             	mov    rdi,rax
    e611:	e8 ad 02 00 00       	call   e8c3 <write_val>
    e616:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    e61a:	be 20 00 00 00       	mov    esi,0x20
    e61f:	48 89 c7             	mov    rdi,rax
    e622:	e8 1b fd ff ff       	call   e342 <sb_putc>
    e627:	8b 45 d8             	mov    eax,DWORD PTR [rbp-0x28]
    e62a:	01 c0                	add    eax,eax
    e62c:	8d 50 01             	lea    edx,[rax+0x1]
    e62f:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e633:	48 63 d2             	movsxd rdx,edx
    e636:	48 83 c2 02          	add    rdx,0x2
    e63a:	48 8b 4c d0 08       	mov    rcx,QWORD PTR [rax+rdx*8+0x8]
    e63f:	8b 55 bc             	mov    edx,DWORD PTR [rbp-0x44]
    e642:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    e646:	48 89 ce             	mov    rsi,rcx
    e649:	48 89 c7             	mov    rdi,rax
    e64c:	e8 72 02 00 00       	call   e8c3 <write_val>
    e651:	83 45 d8 01          	add    DWORD PTR [rbp-0x28],0x1
    e655:	8b 45 d8             	mov    eax,DWORD PTR [rbp-0x28]
    e658:	3b 45 dc             	cmp    eax,DWORD PTR [rbp-0x24]
    e65b:	0f 8c 27 ff ff ff    	jl     e588 <sb_write_hmap+0x113>
    e661:	c9                   	leave
    e662:	c3                   	ret

000000000000e663 <sb_write_hset>:
    e663:	f3 0f 1e fa          	endbr64
    e667:	55                   	push   rbp
    e668:	48 89 e5             	mov    rbp,rsp
    e66b:	48 83 ec 50          	sub    rsp,0x50
    e66f:	48 89 7d c8          	mov    QWORD PTR [rbp-0x38],rdi
    e673:	48 89 75 c0          	mov    QWORD PTR [rbp-0x40],rsi
    e677:	89 55 bc             	mov    DWORD PTR [rbp-0x44],edx
    e67a:	48 89 4d b0          	mov    QWORD PTR [rbp-0x50],rcx
    e67e:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    e682:	48 89 c7             	mov    rdi,rax
    e685:	e8 40 87 ff ff       	call   6dca <obj_type>
    e68a:	83 f8 0c             	cmp    eax,0xc
    e68d:	75 75                	jne    e704 <sb_write_hset+0xa1>
    e68f:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    e693:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    e697:	48 c7 45 e0 00 00 00 	mov    QWORD PTR [rbp-0x20],0x0
    e69e:	00 
    e69f:	eb 50                	jmp    e6f1 <sb_write_hset+0x8e>
    e6a1:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    e6a5:	8b 00                	mov    eax,DWORD PTR [rax]
    e6a7:	85 c0                	test   eax,eax
    e6a9:	75 11                	jne    e6bc <sb_write_hset+0x59>
    e6ab:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    e6af:	be 20 00 00 00       	mov    esi,0x20
    e6b4:	48 89 c7             	mov    rdi,rax
    e6b7:	e8 86 fc ff ff       	call   e342 <sb_putc>
    e6bc:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    e6c0:	c7 00 00 00 00 00    	mov    DWORD PTR [rax],0x0
    e6c6:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    e6ca:	48 8d 14 00          	lea    rdx,[rax+rax*1]
    e6ce:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e6d2:	48 83 c2 04          	add    rdx,0x4
    e6d6:	48 8b 0c d0          	mov    rcx,QWORD PTR [rax+rdx*8]
    e6da:	8b 55 bc             	mov    edx,DWORD PTR [rbp-0x44]
    e6dd:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    e6e1:	48 89 ce             	mov    rsi,rcx
    e6e4:	48 89 c7             	mov    rdi,rax
    e6e7:	e8 d7 01 00 00       	call   e8c3 <write_val>
    e6ec:	48 83 45 e0 01       	add    QWORD PTR [rbp-0x20],0x1
    e6f1:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e6f5:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    e6f9:	48 39 45 e0          	cmp    QWORD PTR [rbp-0x20],rax
    e6fd:	7c a2                	jl     e6a1 <sb_write_hset+0x3e>
    e6ff:	e9 c1 00 00 00       	jmp    e7c5 <sb_write_hset+0x162>
    e704:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    e708:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    e70c:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e710:	8b 40 10             	mov    eax,DWORD PTR [rax+0x10]
    e713:	89 c0                	mov    eax,eax
    e715:	48 89 c7             	mov    rdi,rax
    e718:	e8 93 13 00 00       	call   fab0 <__popcountdi2>
    e71d:	89 45 dc             	mov    DWORD PTR [rbp-0x24],eax
    e720:	c7 45 d8 00 00 00 00 	mov    DWORD PTR [rbp-0x28],0x0
    e727:	e9 8d 00 00 00       	jmp    e7b9 <sb_write_hset+0x156>
    e72c:	8b 45 d8             	mov    eax,DWORD PTR [rbp-0x28]
    e72f:	8d 14 00             	lea    edx,[rax+rax*1]
    e732:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e736:	48 63 d2             	movsxd rdx,edx
    e739:	48 83 c2 02          	add    rdx,0x2
    e73d:	48 8b 44 d0 08       	mov    rax,QWORD PTR [rax+rdx*8+0x8]
    e742:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    e746:	48 83 7d f0 1a       	cmp    QWORD PTR [rbp-0x10],0x1a
    e74b:	75 2d                	jne    e77a <sb_write_hset+0x117>
    e74d:	8b 45 d8             	mov    eax,DWORD PTR [rbp-0x28]
    e750:	01 c0                	add    eax,eax
    e752:	8d 50 01             	lea    edx,[rax+0x1]
    e755:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e759:	48 63 d2             	movsxd rdx,edx
    e75c:	48 83 c2 02          	add    rdx,0x2
    e760:	48 8b 74 d0 08       	mov    rsi,QWORD PTR [rax+rdx*8+0x8]
    e765:	48 8b 4d b0          	mov    rcx,QWORD PTR [rbp-0x50]
    e769:	8b 55 bc             	mov    edx,DWORD PTR [rbp-0x44]
    e76c:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    e770:	48 89 c7             	mov    rdi,rax
    e773:	e8 eb fe ff ff       	call   e663 <sb_write_hset>
    e778:	eb 3b                	jmp    e7b5 <sb_write_hset+0x152>
    e77a:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    e77e:	8b 00                	mov    eax,DWORD PTR [rax]
    e780:	85 c0                	test   eax,eax
    e782:	75 11                	jne    e795 <sb_write_hset+0x132>
    e784:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    e788:	be 20 00 00 00       	mov    esi,0x20
    e78d:	48 89 c7             	mov    rdi,rax
    e790:	e8 ad fb ff ff       	call   e342 <sb_putc>
    e795:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    e799:	c7 00 00 00 00 00    	mov    DWORD PTR [rax],0x0
    e79f:	8b 55 bc             	mov    edx,DWORD PTR [rbp-0x44]
    e7a2:	48 8b 4d f0          	mov    rcx,QWORD PTR [rbp-0x10]
    e7a6:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    e7aa:	48 89 ce             	mov    rsi,rcx
    e7ad:	48 89 c7             	mov    rdi,rax
    e7b0:	e8 0e 01 00 00       	call   e8c3 <write_val>
    e7b5:	83 45 d8 01          	add    DWORD PTR [rbp-0x28],0x1
    e7b9:	8b 45 d8             	mov    eax,DWORD PTR [rbp-0x28]
    e7bc:	3b 45 dc             	cmp    eax,DWORD PTR [rbp-0x24]
    e7bf:	0f 8c 67 ff ff ff    	jl     e72c <sb_write_hset+0xc9>
    e7c5:	c9                   	leave
    e7c6:	c3                   	ret

000000000000e7c7 <sb_write_tree>:
    e7c7:	f3 0f 1e fa          	endbr64
    e7cb:	55                   	push   rbp
    e7cc:	48 89 e5             	mov    rbp,rsp
    e7cf:	48 83 ec 30          	sub    rsp,0x30
    e7d3:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    e7d7:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    e7db:	89 55 dc             	mov    DWORD PTR [rbp-0x24],edx
    e7de:	89 4d d8             	mov    DWORD PTR [rbp-0x28],ecx
    e7e1:	4c 89 45 d0          	mov    QWORD PTR [rbp-0x30],r8
    e7e5:	48 83 7d e0 02       	cmp    QWORD PTR [rbp-0x20],0x2
    e7ea:	0f 84 d0 00 00 00    	je     e8c0 <sb_write_tree+0xf9>
    e7f0:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    e7f4:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    e7f8:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e7fc:	48 8b 70 20          	mov    rsi,QWORD PTR [rax+0x20]
    e800:	48 8b 7d d0          	mov    rdi,QWORD PTR [rbp-0x30]
    e804:	8b 4d d8             	mov    ecx,DWORD PTR [rbp-0x28]
    e807:	8b 55 dc             	mov    edx,DWORD PTR [rbp-0x24]
    e80a:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e80e:	49 89 f8             	mov    r8,rdi
    e811:	48 89 c7             	mov    rdi,rax
    e814:	e8 ae ff ff ff       	call   e7c7 <sb_write_tree>
    e819:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    e81d:	8b 00                	mov    eax,DWORD PTR [rax]
    e81f:	85 c0                	test   eax,eax
    e821:	75 25                	jne    e848 <sb_write_tree+0x81>
    e823:	83 7d dc 00          	cmp    DWORD PTR [rbp-0x24],0x0
    e827:	74 09                	je     e832 <sb_write_tree+0x6b>
    e829:	48 8d 05 2d 1e 00 00 	lea    rax,[rip+0x1e2d]        # 1065d <_IO_stdin_used+0x65d>
    e830:	eb 07                	jmp    e839 <sb_write_tree+0x72>
    e832:	48 8d 05 27 1e 00 00 	lea    rax,[rip+0x1e27]        # 10660 <_IO_stdin_used+0x660>
    e839:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    e83d:	48 89 c6             	mov    rsi,rax
    e840:	48 89 d7             	mov    rdi,rdx
    e843:	e8 f4 fb ff ff       	call   e43c <sb_str>
    e848:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    e84c:	c7 00 00 00 00 00    	mov    DWORD PTR [rax],0x0
    e852:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e856:	48 8b 48 10          	mov    rcx,QWORD PTR [rax+0x10]
    e85a:	8b 55 d8             	mov    edx,DWORD PTR [rbp-0x28]
    e85d:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e861:	48 89 ce             	mov    rsi,rcx
    e864:	48 89 c7             	mov    rdi,rax
    e867:	e8 57 00 00 00       	call   e8c3 <write_val>
    e86c:	83 7d dc 00          	cmp    DWORD PTR [rbp-0x24],0x0
    e870:	74 2b                	je     e89d <sb_write_tree+0xd6>
    e872:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e876:	be 20 00 00 00       	mov    esi,0x20
    e87b:	48 89 c7             	mov    rdi,rax
    e87e:	e8 bf fa ff ff       	call   e342 <sb_putc>
    e883:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e887:	48 8b 48 18          	mov    rcx,QWORD PTR [rax+0x18]
    e88b:	8b 55 d8             	mov    edx,DWORD PTR [rbp-0x28]
    e88e:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e892:	48 89 ce             	mov    rsi,rcx
    e895:	48 89 c7             	mov    rdi,rax
    e898:	e8 26 00 00 00       	call   e8c3 <write_val>
    e89d:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    e8a1:	48 8b 70 28          	mov    rsi,QWORD PTR [rax+0x28]
    e8a5:	48 8b 7d d0          	mov    rdi,QWORD PTR [rbp-0x30]
    e8a9:	8b 4d d8             	mov    ecx,DWORD PTR [rbp-0x28]
    e8ac:	8b 55 dc             	mov    edx,DWORD PTR [rbp-0x24]
    e8af:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    e8b3:	49 89 f8             	mov    r8,rdi
    e8b6:	48 89 c7             	mov    rdi,rax
    e8b9:	e8 09 ff ff ff       	call   e7c7 <sb_write_tree>
    e8be:	eb 01                	jmp    e8c1 <sb_write_tree+0xfa>
    e8c0:	90                   	nop
    e8c1:	c9                   	leave
    e8c2:	c3                   	ret

000000000000e8c3 <write_val>:
    e8c3:	f3 0f 1e fa          	endbr64
    e8c7:	55                   	push   rbp
    e8c8:	48 89 e5             	mov    rbp,rsp
    e8cb:	48 81 ec b0 00 00 00 	sub    rsp,0xb0
    e8d2:	48 89 bd 68 ff ff ff 	mov    QWORD PTR [rbp-0x98],rdi
    e8d9:	48 89 b5 60 ff ff ff 	mov    QWORD PTR [rbp-0xa0],rsi
    e8e0:	89 95 5c ff ff ff    	mov    DWORD PTR [rbp-0xa4],edx
    e8e6:	64 48 8b 04 25 28 00 	mov    rax,QWORD PTR fs:0x28
    e8ed:	00 00 
    e8ef:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    e8f3:	31 c0                	xor    eax,eax
    e8f5:	48 8b 85 60 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa0]
    e8fc:	83 e0 01             	and    eax,0x1
    e8ff:	48 85 c0             	test   rax,rax
    e902:	74 57                	je     e95b <write_val+0x98>
    e904:	48 8b 85 60 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa0]
    e90b:	48 d1 f8             	sar    rax,1
    e90e:	48 89 c2             	mov    rdx,rax
    e911:	48 8d 45 d0          	lea    rax,[rbp-0x30]
    e915:	48 89 d1             	mov    rcx,rdx
    e918:	48 8d 15 43 1d 00 00 	lea    rdx,[rip+0x1d43]        # 10662 <_IO_stdin_used+0x662>
    e91f:	be 20 00 00 00       	mov    esi,0x20
    e924:	48 89 c7             	mov    rdi,rax
    e927:	b8 00 00 00 00       	mov    eax,0x0
    e92c:	e8 3f 27 ff ff       	call   1070 <snprintf@plt>
    e931:	89 85 7c ff ff ff    	mov    DWORD PTR [rbp-0x84],eax
    e937:	8b 85 7c ff ff ff    	mov    eax,DWORD PTR [rbp-0x84]
    e93d:	48 63 d0             	movsxd rdx,eax
    e940:	48 8d 4d d0          	lea    rcx,[rbp-0x30]
    e944:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    e94b:	48 89 ce             	mov    rsi,rcx
    e94e:	48 89 c7             	mov    rdi,rax
    e951:	e8 92 fa ff ff       	call   e3e8 <sb_write>
    e956:	e9 90 06 00 00       	jmp    efeb <write_val+0x728>
    e95b:	48 83 bd 60 ff ff ff 	cmp    QWORD PTR [rbp-0xa0],0x2
    e962:	02 
    e963:	75 2b                	jne    e990 <write_val+0xcd>
    e965:	83 bd 5c ff ff ff 00 	cmp    DWORD PTR [rbp-0xa4],0x0
    e96c:	0f 85 78 06 00 00    	jne    efea <write_val+0x727>
    e972:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    e979:	48 8d 15 e6 1c 00 00 	lea    rdx,[rip+0x1ce6]        # 10666 <_IO_stdin_used+0x666>
    e980:	48 89 d6             	mov    rsi,rdx
    e983:	48 89 c7             	mov    rdi,rax
    e986:	e8 b1 fa ff ff       	call   e43c <sb_str>
    e98b:	e9 5a 06 00 00       	jmp    efea <write_val+0x727>
    e990:	48 83 bd 60 ff ff ff 	cmp    QWORD PTR [rbp-0xa0],0xa
    e997:	0a 
    e998:	75 1e                	jne    e9b8 <write_val+0xf5>
    e99a:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    e9a1:	48 8d 15 c2 1c 00 00 	lea    rdx,[rip+0x1cc2]        # 1066a <_IO_stdin_used+0x66a>
    e9a8:	48 89 d6             	mov    rsi,rdx
    e9ab:	48 89 c7             	mov    rdi,rax
    e9ae:	e8 89 fa ff ff       	call   e43c <sb_str>
    e9b3:	e9 33 06 00 00       	jmp    efeb <write_val+0x728>
    e9b8:	48 83 bd 60 ff ff ff 	cmp    QWORD PTR [rbp-0xa0],0x6
    e9bf:	06 
    e9c0:	75 1e                	jne    e9e0 <write_val+0x11d>
    e9c2:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    e9c9:	48 8d 15 9f 1c 00 00 	lea    rdx,[rip+0x1c9f]        # 1066f <_IO_stdin_used+0x66f>
    e9d0:	48 89 d6             	mov    rsi,rdx
    e9d3:	48 89 c7             	mov    rdi,rax
    e9d6:	e8 61 fa ff ff       	call   e43c <sb_str>
    e9db:	e9 0b 06 00 00       	jmp    efeb <write_val+0x728>
    e9e0:	48 83 bd 60 ff ff ff 	cmp    QWORD PTR [rbp-0xa0],0x12
    e9e7:	12 
    e9e8:	75 1e                	jne    ea08 <write_val+0x145>
    e9ea:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    e9f1:	48 8d 15 7d 1c 00 00 	lea    rdx,[rip+0x1c7d]        # 10675 <_IO_stdin_used+0x675>
    e9f8:	48 89 d6             	mov    rsi,rdx
    e9fb:	48 89 c7             	mov    rdi,rax
    e9fe:	e8 39 fa ff ff       	call   e43c <sb_str>
    ea03:	e9 e3 05 00 00       	jmp    efeb <write_val+0x728>
    ea08:	48 8b 85 60 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa0]
    ea0f:	48 89 c7             	mov    rdi,rax
    ea12:	e8 b3 83 ff ff       	call   6dca <obj_type>
    ea17:	83 f8 10             	cmp    eax,0x10
    ea1a:	0f 87 af 05 00 00    	ja     efcf <write_val+0x70c>
    ea20:	89 c0                	mov    eax,eax
    ea22:	48 8d 14 85 00 00 00 	lea    rdx,[rax*4+0x0]
    ea29:	00 
    ea2a:	48 8d 05 57 1c 00 00 	lea    rax,[rip+0x1c57]        # 10688 <_IO_stdin_used+0x688>
    ea31:	8b 04 02             	mov    eax,DWORD PTR [rdx+rax*1]
    ea34:	48 98                	cdqe
    ea36:	48 8d 15 4b 1c 00 00 	lea    rdx,[rip+0x1c4b]        # 10688 <_IO_stdin_used+0x688>
    ea3d:	48 01 d0             	add    rax,rdx
    ea40:	3e ff e0             	notrack jmp rax
    ea43:	48 8b 85 60 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa0]
    ea4a:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
    ea4e:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    ea52:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    ea56:	48 8b 45 c8          	mov    rax,QWORD PTR [rbp-0x38]
    ea5a:	48 8b 48 18          	mov    rcx,QWORD PTR [rax+0x18]
    ea5e:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ea65:	48 89 ce             	mov    rsi,rcx
    ea68:	48 89 c7             	mov    rdi,rax
    ea6b:	e8 78 f9 ff ff       	call   e3e8 <sb_write>
    ea70:	e9 76 05 00 00       	jmp    efeb <write_val+0x728>
    ea75:	48 8b 85 60 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa0]
    ea7c:	48 89 45 c0          	mov    QWORD PTR [rbp-0x40],rax
    ea80:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ea87:	be 3a 00 00 00       	mov    esi,0x3a
    ea8c:	48 89 c7             	mov    rdi,rax
    ea8f:	e8 ae f8 ff ff       	call   e342 <sb_putc>
    ea94:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    ea98:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    ea9c:	48 8b 45 c0          	mov    rax,QWORD PTR [rbp-0x40]
    eaa0:	48 8b 48 18          	mov    rcx,QWORD PTR [rax+0x18]
    eaa4:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    eaab:	48 89 ce             	mov    rsi,rcx
    eaae:	48 89 c7             	mov    rdi,rax
    eab1:	e8 32 f9 ff ff       	call   e3e8 <sb_write>
    eab6:	e9 30 05 00 00       	jmp    efeb <write_val+0x728>
    eabb:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    eac2:	be 28 00 00 00       	mov    esi,0x28
    eac7:	48 89 c7             	mov    rdi,rax
    eaca:	e8 73 f8 ff ff       	call   e342 <sb_putc>
    eacf:	c7 85 78 ff ff ff 01 	mov    DWORD PTR [rbp-0x88],0x1
    ead6:	00 00 00 
    ead9:	eb 5c                	jmp    eb37 <write_val+0x274>
    eadb:	83 bd 78 ff ff ff 00 	cmp    DWORD PTR [rbp-0x88],0x0
    eae2:	75 14                	jne    eaf8 <write_val+0x235>
    eae4:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    eaeb:	be 20 00 00 00       	mov    esi,0x20
    eaf0:	48 89 c7             	mov    rdi,rax
    eaf3:	e8 4a f8 ff ff       	call   e342 <sb_putc>
    eaf8:	c7 85 78 ff ff ff 00 	mov    DWORD PTR [rbp-0x88],0x0
    eaff:	00 00 00 
    eb02:	48 8b 85 60 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa0]
    eb09:	48 8b 48 10          	mov    rcx,QWORD PTR [rax+0x10]
    eb0d:	8b 95 5c ff ff ff    	mov    edx,DWORD PTR [rbp-0xa4]
    eb13:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    eb1a:	48 89 ce             	mov    rsi,rcx
    eb1d:	48 89 c7             	mov    rdi,rax
    eb20:	e8 9e fd ff ff       	call   e8c3 <write_val>
    eb25:	48 8b 85 60 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa0]
    eb2c:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    eb30:	48 89 85 60 ff ff ff 	mov    QWORD PTR [rbp-0xa0],rax
    eb37:	48 83 bd 60 ff ff ff 	cmp    QWORD PTR [rbp-0xa0],0x12
    eb3e:	12 
    eb3f:	74 14                	je     eb55 <write_val+0x292>
    eb41:	48 8b 85 60 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa0]
    eb48:	48 89 c7             	mov    rdi,rax
    eb4b:	e8 7a 82 ff ff       	call   6dca <obj_type>
    eb50:	83 f8 02             	cmp    eax,0x2
    eb53:	74 86                	je     eadb <write_val+0x218>
    eb55:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    eb5c:	be 29 00 00 00       	mov    esi,0x29
    eb61:	48 89 c7             	mov    rdi,rax
    eb64:	e8 d9 f7 ff ff       	call   e342 <sb_putc>
    eb69:	e9 7d 04 00 00       	jmp    efeb <write_val+0x728>
    eb6e:	48 8b 85 60 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa0]
    eb75:	48 89 45 b8          	mov    QWORD PTR [rbp-0x48],rax
    eb79:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    eb80:	be 5b 00 00 00       	mov    esi,0x5b
    eb85:	48 89 c7             	mov    rdi,rax
    eb88:	e8 b5 f7 ff ff       	call   e342 <sb_putc>
    eb8d:	48 c7 45 80 00 00 00 	mov    QWORD PTR [rbp-0x80],0x0
    eb94:	00 
    eb95:	eb 4e                	jmp    ebe5 <write_val+0x322>
    eb97:	48 83 7d 80 00       	cmp    QWORD PTR [rbp-0x80],0x0
    eb9c:	74 14                	je     ebb2 <write_val+0x2ef>
    eb9e:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    eba5:	be 20 00 00 00       	mov    esi,0x20
    ebaa:	48 89 c7             	mov    rdi,rax
    ebad:	e8 90 f7 ff ff       	call   e342 <sb_putc>
    ebb2:	48 8b 55 80          	mov    rdx,QWORD PTR [rbp-0x80]
    ebb6:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    ebba:	48 89 d6             	mov    rsi,rdx
    ebbd:	48 89 c7             	mov    rdi,rax
    ebc0:	e8 1a 94 ff ff       	call   7fdf <pv_nth>
    ebc5:	48 89 c1             	mov    rcx,rax
    ebc8:	8b 95 5c ff ff ff    	mov    edx,DWORD PTR [rbp-0xa4]
    ebce:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ebd5:	48 89 ce             	mov    rsi,rcx
    ebd8:	48 89 c7             	mov    rdi,rax
    ebdb:	e8 e3 fc ff ff       	call   e8c3 <write_val>
    ebe0:	48 83 45 80 01       	add    QWORD PTR [rbp-0x80],0x1
    ebe5:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    ebe9:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    ebed:	48 39 45 80          	cmp    QWORD PTR [rbp-0x80],rax
    ebf1:	7c a4                	jl     eb97 <write_val+0x2d4>
    ebf3:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ebfa:	be 5d 00 00 00       	mov    esi,0x5d
    ebff:	48 89 c7             	mov    rdi,rax
    ec02:	e8 3b f7 ff ff       	call   e342 <sb_putc>
    ec07:	e9 df 03 00 00       	jmp    efeb <write_val+0x728>
    ec0c:	48 8b 85 60 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa0]
    ec13:	48 89 45 a8          	mov    QWORD PTR [rbp-0x58],rax
    ec17:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ec1e:	48 8d 15 53 1a 00 00 	lea    rdx,[rip+0x1a53]        # 10678 <_IO_stdin_used+0x678>
    ec25:	48 89 d6             	mov    rsi,rdx
    ec28:	48 89 c7             	mov    rdi,rax
    ec2b:	e8 0c f8 ff ff       	call   e43c <sb_str>
    ec30:	48 c7 45 88 00 00 00 	mov    QWORD PTR [rbp-0x78],0x0
    ec37:	00 
    ec38:	eb 49                	jmp    ec83 <write_val+0x3c0>
    ec3a:	48 83 7d 88 00       	cmp    QWORD PTR [rbp-0x78],0x0
    ec3f:	74 14                	je     ec55 <write_val+0x392>
    ec41:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ec48:	be 20 00 00 00       	mov    esi,0x20
    ec4d:	48 89 c7             	mov    rdi,rax
    ec50:	e8 ed f6 ff ff       	call   e342 <sb_putc>
    ec55:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    ec59:	48 8b 55 88          	mov    rdx,QWORD PTR [rbp-0x78]
    ec5d:	48 83 c2 02          	add    rdx,0x2
    ec61:	48 8b 4c d0 08       	mov    rcx,QWORD PTR [rax+rdx*8+0x8]
    ec66:	8b 95 5c ff ff ff    	mov    edx,DWORD PTR [rbp-0xa4]
    ec6c:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ec73:	48 89 ce             	mov    rsi,rcx
    ec76:	48 89 c7             	mov    rdi,rax
    ec79:	e8 45 fc ff ff       	call   e8c3 <write_val>
    ec7e:	48 83 45 88 01       	add    QWORD PTR [rbp-0x78],0x1
    ec83:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    ec87:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    ec8b:	48 39 45 88          	cmp    QWORD PTR [rbp-0x78],rax
    ec8f:	7c a9                	jl     ec3a <write_val+0x377>
    ec91:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ec98:	be 7d 00 00 00       	mov    esi,0x7d
    ec9d:	48 89 c7             	mov    rdi,rax
    eca0:	e8 9d f6 ff ff       	call   e342 <sb_putc>
    eca5:	e9 41 03 00 00       	jmp    efeb <write_val+0x728>
    ecaa:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ecb1:	48 8d 15 c0 19 00 00 	lea    rdx,[rip+0x19c0]        # 10678 <_IO_stdin_used+0x678>
    ecb8:	48 89 d6             	mov    rsi,rdx
    ecbb:	48 89 c7             	mov    rdi,rax
    ecbe:	e8 79 f7 ff ff       	call   e43c <sb_str>
    ecc3:	c7 85 74 ff ff ff 01 	mov    DWORD PTR [rbp-0x8c],0x1
    ecca:	00 00 00 
    eccd:	48 8b 85 60 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa0]
    ecd4:	48 8b 70 18          	mov    rsi,QWORD PTR [rax+0x18]
    ecd8:	48 8d 8d 74 ff ff ff 	lea    rcx,[rbp-0x8c]
    ecdf:	8b 95 5c ff ff ff    	mov    edx,DWORD PTR [rbp-0xa4]
    ece5:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ecec:	48 89 c7             	mov    rdi,rax
    ecef:	e8 6f f9 ff ff       	call   e663 <sb_write_hset>
    ecf4:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ecfb:	be 7d 00 00 00       	mov    esi,0x7d
    ed00:	48 89 c7             	mov    rdi,rax
    ed03:	e8 3a f6 ff ff       	call   e342 <sb_putc>
    ed08:	e9 de 02 00 00       	jmp    efeb <write_val+0x728>
    ed0d:	48 8b 85 60 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa0]
    ed14:	48 89 45 b0          	mov    QWORD PTR [rbp-0x50],rax
    ed18:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ed1f:	be 7b 00 00 00       	mov    esi,0x7b
    ed24:	48 89 c7             	mov    rdi,rax
    ed27:	e8 16 f6 ff ff       	call   e342 <sb_putc>
    ed2c:	48 c7 45 90 00 00 00 	mov    QWORD PTR [rbp-0x70],0x0
    ed33:	00 
    ed34:	e9 96 00 00 00       	jmp    edcf <write_val+0x50c>
    ed39:	48 83 7d 90 00       	cmp    QWORD PTR [rbp-0x70],0x0
    ed3e:	74 19                	je     ed59 <write_val+0x496>
    ed40:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ed47:	48 8d 15 0f 19 00 00 	lea    rdx,[rip+0x190f]        # 1065d <_IO_stdin_used+0x65d>
    ed4e:	48 89 d6             	mov    rsi,rdx
    ed51:	48 89 c7             	mov    rdi,rax
    ed54:	e8 e3 f6 ff ff       	call   e43c <sb_str>
    ed59:	48 8b 45 90          	mov    rax,QWORD PTR [rbp-0x70]
    ed5d:	48 8d 14 00          	lea    rdx,[rax+rax*1]
    ed61:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    ed65:	48 83 c2 02          	add    rdx,0x2
    ed69:	48 8b 4c d0 08       	mov    rcx,QWORD PTR [rax+rdx*8+0x8]
    ed6e:	8b 95 5c ff ff ff    	mov    edx,DWORD PTR [rbp-0xa4]
    ed74:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ed7b:	48 89 ce             	mov    rsi,rcx
    ed7e:	48 89 c7             	mov    rdi,rax
    ed81:	e8 3d fb ff ff       	call   e8c3 <write_val>
    ed86:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ed8d:	be 20 00 00 00       	mov    esi,0x20
    ed92:	48 89 c7             	mov    rdi,rax
    ed95:	e8 a8 f5 ff ff       	call   e342 <sb_putc>
    ed9a:	48 8b 45 90          	mov    rax,QWORD PTR [rbp-0x70]
    ed9e:	48 01 c0             	add    rax,rax
    eda1:	48 8d 50 01          	lea    rdx,[rax+0x1]
    eda5:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    eda9:	48 83 c2 02          	add    rdx,0x2
    edad:	48 8b 4c d0 08       	mov    rcx,QWORD PTR [rax+rdx*8+0x8]
    edb2:	8b 95 5c ff ff ff    	mov    edx,DWORD PTR [rbp-0xa4]
    edb8:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    edbf:	48 89 ce             	mov    rsi,rcx
    edc2:	48 89 c7             	mov    rdi,rax
    edc5:	e8 f9 fa ff ff       	call   e8c3 <write_val>
    edca:	48 83 45 90 01       	add    QWORD PTR [rbp-0x70],0x1
    edcf:	48 8b 45 b0          	mov    rax,QWORD PTR [rbp-0x50]
    edd3:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    edd7:	48 39 45 90          	cmp    QWORD PTR [rbp-0x70],rax
    eddb:	0f 8c 58 ff ff ff    	jl     ed39 <write_val+0x476>
    ede1:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ede8:	be 7d 00 00 00       	mov    esi,0x7d
    eded:	48 89 c7             	mov    rdi,rax
    edf0:	e8 4d f5 ff ff       	call   e342 <sb_putc>
    edf5:	e9 f1 01 00 00       	jmp    efeb <write_val+0x728>
    edfa:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ee01:	be 7b 00 00 00       	mov    esi,0x7b
    ee06:	48 89 c7             	mov    rdi,rax
    ee09:	e8 34 f5 ff ff       	call   e342 <sb_putc>
    ee0e:	c7 85 74 ff ff ff 01 	mov    DWORD PTR [rbp-0x8c],0x1
    ee15:	00 00 00 
    ee18:	48 8b 85 60 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa0]
    ee1f:	48 8b 70 18          	mov    rsi,QWORD PTR [rax+0x18]
    ee23:	48 8d 8d 74 ff ff ff 	lea    rcx,[rbp-0x8c]
    ee2a:	8b 95 5c ff ff ff    	mov    edx,DWORD PTR [rbp-0xa4]
    ee30:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ee37:	48 89 c7             	mov    rdi,rax
    ee3a:	e8 36 f6 ff ff       	call   e475 <sb_write_hmap>
    ee3f:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ee46:	be 7d 00 00 00       	mov    esi,0x7d
    ee4b:	48 89 c7             	mov    rdi,rax
    ee4e:	e8 ef f4 ff ff       	call   e342 <sb_putc>
    ee53:	e9 93 01 00 00       	jmp    efeb <write_val+0x728>
    ee58:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ee5f:	48 8d 15 12 18 00 00 	lea    rdx,[rip+0x1812]        # 10678 <_IO_stdin_used+0x678>
    ee66:	48 89 d6             	mov    rsi,rdx
    ee69:	48 89 c7             	mov    rdi,rax
    ee6c:	e8 cb f5 ff ff       	call   e43c <sb_str>
    ee71:	c7 85 74 ff ff ff 01 	mov    DWORD PTR [rbp-0x8c],0x1
    ee78:	00 00 00 
    ee7b:	48 8b 85 60 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa0]
    ee82:	48 8b 70 18          	mov    rsi,QWORD PTR [rax+0x18]
    ee86:	48 8d 8d 74 ff ff ff 	lea    rcx,[rbp-0x8c]
    ee8d:	8b 95 5c ff ff ff    	mov    edx,DWORD PTR [rbp-0xa4]
    ee93:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ee9a:	49 89 c8             	mov    r8,rcx
    ee9d:	89 d1                	mov    ecx,edx
    ee9f:	ba 00 00 00 00       	mov    edx,0x0
    eea4:	48 89 c7             	mov    rdi,rax
    eea7:	e8 1b f9 ff ff       	call   e7c7 <sb_write_tree>
    eeac:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    eeb3:	be 7d 00 00 00       	mov    esi,0x7d
    eeb8:	48 89 c7             	mov    rdi,rax
    eebb:	e8 82 f4 ff ff       	call   e342 <sb_putc>
    eec0:	e9 26 01 00 00       	jmp    efeb <write_val+0x728>
    eec5:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    eecc:	be 7b 00 00 00       	mov    esi,0x7b
    eed1:	48 89 c7             	mov    rdi,rax
    eed4:	e8 69 f4 ff ff       	call   e342 <sb_putc>
    eed9:	c7 85 74 ff ff ff 01 	mov    DWORD PTR [rbp-0x8c],0x1
    eee0:	00 00 00 
    eee3:	48 8b 85 60 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa0]
    eeea:	48 8b 70 18          	mov    rsi,QWORD PTR [rax+0x18]
    eeee:	48 8d 8d 74 ff ff ff 	lea    rcx,[rbp-0x8c]
    eef5:	8b 95 5c ff ff ff    	mov    edx,DWORD PTR [rbp-0xa4]
    eefb:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ef02:	49 89 c8             	mov    r8,rcx
    ef05:	89 d1                	mov    ecx,edx
    ef07:	ba 01 00 00 00       	mov    edx,0x1
    ef0c:	48 89 c7             	mov    rdi,rax
    ef0f:	e8 b3 f8 ff ff       	call   e7c7 <sb_write_tree>
    ef14:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ef1b:	be 7d 00 00 00       	mov    esi,0x7d
    ef20:	48 89 c7             	mov    rdi,rax
    ef23:	e8 1a f4 ff ff       	call   e342 <sb_putc>
    ef28:	e9 be 00 00 00       	jmp    efeb <write_val+0x728>
    ef2d:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ef34:	48 8d 15 40 17 00 00 	lea    rdx,[rip+0x1740]        # 1067b <_IO_stdin_used+0x67b>
    ef3b:	48 89 d6             	mov    rsi,rdx
    ef3e:	48 89 c7             	mov    rdi,rax
    ef41:	e8 f6 f4 ff ff       	call   e43c <sb_str>
    ef46:	e9 a0 00 00 00       	jmp    efeb <write_val+0x728>
    ef4b:	48 8b 85 60 ff ff ff 	mov    rax,QWORD PTR [rbp-0xa0]
    ef52:	48 89 45 98          	mov    QWORD PTR [rbp-0x68],rax
    ef56:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    ef5d:	be 23 00 00 00       	mov    esi,0x23
    ef62:	48 89 c7             	mov    rdi,rax
    ef65:	e8 d8 f3 ff ff       	call   e342 <sb_putc>
    ef6a:	48 8b 45 98          	mov    rax,QWORD PTR [rbp-0x68]
    ef6e:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    ef72:	48 89 c7             	mov    rdi,rax
    ef75:	e8 50 7e ff ff       	call   6dca <obj_type>
    ef7a:	83 f8 04             	cmp    eax,0x4
    ef7d:	75 2e                	jne    efad <write_val+0x6ea>
    ef7f:	48 8b 45 98          	mov    rax,QWORD PTR [rbp-0x68]
    ef83:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    ef87:	48 89 45 a0          	mov    QWORD PTR [rbp-0x60],rax
    ef8b:	48 8b 45 a0          	mov    rax,QWORD PTR [rbp-0x60]
    ef8f:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    ef93:	48 8b 45 a0          	mov    rax,QWORD PTR [rbp-0x60]
    ef97:	48 8b 48 18          	mov    rcx,QWORD PTR [rax+0x18]
    ef9b:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    efa2:	48 89 ce             	mov    rsi,rcx
    efa5:	48 89 c7             	mov    rdi,rax
    efa8:	e8 3b f4 ff ff       	call   e3e8 <sb_write>
    efad:	48 8b 45 98          	mov    rax,QWORD PTR [rbp-0x68]
    efb1:	48 8b 48 18          	mov    rcx,QWORD PTR [rax+0x18]
    efb5:	8b 95 5c ff ff ff    	mov    edx,DWORD PTR [rbp-0xa4]
    efbb:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    efc2:	48 89 ce             	mov    rsi,rcx
    efc5:	48 89 c7             	mov    rdi,rax
    efc8:	e8 f6 f8 ff ff       	call   e8c3 <write_val>
    efcd:	eb 1c                	jmp    efeb <write_val+0x728>
    efcf:	48 8b 85 68 ff ff ff 	mov    rax,QWORD PTR [rbp-0x98]
    efd6:	48 8d 15 a4 16 00 00 	lea    rdx,[rip+0x16a4]        # 10681 <_IO_stdin_used+0x681>
    efdd:	48 89 d6             	mov    rsi,rdx
    efe0:	48 89 c7             	mov    rdi,rax
    efe3:	e8 54 f4 ff ff       	call   e43c <sb_str>
    efe8:	eb 01                	jmp    efeb <write_val+0x728>
    efea:	90                   	nop
    efeb:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    efef:	64 48 2b 04 25 28 00 	sub    rax,QWORD PTR fs:0x28
    eff6:	00 00 
    eff8:	74 05                	je     efff <write_val+0x73c>
    effa:	e8 61 20 ff ff       	call   1060 <__stack_chk_fail@plt>
    efff:	c9                   	leave
    f000:	c3                   	ret

000000000000f001 <cljn_print>:
    f001:	f3 0f 1e fa          	endbr64
    f005:	55                   	push   rbp
    f006:	48 89 e5             	mov    rbp,rsp
    f009:	48 83 ec 30          	sub    rsp,0x30
    f00d:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    f011:	64 48 8b 04 25 28 00 	mov    rax,QWORD PTR fs:0x28
    f018:	00 00 
    f01a:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    f01e:	31 c0                	xor    eax,eax
    f020:	48 8d 45 e0          	lea    rax,[rbp-0x20]
    f024:	48 89 c7             	mov    rdi,rax
    f027:	e8 d4 f2 ff ff       	call   e300 <sb_init>
    f02c:	48 8b 4d d8          	mov    rcx,QWORD PTR [rbp-0x28]
    f030:	48 8d 45 e0          	lea    rax,[rbp-0x20]
    f034:	ba 00 00 00 00       	mov    edx,0x0
    f039:	48 89 ce             	mov    rsi,rcx
    f03c:	48 89 c7             	mov    rdi,rax
    f03f:	e8 7f f8 ff ff       	call   e8c3 <write_val>
    f044:	48 8b 0d f5 4f 00 00 	mov    rcx,QWORD PTR [rip+0x4ff5]        # 14040 <stdout@GLIBC_2.2.5>
    f04b:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    f04f:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    f053:	be 01 00 00 00       	mov    esi,0x1
    f058:	48 89 c7             	mov    rdi,rax
    f05b:	e8 b0 20 ff ff       	call   1110 <fwrite@plt>
    f060:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    f064:	48 89 c7             	mov    rdi,rax
    f067:	e8 d4 1f ff ff       	call   1040 <free@plt>
    f06c:	90                   	nop
    f06d:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    f071:	64 48 2b 04 25 28 00 	sub    rax,QWORD PTR fs:0x28
    f078:	00 00 
    f07a:	74 05                	je     f081 <cljn_print+0x80>
    f07c:	e8 df 1f ff ff       	call   1060 <__stack_chk_fail@plt>
    f081:	c9                   	leave
    f082:	c3                   	ret

000000000000f083 <cljn_to_str>:
    f083:	f3 0f 1e fa          	endbr64
    f087:	55                   	push   rbp
    f088:	48 89 e5             	mov    rbp,rsp
    f08b:	48 83 ec 40          	sub    rsp,0x40
    f08f:	48 89 7d c8          	mov    QWORD PTR [rbp-0x38],rdi
    f093:	64 48 8b 04 25 28 00 	mov    rax,QWORD PTR fs:0x28
    f09a:	00 00 
    f09c:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    f0a0:	31 c0                	xor    eax,eax
    f0a2:	48 8d 45 e0          	lea    rax,[rbp-0x20]
    f0a6:	48 89 c7             	mov    rdi,rax
    f0a9:	e8 52 f2 ff ff       	call   e300 <sb_init>
    f0ae:	48 8b 4d c8          	mov    rcx,QWORD PTR [rbp-0x38]
    f0b2:	48 8d 45 e0          	lea    rax,[rbp-0x20]
    f0b6:	ba 01 00 00 00       	mov    edx,0x1
    f0bb:	48 89 ce             	mov    rsi,rcx
    f0be:	48 89 c7             	mov    rdi,rax
    f0c1:	e8 fd f7 ff ff       	call   e8c3 <write_val>
    f0c6:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    f0ca:	48 89 c2             	mov    rdx,rax
    f0cd:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    f0d1:	48 89 d6             	mov    rsi,rdx
    f0d4:	48 89 c7             	mov    rdi,rax
    f0d7:	e8 79 85 ff ff       	call   7655 <cljn_str_from>
    f0dc:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    f0e0:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    f0e4:	48 89 c7             	mov    rdi,rax
    f0e7:	e8 54 1f ff ff       	call   1040 <free@plt>
    f0ec:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    f0f0:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    f0f4:	64 48 2b 14 25 28 00 	sub    rdx,QWORD PTR fs:0x28
    f0fb:	00 00 
    f0fd:	74 05                	je     f104 <cljn_to_str+0x81>
    f0ff:	e8 5c 1f ff ff       	call   1060 <__stack_chk_fail@plt>
    f104:	c9                   	leave
    f105:	c3                   	ret

000000000000f106 <cljn_str_concat>:
    f106:	f3 0f 1e fa          	endbr64
    f10a:	55                   	push   rbp
    f10b:	48 89 e5             	mov    rbp,rsp
    f10e:	48 83 ec 30          	sub    rsp,0x30
    f112:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    f116:	48 89 75 d0          	mov    QWORD PTR [rbp-0x30],rsi
    f11a:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    f11e:	48 89 c7             	mov    rdi,rax
    f121:	e8 a4 7c ff ff       	call   6dca <obj_type>
    f126:	83 f8 01             	cmp    eax,0x1
    f129:	75 11                	jne    f13c <cljn_str_concat+0x36>
    f12b:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    f12f:	48 89 c7             	mov    rdi,rax
    f132:	e8 93 7c ff ff       	call   6dca <obj_type>
    f137:	83 f8 01             	cmp    eax,0x1
    f13a:	74 0f                	je     f14b <cljn_str_concat+0x45>
    f13c:	48 8d 05 89 15 00 00 	lea    rax,[rip+0x1589]        # 106cc <_IO_stdin_used+0x6cc>
    f143:	48 89 c7             	mov    rdi,rax
    f146:	e8 43 7c ff ff       	call   6d8e <die>
    f14b:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    f14f:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    f153:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    f157:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    f15b:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    f15f:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    f163:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    f167:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    f16b:	48 01 d0             	add    rax,rdx
    f16e:	48 89 45 f0          	mov    QWORD PTR [rbp-0x10],rax
    f172:	be 01 00 00 00       	mov    esi,0x1
    f177:	bf 20 00 00 00       	mov    edi,0x20
    f17c:	e8 f2 7d ff ff       	call   6f73 <obj_alloc>
    f181:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    f185:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    f189:	48 8b 55 f0          	mov    rdx,QWORD PTR [rbp-0x10]
    f18d:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    f191:	48 83 7d f0 00       	cmp    QWORD PTR [rbp-0x10],0x0
    f196:	74 06                	je     f19e <cljn_str_concat+0x98>
    f198:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    f19c:	eb 05                	jmp    f1a3 <cljn_str_concat+0x9d>
    f19e:	b8 01 00 00 00       	mov    eax,0x1
    f1a3:	48 89 c7             	mov    rdi,rax
    f1a6:	e8 89 7b ff ff       	call   6d34 <xalloc>
    f1ab:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    f1af:	48 89 42 18          	mov    QWORD PTR [rdx+0x18],rax
    f1b3:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    f1b7:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    f1bb:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    f1bf:	48 89 45 e8          	mov    QWORD PTR [rbp-0x18],rax
    f1c3:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    f1c7:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    f1cb:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    f1cf:	48 8b 48 18          	mov    rcx,QWORD PTR [rax+0x18]
    f1d3:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    f1d7:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    f1db:	48 89 ce             	mov    rsi,rcx
    f1de:	48 89 c7             	mov    rdi,rax
    f1e1:	e8 da 1e ff ff       	call   10c0 <memcpy@plt>
    f1e6:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    f1ea:	48 8b 50 10          	mov    rdx,QWORD PTR [rax+0x10]
    f1ee:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    f1f2:	48 8b 40 18          	mov    rax,QWORD PTR [rax+0x18]
    f1f6:	48 8b 4d f8          	mov    rcx,QWORD PTR [rbp-0x8]
    f1fa:	48 8b 71 18          	mov    rsi,QWORD PTR [rcx+0x18]
    f1fe:	48 8b 4d e0          	mov    rcx,QWORD PTR [rbp-0x20]
    f202:	48 8b 49 10          	mov    rcx,QWORD PTR [rcx+0x10]
    f206:	48 01 f1             	add    rcx,rsi
    f209:	48 89 c6             	mov    rsi,rax
    f20c:	48 89 cf             	mov    rdi,rcx
    f20f:	e8 ac 1e ff ff       	call   10c0 <memcpy@plt>
    f214:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    f218:	c9                   	leave
    f219:	c3                   	ret

000000000000f21a <cljn_print_space>:
    f21a:	f3 0f 1e fa          	endbr64
    f21e:	55                   	push   rbp
    f21f:	48 89 e5             	mov    rbp,rsp
    f222:	48 8b 05 17 4e 00 00 	mov    rax,QWORD PTR [rip+0x4e17]        # 14040 <stdout@GLIBC_2.2.5>
    f229:	48 89 c6             	mov    rsi,rax
    f22c:	bf 20 00 00 00       	mov    edi,0x20
    f231:	e8 4a 1e ff ff       	call   1080 <fputc@plt>
    f236:	90                   	nop
    f237:	5d                   	pop    rbp
    f238:	c3                   	ret

000000000000f239 <cljn_print_newline>:
    f239:	f3 0f 1e fa          	endbr64
    f23d:	55                   	push   rbp
    f23e:	48 89 e5             	mov    rbp,rsp
    f241:	48 8b 05 f8 4d 00 00 	mov    rax,QWORD PTR [rip+0x4df8]        # 14040 <stdout@GLIBC_2.2.5>
    f248:	48 89 c6             	mov    rsi,rax
    f24b:	bf 0a 00 00 00       	mov    edi,0xa
    f250:	e8 2b 1e ff ff       	call   1080 <fputc@plt>
    f255:	90                   	nop
    f256:	5d                   	pop    rbp
    f257:	c3                   	ret

000000000000f258 <call_fn0>:
    f258:	f3 0f 1e fa          	endbr64
    f25c:	55                   	push   rbp
    f25d:	48 89 e5             	mov    rbp,rsp
    f260:	48 83 ec 10          	sub    rsp,0x10
    f264:	48 89 7d f8          	mov    QWORD PTR [rbp-0x8],rdi
    f268:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    f26c:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    f270:	48 89 c1             	mov    rcx,rax
    f273:	48 8b 05 06 4e 00 02 	mov    rax,QWORD PTR [rip+0x2004e06]        # 2014080 <gc_sp>
    f27a:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    f281:	00 
    f282:	48 8d 05 f7 4d 00 00 	lea    rax,[rip+0x4df7]        # 14080 <gc_stack>
    f289:	48 01 c2             	add    rdx,rax
    f28c:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    f290:	be 00 00 00 00       	mov    esi,0x0
    f295:	48 89 c7             	mov    rdi,rax
    f298:	ff d1                	call   rcx
    f29a:	c9                   	leave
    f29b:	c3                   	ret

000000000000f29c <call_fn1>:
    f29c:	f3 0f 1e fa          	endbr64
    f2a0:	55                   	push   rbp
    f2a1:	48 89 e5             	mov    rbp,rsp
    f2a4:	48 83 ec 20          	sub    rsp,0x20
    f2a8:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    f2ac:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    f2b0:	48 8b 05 c9 4d 00 02 	mov    rax,QWORD PTR [rip+0x2004dc9]        # 2014080 <gc_sp>
    f2b7:	48 8d 50 01          	lea    rdx,[rax+0x1]
    f2bb:	48 89 15 be 4d 00 02 	mov    QWORD PTR [rip+0x2004dbe],rdx        # 2014080 <gc_sp>
    f2c2:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    f2c9:	00 
    f2ca:	48 8d 15 af 4d 00 00 	lea    rdx,[rip+0x4daf]        # 14080 <gc_stack>
    f2d1:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    f2d5:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    f2d9:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    f2dd:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    f2e1:	48 89 c1             	mov    rcx,rax
    f2e4:	48 8b 05 95 4d 00 02 	mov    rax,QWORD PTR [rip+0x2004d95]        # 2014080 <gc_sp>
    f2eb:	48 83 e8 01          	sub    rax,0x1
    f2ef:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    f2f6:	00 
    f2f7:	48 8d 05 82 4d 00 00 	lea    rax,[rip+0x4d82]        # 14080 <gc_stack>
    f2fe:	48 01 c2             	add    rdx,rax
    f301:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    f305:	be 01 00 00 00       	mov    esi,0x1
    f30a:	48 89 c7             	mov    rdi,rax
    f30d:	ff d1                	call   rcx
    f30f:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    f313:	48 8b 05 66 4d 00 02 	mov    rax,QWORD PTR [rip+0x2004d66]        # 2014080 <gc_sp>
    f31a:	48 83 e8 01          	sub    rax,0x1
    f31e:	48 89 05 5b 4d 00 02 	mov    QWORD PTR [rip+0x2004d5b],rax        # 2014080 <gc_sp>
    f325:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    f329:	c9                   	leave
    f32a:	c3                   	ret

000000000000f32b <call_fn2>:
    f32b:	f3 0f 1e fa          	endbr64
    f32f:	55                   	push   rbp
    f330:	48 89 e5             	mov    rbp,rsp
    f333:	48 83 ec 30          	sub    rsp,0x30
    f337:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    f33b:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    f33f:	48 89 55 d8          	mov    QWORD PTR [rbp-0x28],rdx
    f343:	48 8b 05 36 4d 00 02 	mov    rax,QWORD PTR [rip+0x2004d36]        # 2014080 <gc_sp>
    f34a:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    f351:	00 
    f352:	48 8d 15 27 4d 00 00 	lea    rdx,[rip+0x4d27]        # 14080 <gc_stack>
    f359:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    f35d:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    f361:	48 8b 05 18 4d 00 02 	mov    rax,QWORD PTR [rip+0x2004d18]        # 2014080 <gc_sp>
    f368:	48 83 c0 01          	add    rax,0x1
    f36c:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    f373:	00 
    f374:	48 8d 15 05 4d 00 00 	lea    rdx,[rip+0x4d05]        # 14080 <gc_stack>
    f37b:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    f37f:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    f383:	48 8b 05 f6 4c 00 02 	mov    rax,QWORD PTR [rip+0x2004cf6]        # 2014080 <gc_sp>
    f38a:	48 83 c0 02          	add    rax,0x2
    f38e:	48 89 05 eb 4c 00 02 	mov    QWORD PTR [rip+0x2004ceb],rax        # 2014080 <gc_sp>
    f395:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    f399:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    f39d:	48 89 c1             	mov    rcx,rax
    f3a0:	48 8b 05 d9 4c 00 02 	mov    rax,QWORD PTR [rip+0x2004cd9]        # 2014080 <gc_sp>
    f3a7:	48 83 e8 02          	sub    rax,0x2
    f3ab:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    f3b2:	00 
    f3b3:	48 8d 05 c6 4c 00 00 	lea    rax,[rip+0x4cc6]        # 14080 <gc_stack>
    f3ba:	48 01 c2             	add    rdx,rax
    f3bd:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    f3c1:	be 02 00 00 00       	mov    esi,0x2
    f3c6:	48 89 c7             	mov    rdi,rax
    f3c9:	ff d1                	call   rcx
    f3cb:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    f3cf:	48 8b 05 aa 4c 00 02 	mov    rax,QWORD PTR [rip+0x2004caa]        # 2014080 <gc_sp>
    f3d6:	48 83 e8 02          	sub    rax,0x2
    f3da:	48 89 05 9f 4c 00 02 	mov    QWORD PTR [rip+0x2004c9f],rax        # 2014080 <gc_sp>
    f3e1:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    f3e5:	c9                   	leave
    f3e6:	c3                   	ret

000000000000f3e7 <call_fn3>:
    f3e7:	f3 0f 1e fa          	endbr64
    f3eb:	55                   	push   rbp
    f3ec:	48 89 e5             	mov    rbp,rsp
    f3ef:	48 83 ec 30          	sub    rsp,0x30
    f3f3:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    f3f7:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    f3fb:	48 89 55 d8          	mov    QWORD PTR [rbp-0x28],rdx
    f3ff:	48 89 4d d0          	mov    QWORD PTR [rbp-0x30],rcx
    f403:	48 8b 05 76 4c 00 02 	mov    rax,QWORD PTR [rip+0x2004c76]        # 2014080 <gc_sp>
    f40a:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    f411:	00 
    f412:	48 8d 15 67 4c 00 00 	lea    rdx,[rip+0x4c67]        # 14080 <gc_stack>
    f419:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    f41d:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    f421:	48 8b 05 58 4c 00 02 	mov    rax,QWORD PTR [rip+0x2004c58]        # 2014080 <gc_sp>
    f428:	48 83 c0 01          	add    rax,0x1
    f42c:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    f433:	00 
    f434:	48 8d 15 45 4c 00 00 	lea    rdx,[rip+0x4c45]        # 14080 <gc_stack>
    f43b:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    f43f:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    f443:	48 8b 05 36 4c 00 02 	mov    rax,QWORD PTR [rip+0x2004c36]        # 2014080 <gc_sp>
    f44a:	48 83 c0 02          	add    rax,0x2
    f44e:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    f455:	00 
    f456:	48 8d 15 23 4c 00 00 	lea    rdx,[rip+0x4c23]        # 14080 <gc_stack>
    f45d:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    f461:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    f465:	48 8b 05 14 4c 00 02 	mov    rax,QWORD PTR [rip+0x2004c14]        # 2014080 <gc_sp>
    f46c:	48 83 c0 03          	add    rax,0x3
    f470:	48 89 05 09 4c 00 02 	mov    QWORD PTR [rip+0x2004c09],rax        # 2014080 <gc_sp>
    f477:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    f47b:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    f47f:	48 89 c1             	mov    rcx,rax
    f482:	48 8b 05 f7 4b 00 02 	mov    rax,QWORD PTR [rip+0x2004bf7]        # 2014080 <gc_sp>
    f489:	48 83 e8 03          	sub    rax,0x3
    f48d:	48 8d 14 c5 00 00 00 	lea    rdx,[rax*8+0x0]
    f494:	00 
    f495:	48 8d 05 e4 4b 00 00 	lea    rax,[rip+0x4be4]        # 14080 <gc_stack>
    f49c:	48 01 c2             	add    rdx,rax
    f49f:	48 8b 45 e8          	mov    rax,QWORD PTR [rbp-0x18]
    f4a3:	be 03 00 00 00       	mov    esi,0x3
    f4a8:	48 89 c7             	mov    rdi,rax
    f4ab:	ff d1                	call   rcx
    f4ad:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    f4b1:	48 8b 05 c8 4b 00 02 	mov    rax,QWORD PTR [rip+0x2004bc8]        # 2014080 <gc_sp>
    f4b8:	48 83 e8 03          	sub    rax,0x3
    f4bc:	48 89 05 bd 4b 00 02 	mov    QWORD PTR [rip+0x2004bbd],rax        # 2014080 <gc_sp>
    f4c3:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    f4c7:	c9                   	leave
    f4c8:	c3                   	ret

000000000000f4c9 <cljn_throw>:
    f4c9:	f3 0f 1e fa          	endbr64
    f4cd:	55                   	push   rbp
    f4ce:	48 89 e5             	mov    rbp,rsp
    f4d1:	48 83 ec 30          	sub    rsp,0x30
    f4d5:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    f4d9:	64 48 8b 04 25 28 00 	mov    rax,QWORD PTR fs:0x28
    f4e0:	00 00 
    f4e2:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    f4e6:	31 c0                	xor    eax,eax
    f4e8:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    f4ec:	48 89 05 2d 4b 00 00 	mov    QWORD PTR [rip+0x4b2d],rax        # 14020 <exception_value>
    f4f3:	48 8b 05 c6 4c 00 02 	mov    rax,QWORD PTR [rip+0x2004cc6]        # 20141c0 <handler_top>
    f4fa:	48 85 c0             	test   rax,rax
    f4fd:	0f 85 8d 00 00 00    	jne    f590 <cljn_throw+0xc7>
    f503:	48 8d 45 e0          	lea    rax,[rbp-0x20]
    f507:	48 89 c7             	mov    rdi,rax
    f50a:	e8 f1 ed ff ff       	call   e300 <sb_init>
    f50f:	48 8b 4d d8          	mov    rcx,QWORD PTR [rbp-0x28]
    f513:	48 8d 45 e0          	lea    rax,[rbp-0x20]
    f517:	ba 00 00 00 00       	mov    edx,0x0
    f51c:	48 89 ce             	mov    rsi,rcx
    f51f:	48 89 c7             	mov    rdi,rax
    f522:	e8 9c f3 ff ff       	call   e8c3 <write_val>
    f527:	48 8b 05 32 4b 00 00 	mov    rax,QWORD PTR [rip+0x4b32]        # 14060 <stderr@GLIBC_2.2.5>
    f52e:	48 89 c1             	mov    rcx,rax
    f531:	ba 1a 00 00 00       	mov    edx,0x1a
    f536:	be 01 00 00 00       	mov    esi,0x1
    f53b:	48 8d 05 a7 11 00 00 	lea    rax,[rip+0x11a7]        # 106e9 <_IO_stdin_used+0x6e9>
    f542:	48 89 c7             	mov    rdi,rax
    f545:	e8 c6 1b ff ff       	call   1110 <fwrite@plt>
    f54a:	48 8b 0d 0f 4b 00 00 	mov    rcx,QWORD PTR [rip+0x4b0f]        # 14060 <stderr@GLIBC_2.2.5>
    f551:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    f555:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    f559:	be 01 00 00 00       	mov    esi,0x1
    f55e:	48 89 c7             	mov    rdi,rax
    f561:	e8 aa 1b ff ff       	call   1110 <fwrite@plt>
    f566:	48 8b 05 f3 4a 00 00 	mov    rax,QWORD PTR [rip+0x4af3]        # 14060 <stderr@GLIBC_2.2.5>
    f56d:	48 89 c6             	mov    rsi,rax
    f570:	bf 0a 00 00 00       	mov    edi,0xa
    f575:	e8 06 1b ff ff       	call   1080 <fputc@plt>
    f57a:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    f57e:	48 89 c7             	mov    rdi,rax
    f581:	e8 ba 1a ff ff       	call   1040 <free@plt>
    f586:	bf 01 00 00 00       	mov    edi,0x1
    f58b:	e8 70 1b ff ff       	call   1100 <exit@plt>
    f590:	48 8b 05 29 4c 00 02 	mov    rax,QWORD PTR [rip+0x2004c29]        # 20141c0 <handler_top>
    f597:	be 01 00 00 00       	mov    esi,0x1
    f59c:	48 89 c7             	mov    rdi,rax
    f59f:	e8 4c 1b ff ff       	call   10f0 <longjmp@plt>

000000000000f5a4 <cljn_try>:
    f5a4:	f3 0f 1e fa          	endbr64
    f5a8:	55                   	push   rbp
    f5a9:	48 89 e5             	mov    rbp,rsp
    f5ac:	48 81 ec 20 01 00 00 	sub    rsp,0x120
    f5b3:	48 89 bd f8 fe ff ff 	mov    QWORD PTR [rbp-0x108],rdi
    f5ba:	48 89 b5 f0 fe ff ff 	mov    QWORD PTR [rbp-0x110],rsi
    f5c1:	48 89 95 e8 fe ff ff 	mov    QWORD PTR [rbp-0x118],rdx
    f5c8:	64 48 8b 04 25 28 00 	mov    rax,QWORD PTR fs:0x28
    f5cf:	00 00 
    f5d1:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    f5d5:	31 c0                	xor    eax,eax
    f5d7:	48 8b 05 e2 4b 00 02 	mov    rax,QWORD PTR [rip+0x2004be2]        # 20141c0 <handler_top>
    f5de:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    f5e2:	48 8b 05 97 4a 00 02 	mov    rax,QWORD PTR [rip+0x2004a97]        # 2014080 <gc_sp>
    f5e9:	48 89 45 e0          	mov    QWORD PTR [rbp-0x20],rax
    f5ed:	8b 05 a5 4a 00 02    	mov    eax,DWORD PTR [rip+0x2004aa5]        # 2014098 <gc_disabled>
    f5f3:	89 45 e8             	mov    DWORD PTR [rbp-0x18],eax
    f5f6:	48 8d 85 10 ff ff ff 	lea    rax,[rbp-0xf0]
    f5fd:	48 89 c7             	mov    rdi,rax
    f600:	e8 9b 1a ff ff       	call   10a0 <_setjmp@plt>
    f605:	f3 0f 1e fa          	endbr64
    f609:	85 c0                	test   eax,eax
    f60b:	75 34                	jne    f641 <cljn_try+0x9d>
    f60d:	48 8d 85 10 ff ff ff 	lea    rax,[rbp-0xf0]
    f614:	48 89 05 a5 4b 00 02 	mov    QWORD PTR [rip+0x2004ba5],rax        # 20141c0 <handler_top>
    f61b:	48 8b 85 f8 fe ff ff 	mov    rax,QWORD PTR [rbp-0x108]
    f622:	48 89 c7             	mov    rdi,rax
    f625:	e8 2e fc ff ff       	call   f258 <call_fn0>
    f62a:	48 89 85 00 ff ff ff 	mov    QWORD PTR [rbp-0x100],rax
    f631:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    f635:	48 89 05 84 4b 00 02 	mov    QWORD PTR [rip+0x2004b84],rax        # 20141c0 <handler_top>
    f63c:	e9 81 00 00 00       	jmp    f6c2 <cljn_try+0x11e>
    f641:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    f645:	48 89 05 74 4b 00 02 	mov    QWORD PTR [rip+0x2004b74],rax        # 20141c0 <handler_top>
    f64c:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    f650:	48 89 05 29 4a 00 02 	mov    QWORD PTR [rip+0x2004a29],rax        # 2014080 <gc_sp>
    f657:	8b 45 e8             	mov    eax,DWORD PTR [rbp-0x18]
    f65a:	89 05 38 4a 00 02    	mov    DWORD PTR [rip+0x2004a38],eax        # 2014098 <gc_disabled>
    f660:	48 8b 05 b9 49 00 00 	mov    rax,QWORD PTR [rip+0x49b9]        # 14020 <exception_value>
    f667:	48 89 85 08 ff ff ff 	mov    QWORD PTR [rbp-0xf8],rax
    f66e:	48 83 bd f0 fe ff ff 	cmp    QWORD PTR [rbp-0x110],0x2
    f675:	02 
    f676:	75 2a                	jne    f6a2 <cljn_try+0xfe>
    f678:	48 83 bd e8 fe ff ff 	cmp    QWORD PTR [rbp-0x118],0x2
    f67f:	02 
    f680:	74 0f                	je     f691 <cljn_try+0xed>
    f682:	48 8b 85 e8 fe ff ff 	mov    rax,QWORD PTR [rbp-0x118]
    f689:	48 89 c7             	mov    rdi,rax
    f68c:	e8 c7 fb ff ff       	call   f258 <call_fn0>
    f691:	48 8b 85 08 ff ff ff 	mov    rax,QWORD PTR [rbp-0xf8]
    f698:	48 89 c7             	mov    rdi,rax
    f69b:	e8 29 fe ff ff       	call   f4c9 <cljn_throw>
    f6a0:	eb 7e                	jmp    f720 <cljn_try+0x17c>
    f6a2:	48 8b 95 08 ff ff ff 	mov    rdx,QWORD PTR [rbp-0xf8]
    f6a9:	48 8b 85 f0 fe ff ff 	mov    rax,QWORD PTR [rbp-0x110]
    f6b0:	48 89 d6             	mov    rsi,rdx
    f6b3:	48 89 c7             	mov    rdi,rax
    f6b6:	e8 e1 fb ff ff       	call   f29c <call_fn1>
    f6bb:	48 89 85 00 ff ff ff 	mov    QWORD PTR [rbp-0x100],rax
    f6c2:	48 83 bd e8 fe ff ff 	cmp    QWORD PTR [rbp-0x118],0x2
    f6c9:	02 
    f6ca:	74 4d                	je     f719 <cljn_try+0x175>
    f6cc:	48 8b 05 ad 49 00 02 	mov    rax,QWORD PTR [rip+0x20049ad]        # 2014080 <gc_sp>
    f6d3:	48 8d 50 01          	lea    rdx,[rax+0x1]
    f6d7:	48 89 15 a2 49 00 02 	mov    QWORD PTR [rip+0x20049a2],rdx        # 2014080 <gc_sp>
    f6de:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    f6e5:	00 
    f6e6:	48 8d 15 93 49 00 00 	lea    rdx,[rip+0x4993]        # 14080 <gc_stack>
    f6ed:	48 8b 85 00 ff ff ff 	mov    rax,QWORD PTR [rbp-0x100]
    f6f4:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    f6f8:	48 8b 85 e8 fe ff ff 	mov    rax,QWORD PTR [rbp-0x118]
    f6ff:	48 89 c7             	mov    rdi,rax
    f702:	e8 51 fb ff ff       	call   f258 <call_fn0>
    f707:	48 8b 05 72 49 00 02 	mov    rax,QWORD PTR [rip+0x2004972]        # 2014080 <gc_sp>
    f70e:	48 83 e8 01          	sub    rax,0x1
    f712:	48 89 05 67 49 00 02 	mov    QWORD PTR [rip+0x2004967],rax        # 2014080 <gc_sp>
    f719:	48 8b 85 00 ff ff ff 	mov    rax,QWORD PTR [rbp-0x100]
    f720:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    f724:	64 48 2b 14 25 28 00 	sub    rdx,QWORD PTR fs:0x28
    f72b:	00 00 
    f72d:	74 05                	je     f734 <cljn_try+0x190>
    f72f:	e8 2c 19 ff ff       	call   1060 <__stack_chk_fail@plt>
    f734:	c9                   	leave
    f735:	c3                   	ret

000000000000f736 <gc_mark_exceptions>:
    f736:	f3 0f 1e fa          	endbr64
    f73a:	55                   	push   rbp
    f73b:	48 89 e5             	mov    rbp,rsp
    f73e:	48 8b 05 db 48 00 00 	mov    rax,QWORD PTR [rip+0x48db]        # 14020 <exception_value>
    f745:	48 89 c7             	mov    rdi,rax
    f748:	e8 75 79 ff ff       	call   70c2 <gc_mark>
    f74d:	90                   	nop
    f74e:	5d                   	pop    rbp
    f74f:	c3                   	ret

000000000000f750 <cljn_multi_register>:
    f750:	f3 0f 1e fa          	endbr64
    f754:	55                   	push   rbp
    f755:	48 89 e5             	mov    rbp,rsp
    f758:	48 83 ec 20          	sub    rsp,0x20
    f75c:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    f760:	48 89 75 e0          	mov    QWORD PTR [rbp-0x20],rsi
    f764:	bf 18 00 00 00       	mov    edi,0x18
    f769:	e8 c6 75 ff ff       	call   6d34 <xalloc>
    f76e:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    f772:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    f776:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    f77a:	48 89 10             	mov    QWORD PTR [rax],rdx
    f77d:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    f781:	48 8b 55 e0          	mov    rdx,QWORD PTR [rbp-0x20]
    f785:	48 89 50 08          	mov    QWORD PTR [rax+0x8],rdx
    f789:	48 8b 15 38 4a 00 02 	mov    rdx,QWORD PTR [rip+0x2004a38]        # 20141c8 <multi_table>
    f790:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    f794:	48 89 50 10          	mov    QWORD PTR [rax+0x10],rdx
    f798:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    f79c:	48 89 05 25 4a 00 02 	mov    QWORD PTR [rip+0x2004a25],rax        # 20141c8 <multi_table>
    f7a3:	90                   	nop
    f7a4:	c9                   	leave
    f7a5:	c3                   	ret

000000000000f7a6 <multi_dispatch_fn>:
    f7a6:	f3 0f 1e fa          	endbr64
    f7aa:	55                   	push   rbp
    f7ab:	48 89 e5             	mov    rbp,rsp
    f7ae:	48 89 7d e8          	mov    QWORD PTR [rbp-0x18],rdi
    f7b2:	48 8b 05 0f 4a 00 02 	mov    rax,QWORD PTR [rip+0x2004a0f]        # 20141c8 <multi_table>
    f7b9:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    f7bd:	eb 23                	jmp    f7e2 <multi_dispatch_fn+0x3c>
    f7bf:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    f7c3:	48 8b 00             	mov    rax,QWORD PTR [rax]
    f7c6:	48 39 45 e8          	cmp    QWORD PTR [rbp-0x18],rax
    f7ca:	75 0a                	jne    f7d6 <multi_dispatch_fn+0x30>
    f7cc:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    f7d0:	48 8b 40 08          	mov    rax,QWORD PTR [rax+0x8]
    f7d4:	eb 18                	jmp    f7ee <multi_dispatch_fn+0x48>
    f7d6:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    f7da:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    f7de:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    f7e2:	48 83 7d f8 00       	cmp    QWORD PTR [rbp-0x8],0x0
    f7e7:	75 d6                	jne    f7bf <multi_dispatch_fn+0x19>
    f7e9:	b8 02 00 00 00       	mov    eax,0x2
    f7ee:	5d                   	pop    rbp
    f7ef:	c3                   	ret

000000000000f7f0 <cljn_multi_call>:
    f7f0:	f3 0f 1e fa          	endbr64
    f7f4:	55                   	push   rbp
    f7f5:	48 89 e5             	mov    rbp,rsp
    f7f8:	48 83 ec 70          	sub    rsp,0x70
    f7fc:	48 89 7d a8          	mov    QWORD PTR [rbp-0x58],rdi
    f800:	48 89 75 a0          	mov    QWORD PTR [rbp-0x60],rsi
    f804:	48 89 55 98          	mov    QWORD PTR [rbp-0x68],rdx
    f808:	64 48 8b 04 25 28 00 	mov    rax,QWORD PTR fs:0x28
    f80f:	00 00 
    f811:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    f815:	31 c0                	xor    eax,eax
    f817:	48 8b 45 a0          	mov    rax,QWORD PTR [rbp-0x60]
    f81b:	48 89 45 c0          	mov    QWORD PTR [rbp-0x40],rax
    f81f:	48 8b 45 98          	mov    rax,QWORD PTR [rbp-0x68]
    f823:	48 89 45 c8          	mov    QWORD PTR [rbp-0x38],rax
    f827:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    f82b:	48 89 c7             	mov    rdi,rax
    f82e:	e8 73 ff ff ff       	call   f7a6 <multi_dispatch_fn>
    f833:	48 89 45 d0          	mov    QWORD PTR [rbp-0x30],rax
    f837:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    f83b:	48 89 c7             	mov    rdi,rax
    f83e:	e8 87 75 ff ff       	call   6dca <obj_type>
    f843:	83 f8 03             	cmp    eax,0x3
    f846:	74 2d                	je     f875 <cljn_multi_call+0x85>
    f848:	48 8b 05 11 48 00 00 	mov    rax,QWORD PTR [rip+0x4811]        # 14060 <stderr@GLIBC_2.2.5>
    f84f:	48 89 c1             	mov    rcx,rax
    f852:	ba 2b 00 00 00       	mov    edx,0x2b
    f857:	be 01 00 00 00       	mov    esi,0x1
    f85c:	48 8d 05 a5 0e 00 00 	lea    rax,[rip+0xea5]        # 10708 <_IO_stdin_used+0x708>
    f863:	48 89 c7             	mov    rdi,rax
    f866:	e8 a5 18 ff ff       	call   1110 <fwrite@plt>
    f86b:	bf 01 00 00 00       	mov    edi,0x1
    f870:	e8 8b 18 ff ff       	call   1100 <exit@plt>
    f875:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    f879:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    f87d:	49 89 c0             	mov    r8,rax
    f880:	48 8b 55 c8          	mov    rdx,QWORD PTR [rbp-0x38]
    f884:	48 8b 4d c0          	mov    rcx,QWORD PTR [rbp-0x40]
    f888:	48 8b 45 d0          	mov    rax,QWORD PTR [rbp-0x30]
    f88c:	48 89 ce             	mov    rsi,rcx
    f88f:	48 89 c7             	mov    rdi,rax
    f892:	41 ff d0             	call   r8
    f895:	48 89 45 d8          	mov    QWORD PTR [rbp-0x28],rax
    f899:	48 8b 05 e0 47 00 02 	mov    rax,QWORD PTR [rip+0x20047e0]        # 2014080 <gc_sp>
    f8a0:	48 8d 50 01          	lea    rdx,[rax+0x1]
    f8a4:	48 89 15 d5 47 00 02 	mov    QWORD PTR [rip+0x20047d5],rdx        # 2014080 <gc_sp>
    f8ab:	48 8d 0c c5 00 00 00 	lea    rcx,[rax*8+0x0]
    f8b2:	00 
    f8b3:	48 8d 15 c6 47 00 00 	lea    rdx,[rip+0x47c6]        # 14080 <gc_stack>
    f8ba:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    f8be:	48 89 04 11          	mov    QWORD PTR [rcx+rdx*1],rax
    f8c2:	48 8b 55 d8          	mov    rdx,QWORD PTR [rbp-0x28]
    f8c6:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    f8ca:	48 89 d6             	mov    rsi,rdx
    f8cd:	48 89 c7             	mov    rdi,rax
    f8d0:	e8 be c3 ff ff       	call   bc93 <cljn_lookup_method>
    f8d5:	48 89 45 b8          	mov    QWORD PTR [rbp-0x48],rax
    f8d9:	48 83 7d b8 02       	cmp    QWORD PTR [rbp-0x48],0x2
    f8de:	75 41                	jne    f921 <cljn_multi_call+0x131>
    f8e0:	48 8b 05 e9 48 00 02 	mov    rax,QWORD PTR [rip+0x20048e9]        # 20141d0 <cljn_default_kw>
    f8e7:	48 85 c0             	test   rax,rax
    f8ea:	75 1b                	jne    f907 <cljn_multi_call+0x117>
    f8ec:	be 07 00 00 00       	mov    esi,0x7
    f8f1:	48 8d 05 3c 0e 00 00 	lea    rax,[rip+0xe3c]        # 10734 <_IO_stdin_used+0x734>
    f8f8:	48 89 c7             	mov    rdi,rax
    f8fb:	e8 3e 82 ff ff       	call   7b3e <cljn_kw>
    f900:	48 89 05 c9 48 00 02 	mov    QWORD PTR [rip+0x20048c9],rax        # 20141d0 <cljn_default_kw>
    f907:	48 8b 15 c2 48 00 02 	mov    rdx,QWORD PTR [rip+0x20048c2]        # 20141d0 <cljn_default_kw>
    f90e:	48 8b 45 a8          	mov    rax,QWORD PTR [rbp-0x58]
    f912:	48 89 d6             	mov    rsi,rdx
    f915:	48 89 c7             	mov    rdi,rax
    f918:	e8 76 c3 ff ff       	call   bc93 <cljn_lookup_method>
    f91d:	48 89 45 b8          	mov    QWORD PTR [rbp-0x48],rax
    f921:	48 8b 05 58 47 00 02 	mov    rax,QWORD PTR [rip+0x2004758]        # 2014080 <gc_sp>
    f928:	48 83 e8 01          	sub    rax,0x1
    f92c:	48 89 05 4d 47 00 02 	mov    QWORD PTR [rip+0x200474d],rax        # 2014080 <gc_sp>
    f933:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    f937:	48 89 c7             	mov    rdi,rax
    f93a:	e8 8b 74 ff ff       	call   6dca <obj_type>
    f93f:	83 f8 03             	cmp    eax,0x3
    f942:	0f 84 8d 00 00 00    	je     f9d5 <cljn_multi_call+0x1e5>
    f948:	48 8b 05 11 47 00 00 	mov    rax,QWORD PTR [rip+0x4711]        # 14060 <stderr@GLIBC_2.2.5>
    f94f:	48 89 c1             	mov    rcx,rax
    f952:	ba 3b 00 00 00       	mov    edx,0x3b
    f957:	be 01 00 00 00       	mov    esi,0x1
    f95c:	48 8d 05 dd 0d 00 00 	lea    rax,[rip+0xddd]        # 10740 <_IO_stdin_used+0x740>
    f963:	48 89 c7             	mov    rdi,rax
    f966:	e8 a5 17 ff ff       	call   1110 <fwrite@plt>
    f96b:	48 8d 45 e0          	lea    rax,[rbp-0x20]
    f96f:	48 89 c7             	mov    rdi,rax
    f972:	e8 89 e9 ff ff       	call   e300 <sb_init>
    f977:	48 8b 4d d8          	mov    rcx,QWORD PTR [rbp-0x28]
    f97b:	48 8d 45 e0          	lea    rax,[rbp-0x20]
    f97f:	ba 00 00 00 00       	mov    edx,0x0
    f984:	48 89 ce             	mov    rsi,rcx
    f987:	48 89 c7             	mov    rdi,rax
    f98a:	e8 34 ef ff ff       	call   e8c3 <write_val>
    f98f:	48 8b 0d ca 46 00 00 	mov    rcx,QWORD PTR [rip+0x46ca]        # 14060 <stderr@GLIBC_2.2.5>
    f996:	48 8b 55 e8          	mov    rdx,QWORD PTR [rbp-0x18]
    f99a:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    f99e:	be 01 00 00 00       	mov    esi,0x1
    f9a3:	48 89 c7             	mov    rdi,rax
    f9a6:	e8 65 17 ff ff       	call   1110 <fwrite@plt>
    f9ab:	48 8b 45 e0          	mov    rax,QWORD PTR [rbp-0x20]
    f9af:	48 89 c7             	mov    rdi,rax
    f9b2:	e8 89 16 ff ff       	call   1040 <free@plt>
    f9b7:	48 8b 05 a2 46 00 00 	mov    rax,QWORD PTR [rip+0x46a2]        # 14060 <stderr@GLIBC_2.2.5>
    f9be:	48 89 c6             	mov    rsi,rax
    f9c1:	bf 0a 00 00 00       	mov    edi,0xa
    f9c6:	e8 b5 16 ff ff       	call   1080 <fputc@plt>
    f9cb:	bf 01 00 00 00       	mov    edi,0x1
    f9d0:	e8 2b 17 ff ff       	call   1100 <exit@plt>
    f9d5:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    f9d9:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    f9dd:	49 89 c0             	mov    r8,rax
    f9e0:	48 8b 55 c8          	mov    rdx,QWORD PTR [rbp-0x38]
    f9e4:	48 8b 4d c0          	mov    rcx,QWORD PTR [rbp-0x40]
    f9e8:	48 8b 45 b8          	mov    rax,QWORD PTR [rbp-0x48]
    f9ec:	48 89 ce             	mov    rsi,rcx
    f9ef:	48 89 c7             	mov    rdi,rax
    f9f2:	41 ff d0             	call   r8
    f9f5:	48 8b 55 f8          	mov    rdx,QWORD PTR [rbp-0x8]
    f9f9:	64 48 2b 14 25 28 00 	sub    rdx,QWORD PTR fs:0x28
    fa00:	00 00 
    fa02:	74 05                	je     fa09 <cljn_multi_call+0x219>
    fa04:	e8 57 16 ff ff       	call   1060 <__stack_chk_fail@plt>
    fa09:	c9                   	leave
    fa0a:	c3                   	ret

000000000000fa0b <gc_mark_multi>:
    fa0b:	f3 0f 1e fa          	endbr64
    fa0f:	55                   	push   rbp
    fa10:	48 89 e5             	mov    rbp,rsp
    fa13:	48 83 ec 10          	sub    rsp,0x10
    fa17:	48 8b 05 aa 47 00 02 	mov    rax,QWORD PTR [rip+0x20047aa]        # 20141c8 <multi_table>
    fa1e:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    fa22:	eb 1c                	jmp    fa40 <gc_mark_multi+0x35>
    fa24:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    fa28:	48 8b 40 08          	mov    rax,QWORD PTR [rax+0x8]
    fa2c:	48 89 c7             	mov    rdi,rax
    fa2f:	e8 8e 76 ff ff       	call   70c2 <gc_mark>
    fa34:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    fa38:	48 8b 40 10          	mov    rax,QWORD PTR [rax+0x10]
    fa3c:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    fa40:	48 83 7d f8 00       	cmp    QWORD PTR [rbp-0x8],0x0
    fa45:	75 dd                	jne    fa24 <gc_mark_multi+0x19>
    fa47:	48 8b 05 82 47 00 02 	mov    rax,QWORD PTR [rip+0x2004782]        # 20141d0 <cljn_default_kw>
    fa4e:	48 85 c0             	test   rax,rax
    fa51:	74 0f                	je     fa62 <gc_mark_multi+0x57>
    fa53:	48 8b 05 76 47 00 02 	mov    rax,QWORD PTR [rip+0x2004776]        # 20141d0 <cljn_default_kw>
    fa5a:	48 89 c7             	mov    rdi,rax
    fa5d:	e8 60 76 ff ff       	call   70c2 <gc_mark>
    fa62:	90                   	nop
    fa63:	c9                   	leave
    fa64:	c3                   	ret

000000000000fa65 <cljn_gc_live_objects>:
    fa65:	f3 0f 1e fa          	endbr64
    fa69:	55                   	push   rbp
    fa6a:	48 89 e5             	mov    rbp,rsp
    fa6d:	48 c7 45 f0 00 00 00 	mov    QWORD PTR [rbp-0x10],0x0
    fa74:	00 
    fa75:	48 8b 05 0c 46 00 02 	mov    rax,QWORD PTR [rip+0x200460c]        # 2014088 <all_objs>
    fa7c:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    fa80:	eb 11                	jmp    fa93 <cljn_gc_live_objects+0x2e>
    fa82:	48 83 45 f0 01       	add    QWORD PTR [rbp-0x10],0x1
    fa87:	48 8b 45 f8          	mov    rax,QWORD PTR [rbp-0x8]
    fa8b:	48 8b 40 08          	mov    rax,QWORD PTR [rax+0x8]
    fa8f:	48 89 45 f8          	mov    QWORD PTR [rbp-0x8],rax
    fa93:	48 83 7d f8 00       	cmp    QWORD PTR [rbp-0x8],0x0
    fa98:	75 e8                	jne    fa82 <cljn_gc_live_objects+0x1d>
    fa9a:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    fa9e:	5d                   	pop    rbp
    fa9f:	c3                   	ret

000000000000faa0 <cljn_gc_force>:
    faa0:	f3 0f 1e fa          	endbr64
    faa4:	55                   	push   rbp
    faa5:	48 89 e5             	mov    rbp,rsp
    faa8:	e8 3f 7b ff ff       	call   75ec <gc_collect>
    faad:	90                   	nop
    faae:	5d                   	pop    rbp
    faaf:	c3                   	ret

000000000000fab0 <__popcountdi2>:
    fab0:	f3 0f 1e fa          	endbr64
    fab4:	48 ba 55 55 55 55 55 	movabs rdx,0x5555555555555555
    fabb:	55 55 55 
    fabe:	48 89 f8             	mov    rax,rdi
    fac1:	48 d1 e8             	shr    rax,1
    fac4:	48 21 d0             	and    rax,rdx
    fac7:	48 29 c7             	sub    rdi,rax
    faca:	48 b8 33 33 33 33 33 	movabs rax,0x3333333333333333
    fad1:	33 33 33 
    fad4:	48 89 fa             	mov    rdx,rdi
    fad7:	48 c1 ef 02          	shr    rdi,0x2
    fadb:	48 21 c2             	and    rdx,rax
    fade:	48 21 c7             	and    rdi,rax
    fae1:	48 01 fa             	add    rdx,rdi
    fae4:	48 89 d0             	mov    rax,rdx
    fae7:	48 c1 e8 04          	shr    rax,0x4
    faeb:	48 01 d0             	add    rax,rdx
    faee:	48 ba 0f 0f 0f 0f 0f 	movabs rdx,0xf0f0f0f0f0f0f0f
    faf5:	0f 0f 0f 
    faf8:	48 21 d0             	and    rax,rdx
    fafb:	48 ba 01 01 01 01 01 	movabs rdx,0x101010101010101
    fb02:	01 01 01 
    fb05:	48 0f af c2          	imul   rax,rdx
    fb09:	48 c1 e8 38          	shr    rax,0x38
    fb0d:	c3                   	ret

Disassembly of section .fini:

000000000000fb10 <_fini>:
    fb10:	f3 0f 1e fa          	endbr64
    fb14:	48 83 ec 08          	sub    rsp,0x8
    fb18:	48 83 c4 08          	add    rsp,0x8
    fb1c:	c3                   	ret
