# dma_cs2moneyreveal
A money revealer for CS2 through DMA/Memflow using bytepatching

## How does this work?
Simply put, CS2 checks if the current game is an HLTV replay/stream.  
If that's the case, it will show the money on the scoreboard.  
[This was originally found by rushensky](https://www.unknowncheats.me/forum/3710688-post110.html), so huge shoutout to them.

What we are doing here is patching the `is_hltv` function in `client.dll` to always return true.  
That will cause the game to display the enemy team's money on the scoreboard.

In detail, we patch the first 3 bytes (`48 83 EC`) of the function into this (`B0 01 C3`):

```
    0xB0 0x01   | MOV AL,1  // Move 1 into the AL subregister
    0xC3        | RET       // Return
```