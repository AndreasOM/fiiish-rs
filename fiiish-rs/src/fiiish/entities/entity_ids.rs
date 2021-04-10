
/* Note: use the enum below instead
pub const ENTITYID_BUBBLE			:u32	= 0xeb20f1f7;
pub const ENTITYID_PICKUPCOIN		:u32	= 0xe4c651aa;
pub const ENTITYID_PICKUPRAIN		:u32	= 0x06fd4c5a;
pub const ENTITYID_PICKUPEXPLOSION	:u32	= 0xf75fd92f;
pub const ENTITYID_PICKUPMAGNET		:u32	= 0x235a41dd;
pub const ENTITYID_COIN				:u32	= 0x5569975d;
pub const ENTITYID_ROCKA			:u32	= 0xd058353c;
pub const ENTITYID_ROCKB			:u32	= 0x49516486;
pub const ENTITYID_ROCKC			:u32	= 0x3e565410;
pub const ENTITYID_ROCKD			:u32	= 0xa032c1b3;
pub const ENTITYID_ROCKE			:u32	= 0xd735f125;
pub const ENTITYID_ROCKF			:u32	= 0x4e3ca09f;
pub const ENTITYID_SEAWEEDA			:u32	= 0x6fe93bef;
pub const ENTITYID_SEAWEEDB			:u32	= 0xf6e06a55;
pub const ENTITYID_SEAWEEDC			:u32	= 0x81e75ac3;
pub const ENTITYID_SEAWEEDD			:u32	= 0x1f83cf60;
pub const ENTITYID_SEAWEEDE			:u32	= 0x6884fff6;
pub const ENTITYID_SEAWEEDF			:u32	= 0xf18dae4c;
pub const ENTITYID_SEAWEEDG			:u32	= 0x868a9eda;
pub const ENTITYID_CORALAA			:u32	= 0xd9862e2c;
pub const ENTITYID_CORALAB			:u32	= 0x408f7f96;
pub const ENTITYID_CORALBA			:u32	= 0xf2ab7def;
pub const ENTITYID_CORALBB			:u32	= 0x6ba22c55;
pub const ENTITYID_CORALBC			:u32	= 0x1ca51cc3;
pub const ENTITYID_CORALCA			:u32	= 0xebb04cae;
pub const ENTITYID_CORALCB			:u32	= 0x72b91d14;
pub const ENTITYID_ANCHOR			:u32	= 0x6751117d;
pub const ENTITYID_BLOCK1X1			:u32	= 0xe37ce24d;
*/

#[derive(Debug,Copy,Clone,Eq,PartialEq,Hash)]
pub enum EntityId {
	NONE				= 0x00000000,
	FISHSWIM			= 0x12345678, // :TODO: fix me to use correct CRC
	BUBBLE				= 0xeb20f1f7,
	PICKUPCOIN			= 0xe4c651aa,
	PICKUPRAIN			= 0x06fd4c5a,
	PICKUPEXPLOSION		= 0xf75fd92f,
	PICKUPMAGNET		= 0x235a41dd,
	COIN				= 0x5569975d,
	ROCKA				= 0xd058353c,
	ROCKB				= 0x49516486,
	ROCKC				= 0x3e565410,
	ROCKD				= 0xa032c1b3,
	ROCKE				= 0xd735f125,
	ROCKF				= 0x4e3ca09f,
	SEAWEEDA			= 0x6fe93bef,
	SEAWEEDB			= 0xf6e06a55,
	SEAWEEDC			= 0x81e75ac3,
	SEAWEEDD			= 0x1f83cf60,
	SEAWEEDE			= 0x6884fff6,
	SEAWEEDF			= 0xf18dae4c,
	SEAWEEDG			= 0x868a9eda,
	CORALAA				= 0xd9862e2c,
	CORALAB				= 0x408f7f96,
	CORALBA				= 0xf2ab7def,
	CORALBB				= 0x6ba22c55,
	CORALBC				= 0x1ca51cc3,
	CORALCA				= 0xebb04cae,
	CORALCB				= 0x72b91d14,
	ANCHOR				= 0x6751117d,
	FERRIS				= 0xd5cd9198,
	HEART				= 0x60ecc3e6,
	FIIISH				= 0xbd1ed3a1,
	BLOCK1X1			= 0xe37ce24d,
}
