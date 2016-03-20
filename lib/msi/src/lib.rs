// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to msi.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "cdecl" {
    // pub fn MsiInvalidateFeatureCache();
}
extern "system" {
    // pub fn DllGetVersion();
    // pub fn Migrate10CachedPackagesA();
    // pub fn Migrate10CachedPackagesW();
    // pub fn MsiAdvertiseProductA();
    // pub fn MsiAdvertiseProductExA();
    // pub fn MsiAdvertiseProductExW();
    // pub fn MsiAdvertiseProductW();
    // pub fn MsiAdvertiseScriptA();
    // pub fn MsiAdvertiseScriptW();
    // pub fn MsiApplyMultiplePatchesA();
    // pub fn MsiApplyMultiplePatchesW();
    // pub fn MsiApplyPatchA();
    // pub fn MsiApplyPatchW();
    // pub fn MsiBeginTransactionA();
    // pub fn MsiBeginTransactionW();
    // pub fn MsiCloseAllHandles();
    // pub fn MsiCloseHandle();
    // pub fn MsiCollectUserInfoA();
    // pub fn MsiCollectUserInfoW();
    // pub fn MsiConfigureFeatureA();
    // pub fn MsiConfigureFeatureFromDescriptorA();
    // pub fn MsiConfigureFeatureFromDescriptorW();
    // pub fn MsiConfigureFeatureW();
    // pub fn MsiConfigureProductA();
    // pub fn MsiConfigureProductExA();
    // pub fn MsiConfigureProductExW();
    // pub fn MsiConfigureProductW();
    // pub fn MsiCreateAndVerifyInstallerDirectory();
    // pub fn MsiCreateRecord();
    // pub fn MsiCreateTransformSummaryInfoA();
    // pub fn MsiCreateTransformSummaryInfoW();
    // pub fn MsiDatabaseApplyTransformA();
    // pub fn MsiDatabaseApplyTransformW();
    // pub fn MsiDatabaseCommit();
    // pub fn MsiDatabaseExportA();
    // pub fn MsiDatabaseExportW();
    // pub fn MsiDatabaseGenerateTransformA();
    // pub fn MsiDatabaseGenerateTransformW();
    // pub fn MsiDatabaseGetPrimaryKeysA();
    // pub fn MsiDatabaseGetPrimaryKeysW();
    // pub fn MsiDatabaseImportA();
    // pub fn MsiDatabaseImportW();
    // pub fn MsiDatabaseIsTablePersistentA();
    // pub fn MsiDatabaseIsTablePersistentW();
    // pub fn MsiDatabaseMergeA();
    // pub fn MsiDatabaseMergeW();
    // pub fn MsiDatabaseOpenViewA();
    // pub fn MsiDatabaseOpenViewW();
    // pub fn MsiDecomposeDescriptorA();
    // pub fn MsiDecomposeDescriptorW();
    // pub fn MsiDeleteUserDataA();
    // pub fn MsiDeleteUserDataW();
    // pub fn MsiDetermineApplicablePatchesA();
    // pub fn MsiDetermineApplicablePatchesW();
    // pub fn MsiDeterminePatchSequenceA();
    // pub fn MsiDeterminePatchSequenceW();
    // pub fn MsiDoActionA();
    // pub fn MsiDoActionW();
    // pub fn MsiEnableLogA();
    // pub fn MsiEnableLogW();
    // pub fn MsiEnableUIPreview();
    // pub fn MsiEndTransaction();
    // pub fn MsiEnumClientsA();
    // pub fn MsiEnumClientsExA();
    // pub fn MsiEnumClientsExW();
    // pub fn MsiEnumClientsW();
    // pub fn MsiEnumComponentCostsA();
    // pub fn MsiEnumComponentCostsW();
    // pub fn MsiEnumComponentQualifiersA();
    // pub fn MsiEnumComponentQualifiersW();
    // pub fn MsiEnumComponentsA();
    // pub fn MsiEnumComponentsExA();
    // pub fn MsiEnumComponentsExW();
    // pub fn MsiEnumComponentsW();
    // pub fn MsiEnumFeaturesA();
    // pub fn MsiEnumFeaturesW();
    // pub fn MsiEnumPatchesA();
    // pub fn MsiEnumPatchesExA();
    // pub fn MsiEnumPatchesExW();
    // pub fn MsiEnumPatchesW();
    // pub fn MsiEnumProductsA();
    // pub fn MsiEnumProductsExA();
    // pub fn MsiEnumProductsExW();
    // pub fn MsiEnumProductsW();
    // pub fn MsiEnumRelatedProductsA();
    // pub fn MsiEnumRelatedProductsW();
    // pub fn MsiEvaluateConditionA();
    // pub fn MsiEvaluateConditionW();
    // pub fn MsiExtractPatchXMLDataA();
    // pub fn MsiExtractPatchXMLDataW();
    // pub fn MsiFormatRecordA();
    // pub fn MsiFormatRecordW();
    // pub fn MsiGetActiveDatabase();
    // pub fn MsiGetComponentPathA();
    // pub fn MsiGetComponentPathExA();
    // pub fn MsiGetComponentPathExW();
    // pub fn MsiGetComponentPathW();
    // pub fn MsiGetComponentStateA();
    // pub fn MsiGetComponentStateW();
    // pub fn MsiGetDatabaseState();
    // pub fn MsiGetFeatureCostA();
    // pub fn MsiGetFeatureCostW();
    // pub fn MsiGetFeatureInfoA();
    // pub fn MsiGetFeatureInfoW();
    // pub fn MsiGetFeatureStateA();
    // pub fn MsiGetFeatureStateW();
    // pub fn MsiGetFeatureUsageA();
    // pub fn MsiGetFeatureUsageW();
    // pub fn MsiGetFeatureValidStatesA();
    // pub fn MsiGetFeatureValidStatesW();
    // pub fn MsiGetFileHashA();
    // pub fn MsiGetFileHashW();
    // pub fn MsiGetFileSignatureInformationA();
    // pub fn MsiGetFileSignatureInformationW();
    // pub fn MsiGetFileVersionA();
    // pub fn MsiGetFileVersionW();
    // pub fn MsiGetLanguage();
    // pub fn MsiGetLastErrorRecord();
    // pub fn MsiGetMode();
    // pub fn MsiGetPatchFileListA();
    // pub fn MsiGetPatchFileListW();
    // pub fn MsiGetPatchInfoA();
    // pub fn MsiGetPatchInfoExA();
    // pub fn MsiGetPatchInfoExW();
    // pub fn MsiGetPatchInfoW();
    // pub fn MsiGetProductCodeA();
    // pub fn MsiGetProductCodeFromPackageCodeA();
    // pub fn MsiGetProductCodeFromPackageCodeW();
    // pub fn MsiGetProductCodeW();
    // pub fn MsiGetProductInfoA();
    // pub fn MsiGetProductInfoExA();
    // pub fn MsiGetProductInfoExW();
    // pub fn MsiGetProductInfoFromScriptA();
    // pub fn MsiGetProductInfoFromScriptW();
    // pub fn MsiGetProductInfoW();
    // pub fn MsiGetProductPropertyA();
    // pub fn MsiGetProductPropertyW();
    // pub fn MsiGetPropertyA();
    // pub fn MsiGetPropertyW();
    // pub fn MsiGetShortcutTargetA();
    // pub fn MsiGetShortcutTargetW();
    // pub fn MsiGetSourcePathA();
    // pub fn MsiGetSourcePathW();
    // pub fn MsiGetSummaryInformationA();
    // pub fn MsiGetSummaryInformationW();
    // pub fn MsiGetTargetPathA();
    // pub fn MsiGetTargetPathW();
    // pub fn MsiGetUserInfoA();
    // pub fn MsiGetUserInfoW();
    // pub fn MsiInstallMissingComponentA();
    // pub fn MsiInstallMissingComponentW();
    // pub fn MsiInstallMissingFileA();
    // pub fn MsiInstallMissingFileW();
    // pub fn MsiInstallProductA();
    // pub fn MsiInstallProductW();
    // pub fn MsiIsProductElevatedA();
    // pub fn MsiIsProductElevatedW();
    // pub fn MsiJoinTransaction();
    // pub fn MsiLoadStringA();
    // pub fn MsiLoadStringW();
    // pub fn MsiLocateComponentA();
    // pub fn MsiLocateComponentW();
    // pub fn MsiMessageBoxA();
    // pub fn MsiMessageBoxExA();
    // pub fn MsiMessageBoxExW();
    // pub fn MsiMessageBoxW();
    // pub fn MsiNotifySidChangeA();
    // pub fn MsiNotifySidChangeW();
    // pub fn MsiOpenDatabaseA();
    // pub fn MsiOpenDatabaseW();
    // pub fn MsiOpenPackageA();
    // pub fn MsiOpenPackageExA();
    // pub fn MsiOpenPackageExW();
    // pub fn MsiOpenPackageW();
    // pub fn MsiOpenProductA();
    // pub fn MsiOpenProductW();
    // pub fn MsiPreviewBillboardA();
    // pub fn MsiPreviewBillboardW();
    // pub fn MsiPreviewDialogA();
    // pub fn MsiPreviewDialogW();
    // pub fn MsiProcessAdvertiseScriptA();
    // pub fn MsiProcessAdvertiseScriptW();
    // pub fn MsiProcessMessage();
    // pub fn MsiProvideAssemblyA();
    // pub fn MsiProvideAssemblyW();
    // pub fn MsiProvideComponentA();
    // pub fn MsiProvideComponentFromDescriptorA();
    // pub fn MsiProvideComponentFromDescriptorW();
    // pub fn MsiProvideComponentW();
    // pub fn MsiProvideQualifiedComponentA();
    // pub fn MsiProvideQualifiedComponentExA();
    // pub fn MsiProvideQualifiedComponentExW();
    // pub fn MsiProvideQualifiedComponentW();
    // pub fn MsiQueryComponentStateA();
    // pub fn MsiQueryComponentStateW();
    // pub fn MsiQueryFeatureStateA();
    // pub fn MsiQueryFeatureStateExA();
    // pub fn MsiQueryFeatureStateExW();
    // pub fn MsiQueryFeatureStateFromDescriptorA();
    // pub fn MsiQueryFeatureStateFromDescriptorW();
    // pub fn MsiQueryFeatureStateW();
    // pub fn MsiQueryProductStateA();
    // pub fn MsiQueryProductStateW();
    // pub fn MsiRecordClearData();
    // pub fn MsiRecordDataSize();
    // pub fn MsiRecordGetFieldCount();
    // pub fn MsiRecordGetInteger();
    // pub fn MsiRecordGetStringA();
    // pub fn MsiRecordGetStringW();
    // pub fn MsiRecordIsNull();
    // pub fn MsiRecordReadStream();
    // pub fn MsiRecordSetInteger();
    // pub fn MsiRecordSetStreamA();
    // pub fn MsiRecordSetStreamW();
    // pub fn MsiRecordSetStringA();
    // pub fn MsiRecordSetStringW();
    // pub fn MsiReinstallFeatureA();
    // pub fn MsiReinstallFeatureFromDescriptorA();
    // pub fn MsiReinstallFeatureFromDescriptorW();
    // pub fn MsiReinstallFeatureW();
    // pub fn MsiReinstallProductA();
    // pub fn MsiReinstallProductW();
    // pub fn MsiRemovePatchesA();
    // pub fn MsiRemovePatchesW();
    // pub fn MsiSequenceA();
    // pub fn MsiSequenceW();
    // pub fn MsiSetComponentStateA();
    // pub fn MsiSetComponentStateW();
    // pub fn MsiSetExternalUIA();
    // pub fn MsiSetExternalUIRecord();
    // pub fn MsiSetExternalUIW();
    // pub fn MsiSetFeatureAttributesA();
    // pub fn MsiSetFeatureAttributesW();
    // pub fn MsiSetFeatureStateA();
    // pub fn MsiSetFeatureStateW();
    // pub fn MsiSetInstallLevel();
    // pub fn MsiSetInternalUI();
    // pub fn MsiSetMode();
    // pub fn MsiSetOfflineContextW();
    // pub fn MsiSetPropertyA();
    // pub fn MsiSetPropertyW();
    // pub fn MsiSetTargetPathA();
    // pub fn MsiSetTargetPathW();
    // pub fn MsiSourceListAddMediaDiskA();
    // pub fn MsiSourceListAddMediaDiskW();
    // pub fn MsiSourceListAddSourceA();
    // pub fn MsiSourceListAddSourceExA();
    // pub fn MsiSourceListAddSourceExW();
    // pub fn MsiSourceListAddSourceW();
    // pub fn MsiSourceListClearAllA();
    // pub fn MsiSourceListClearAllExA();
    // pub fn MsiSourceListClearAllExW();
    // pub fn MsiSourceListClearAllW();
    // pub fn MsiSourceListClearMediaDiskA();
    // pub fn MsiSourceListClearMediaDiskW();
    // pub fn MsiSourceListClearSourceA();
    // pub fn MsiSourceListClearSourceW();
    // pub fn MsiSourceListEnumMediaDisksA();
    // pub fn MsiSourceListEnumMediaDisksW();
    // pub fn MsiSourceListEnumSourcesA();
    // pub fn MsiSourceListEnumSourcesW();
    // pub fn MsiSourceListForceResolutionA();
    // pub fn MsiSourceListForceResolutionExA();
    // pub fn MsiSourceListForceResolutionExW();
    // pub fn MsiSourceListForceResolutionW();
    // pub fn MsiSourceListGetInfoA();
    // pub fn MsiSourceListGetInfoW();
    // pub fn MsiSourceListSetInfoA();
    // pub fn MsiSourceListSetInfoW();
    // pub fn MsiSummaryInfoGetPropertyA();
    // pub fn MsiSummaryInfoGetPropertyCount();
    // pub fn MsiSummaryInfoGetPropertyW();
    // pub fn MsiSummaryInfoPersist();
    // pub fn MsiSummaryInfoSetPropertyA();
    // pub fn MsiSummaryInfoSetPropertyW();
    // pub fn MsiUseFeatureA();
    // pub fn MsiUseFeatureExA();
    // pub fn MsiUseFeatureExW();
    // pub fn MsiUseFeatureW();
    // pub fn MsiVerifyDiskSpace();
    // pub fn MsiVerifyPackageA();
    // pub fn MsiVerifyPackageW();
    // pub fn MsiViewClose();
    // pub fn MsiViewExecute();
    // pub fn MsiViewFetch();
    // pub fn MsiViewGetColumnInfo();
    // pub fn MsiViewGetErrorA();
    // pub fn MsiViewGetErrorW();
    // pub fn MsiViewModify();
}
