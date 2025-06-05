/******************************************************************************
 *
 * Name: acrust.h - OS specific defines, etc. for acpi-bindings. Adapted from aclinux.h
 *
 *****************************************************************************/

/*
 * Copyright (C) 2000 - 2023, Intel Corp.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions, and the following disclaimer,
 *    without modification.
 * 2. Redistributions in binary form must reproduce at minimum a disclaimer
 *    substantially similar to the "NO WARRANTY" disclaimer below
 *    ("Disclaimer") and any redistribution must be conditioned upon
 *    including a substantially similar Disclaimer requirement for further
 *    binary redistribution.
 * 3. Neither the names of the above-listed copyright holders nor the names
 *    of any contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * Alternatively, this software may be distributed under the terms of the
 * GNU General Public License ("GPL") version 2 as published by the Free
 * Software Foundation.
 *
 * NO WARRANTY
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDERS OR CONTRIBUTORS BE LIABLE FOR SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
 * IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGES.
 */

#ifndef __ACRUST_H__
#define __ACRUST_H__

/* Common (in-kernel/user-space) ACPICA configuration */

#define ACPI_USE_DO_WHILE_0
#define ACPI_IGNORE_PACKAGE_RESOLUTION_ERRORS

#define ACPI_USE_SYSTEM_INTTYPES
#define ACPI_USE_GPE_POLLING

/* Kernel specific ACPICA configuration */

#ifdef CONFIG_ACPI_REDUCED_HARDWARE_ONLY
#define ACPI_REDUCED_HARDWARE 1
#endif

#ifdef CONFIG_ACPI_DEBUGGER
#define ACPI_DEBUGGER
#endif

#ifdef CONFIG_ACPI_DEBUG
#define ACPI_MUTEX_DEBUG
#endif

#define ACPI_INIT_FUNCTION __init

/* Use a specific bugging default separate from ACPICA */

#undef ACPI_DEBUG_DEFAULT
#define ACPI_DEBUG_DEFAULT (ACPI_LV_INFO | ACPI_LV_REPAIR)

#ifndef CONFIG_ACPI

/* External globals for __KERNEL__, stubs is needed */

#define ACPI_GLOBAL(t, a)
#define ACPI_INIT_GLOBAL(t, a, b)

/* Generating stubs for configurable ACPICA macros */

#define ACPI_NO_MEM_ALLOCATIONS

/* Generating stubs for configurable ACPICA functions */

#define ACPI_NO_ERROR_MESSAGES
#undef ACPI_DEBUG_OUTPUT

/* External interface for __KERNEL__, stub is needed */

#define ACPI_EXTERNAL_RETURN_STATUS(Prototype) \
    static ACPI_INLINE Prototype { return (AE_NOT_CONFIGURED); }
#define ACPI_EXTERNAL_RETURN_OK(Prototype) \
    static ACPI_INLINE Prototype { return (AE_OK); }
#define ACPI_EXTERNAL_RETURN_VOID(Prototype) \
    static ACPI_INLINE Prototype { return; }
#define ACPI_EXTERNAL_RETURN_UINT32(Prototype) \
    static ACPI_INLINE Prototype { return (0); }
#define ACPI_EXTERNAL_RETURN_PTR(Prototype) \
    static ACPI_INLINE Prototype { return (NULL); }

#endif /* CONFIG_ACPI */

/* Host-dependent types and defines for in-kernel ACPICA */

#define ACPI_MACHINE_WIDTH BITS_PER_LONG
#define ACPI_USE_NATIVE_MATH64
#define ACPI_EXPORT_SYMBOL(symbol) EXPORT_SYMBOL(symbol);
#define strtoul simple_strtoul

#define ACPI_CACHE_T struct kmem_cache
#define ACPI_SPINLOCK spinlock_t *
#define ACPI_CPU_FLAGS unsigned long

#define ACPI_UINTPTR_T uintptr_t

#define ACPI_TO_INTEGER(p) ((uintptr_t)(p))
#define ACPI_OFFSET(d, f) offsetof(d, f)

/*
 * Overrides for in-kernel ACPICA
 */
#define ACPI_USE_ALTERNATE_PROTOTYPE_AcpiOsInitialize
#define ACPI_USE_ALTERNATE_PROTOTYPE_AcpiOsTerminate
#define ACPI_USE_ALTERNATE_PROTOTYPE_AcpiOsAllocate
#define ACPI_USE_ALTERNATE_PROTOTYPE_AcpiOsAllocateZeroed
#define ACPI_USE_ALTERNATE_PROTOTYPE_AcpiOsFree
#define ACPI_USE_ALTERNATE_PROTOTYPE_AcpiOsAcquireObject
#define ACPI_USE_ALTERNATE_PROTOTYPE_AcpiOsGetThreadId
#define ACPI_USE_ALTERNATE_PROTOTYPE_AcpiOsCreateLock

/*
 * OSL interfaces used by debugger/disassembler
 */
#define ACPI_USE_ALTERNATE_PROTOTYPE_AcpiOsReadable
#define ACPI_USE_ALTERNATE_PROTOTYPE_AcpiOsWritable
#define ACPI_USE_ALTERNATE_PROTOTYPE_AcpiOsInitializeDebugger
#define ACPI_USE_ALTERNATE_PROTOTYPE_AcpiOsTerminateDebugger

/*
 * OSL interfaces used by utilities
 */
#define ACPI_USE_ALTERNATE_PROTOTYPE_AcpiOsRedirectOutput
#define ACPI_USE_ALTERNATE_PROTOTYPE_AcpiOsGetTableByName
#define ACPI_USE_ALTERNATE_PROTOTYPE_AcpiOsGetTableByIndex
#define ACPI_USE_ALTERNATE_PROTOTYPE_AcpiOsGetTableByAddress
#define ACPI_USE_ALTERNATE_PROTOTYPE_AcpiOsOpenDirectory
#define ACPI_USE_ALTERNATE_PROTOTYPE_AcpiOsGetNextFilename
#define ACPI_USE_ALTERNATE_PROTOTYPE_AcpiOsCloseDirectory

#define ACPI_MSG_ERROR KERN_ERR ""
#define ACPI_MSG_EXCEPTION KERN_ERR "@FATAL"
#define ACPI_MSG_WARNING KERN_WARNING ""
#define ACPI_MSG_INFO KERN_INFO ""

#define ACPI_MSG_BIOS_ERROR KERN_ERR "ACPI BIOS Error (bug): "
#define ACPI_MSG_BIOS_WARNING KERN_WARNING "ACPI BIOS Warning (bug): "

/*
 * Linux wants to use designated initializers for function pointer structs.
 */
#define ACPI_STRUCT_INIT(field, value) .field = value

#endif /* __ACRUST_H__ */
