# Memory Table
Memory tables are structured tables of data that live separately from any specific block or collection. They are used for storing data that is shared between blocks, and are integrated into RSIS's internal API. They are similar to the idea of a database, that other blocks can access and modify.

## Buffering
By default, Memory Tables are setup as double-buffered. One buffer is active, and one buffer is inactive. During a simulation the inactive copy can be updated in chunks by blocks accessing the API. Once the inactive buffer has been fully updated, a block can trigger a switch between the two buffers.

### Binary
Memory tables can be loaded directly from binary data. This option is disabled if elements that are dynamically allocated are present within the table.

### MessagePack
With MessagePack enabled, memory tables can be loaded via directly via MessagePack.

## Block Access

-[] multi-threading?
