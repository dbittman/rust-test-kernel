
#define WOB ((0 << 4) | (15 & 0x0F))

int cx=0, cy=0;

void putch(int x, int y, char c)
{
	char *scr = (char *)0xB8000 + (y * 80 + x) * 2;
	*scr++ = c;
	*scr = (char)WOB;
}

void putstr(char *str)
{
	while(*str) {
		putch(cx, cy, *str);
		cx++;
		if(cx >= 80)
			cx=0, cy++;
		str++;
	}
}

void kmain()
{
	short *screen = (short *)0xB8000;
	for(int i=0;i<25;i++)
		for(int j=0;j<80;j++)
			*screen++ = 0;
	putstr("Hello World!");
	for(;;)
		asm volatile("hlt");
}

