#include <stdbool.h>
#include <stdint.h>

#include <windows.h>

typedef struct ISystem ISystem;

typedef struct ISystem_VTable
{
	void (*dtor)(ISystem* this);
	bool (*Initialize)(ISystem* this);
	void (*Shutdown)(ISystem* this);
	const char* (*GetName)(ISystem* this);
	uint32_t (*GetVersion)(ISystem* this);
} ISystem_VTable;

struct ISystem
{
	ISystem_VTable* vfptr;
	ISystem_VTable vtable;
};

void ISystem_dtor(struct ISystem* this)
{

}

bool ISystem_Initialize(struct ISystem* this)
{
	return true;
}

void ISystem_Shutdown(struct ISystem* this)
{
	
}

const char* ISystem_GetName(struct ISystem* this)
{
	return "Test";
}

uint32_t ISystem_GetVersion(struct ISystem* this)
{
	return 69;
}

__declspec(dllexport) ISystem* CreateInterface(void)
{
	HANDLE base = GetModuleHandle("Base.dll");
	void* (*Base_Alloc)(intptr_t, intptr_t);
	Base_Alloc = (void*(*)(intptr_t, intptr_t))GetProcAddress(base, "?Base_Alloc@@YAPEAX_J0@Z");

	// must be allocated by Base_Alloc, or it'll crash the launcher after the app runs (which would probably be fine)
	ISystem* system = Base_Alloc(sizeof(ISystem), 8);
	
	system->vfptr = &system->vtable;
	system->vfptr->dtor = ISystem_dtor;
	system->vfptr->Initialize = ISystem_Initialize;
	system->vfptr->Shutdown = ISystem_Shutdown;
	system->vfptr->GetName = ISystem_GetName;
	system->vfptr->GetVersion = ISystem_GetVersion;

	return system;
}

BOOL WINAPI DllMain(HINSTANCE hinstDLL, DWORD fdwReason, LPVOID lpvReserved)
{
	return true;
}

