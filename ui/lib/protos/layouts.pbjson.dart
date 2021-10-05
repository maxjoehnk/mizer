///
//  Generated code. Do not modify.
//  source: layouts.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use layoutResponseDescriptor instead')
const LayoutResponse$json = const {
  '1': 'LayoutResponse',
};

/// Descriptor for `LayoutResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List layoutResponseDescriptor = $convert.base64Decode('Cg5MYXlvdXRSZXNwb25zZQ==');
@$core.Deprecated('Use getLayoutsRequestDescriptor instead')
const GetLayoutsRequest$json = const {
  '1': 'GetLayoutsRequest',
};

/// Descriptor for `GetLayoutsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getLayoutsRequestDescriptor = $convert.base64Decode('ChFHZXRMYXlvdXRzUmVxdWVzdA==');
@$core.Deprecated('Use addLayoutRequestDescriptor instead')
const AddLayoutRequest$json = const {
  '1': 'AddLayoutRequest',
  '2': const [
    const {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
  ],
};

/// Descriptor for `AddLayoutRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addLayoutRequestDescriptor = $convert.base64Decode('ChBBZGRMYXlvdXRSZXF1ZXN0EhIKBG5hbWUYASABKAlSBG5hbWU=');
@$core.Deprecated('Use removeLayoutRequestDescriptor instead')
const RemoveLayoutRequest$json = const {
  '1': 'RemoveLayoutRequest',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
  ],
};

/// Descriptor for `RemoveLayoutRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeLayoutRequestDescriptor = $convert.base64Decode('ChNSZW1vdmVMYXlvdXRSZXF1ZXN0Eg4KAmlkGAEgASgJUgJpZA==');
@$core.Deprecated('Use renameLayoutRequestDescriptor instead')
const RenameLayoutRequest$json = const {
  '1': 'RenameLayoutRequest',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    const {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
  ],
};

/// Descriptor for `RenameLayoutRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List renameLayoutRequestDescriptor = $convert.base64Decode('ChNSZW5hbWVMYXlvdXRSZXF1ZXN0Eg4KAmlkGAEgASgJUgJpZBISCgRuYW1lGAIgASgJUgRuYW1l');
@$core.Deprecated('Use renameControlRequestDescriptor instead')
const RenameControlRequest$json = const {
  '1': 'RenameControlRequest',
  '2': const [
    const {'1': 'layout_id', '3': 1, '4': 1, '5': 9, '10': 'layoutId'},
    const {'1': 'control_id', '3': 2, '4': 1, '5': 9, '10': 'controlId'},
    const {'1': 'name', '3': 3, '4': 1, '5': 9, '10': 'name'},
  ],
};

/// Descriptor for `RenameControlRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List renameControlRequestDescriptor = $convert.base64Decode('ChRSZW5hbWVDb250cm9sUmVxdWVzdBIbCglsYXlvdXRfaWQYASABKAlSCGxheW91dElkEh0KCmNvbnRyb2xfaWQYAiABKAlSCWNvbnRyb2xJZBISCgRuYW1lGAMgASgJUgRuYW1l');
@$core.Deprecated('Use moveControlRequestDescriptor instead')
const MoveControlRequest$json = const {
  '1': 'MoveControlRequest',
  '2': const [
    const {'1': 'layout_id', '3': 1, '4': 1, '5': 9, '10': 'layoutId'},
    const {'1': 'control_id', '3': 2, '4': 1, '5': 9, '10': 'controlId'},
    const {'1': 'position', '3': 3, '4': 1, '5': 11, '6': '.mizer.ControlPosition', '10': 'position'},
  ],
};

/// Descriptor for `MoveControlRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List moveControlRequestDescriptor = $convert.base64Decode('ChJNb3ZlQ29udHJvbFJlcXVlc3QSGwoJbGF5b3V0X2lkGAEgASgJUghsYXlvdXRJZBIdCgpjb250cm9sX2lkGAIgASgJUgljb250cm9sSWQSMgoIcG9zaXRpb24YAyABKAsyFi5taXplci5Db250cm9sUG9zaXRpb25SCHBvc2l0aW9u');
@$core.Deprecated('Use updateControlRequestDescriptor instead')
const UpdateControlRequest$json = const {
  '1': 'UpdateControlRequest',
  '2': const [
    const {'1': 'layout_id', '3': 1, '4': 1, '5': 9, '10': 'layoutId'},
    const {'1': 'control_id', '3': 2, '4': 1, '5': 9, '10': 'controlId'},
    const {'1': 'decorations', '3': 3, '4': 1, '5': 11, '6': '.mizer.ControlDecorations', '10': 'decorations'},
  ],
};

/// Descriptor for `UpdateControlRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateControlRequestDescriptor = $convert.base64Decode('ChRVcGRhdGVDb250cm9sUmVxdWVzdBIbCglsYXlvdXRfaWQYASABKAlSCGxheW91dElkEh0KCmNvbnRyb2xfaWQYAiABKAlSCWNvbnRyb2xJZBI7CgtkZWNvcmF0aW9ucxgDIAEoCzIZLm1pemVyLkNvbnRyb2xEZWNvcmF0aW9uc1ILZGVjb3JhdGlvbnM=');
@$core.Deprecated('Use removeControlRequestDescriptor instead')
const RemoveControlRequest$json = const {
  '1': 'RemoveControlRequest',
  '2': const [
    const {'1': 'layout_id', '3': 1, '4': 1, '5': 9, '10': 'layoutId'},
    const {'1': 'control_id', '3': 2, '4': 1, '5': 9, '10': 'controlId'},
  ],
};

/// Descriptor for `RemoveControlRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeControlRequestDescriptor = $convert.base64Decode('ChRSZW1vdmVDb250cm9sUmVxdWVzdBIbCglsYXlvdXRfaWQYASABKAlSCGxheW91dElkEh0KCmNvbnRyb2xfaWQYAiABKAlSCWNvbnRyb2xJZA==');
@$core.Deprecated('Use addControlRequestDescriptor instead')
const AddControlRequest$json = const {
  '1': 'AddControlRequest',
  '2': const [
    const {'1': 'layout_id', '3': 1, '4': 1, '5': 9, '10': 'layoutId'},
    const {'1': 'node_type', '3': 2, '4': 1, '5': 14, '6': '.mizer.Node.NodeType', '10': 'nodeType'},
    const {'1': 'position', '3': 3, '4': 1, '5': 11, '6': '.mizer.ControlPosition', '10': 'position'},
  ],
};

/// Descriptor for `AddControlRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addControlRequestDescriptor = $convert.base64Decode('ChFBZGRDb250cm9sUmVxdWVzdBIbCglsYXlvdXRfaWQYASABKAlSCGxheW91dElkEjEKCW5vZGVfdHlwZRgCIAEoDjIULm1pemVyLk5vZGUuTm9kZVR5cGVSCG5vZGVUeXBlEjIKCHBvc2l0aW9uGAMgASgLMhYubWl6ZXIuQ29udHJvbFBvc2l0aW9uUghwb3NpdGlvbg==');
@$core.Deprecated('Use addExistingControlRequestDescriptor instead')
const AddExistingControlRequest$json = const {
  '1': 'AddExistingControlRequest',
  '2': const [
    const {'1': 'layout_id', '3': 1, '4': 1, '5': 9, '10': 'layoutId'},
    const {'1': 'node', '3': 2, '4': 1, '5': 9, '10': 'node'},
    const {'1': 'position', '3': 3, '4': 1, '5': 11, '6': '.mizer.ControlPosition', '10': 'position'},
  ],
};

/// Descriptor for `AddExistingControlRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addExistingControlRequestDescriptor = $convert.base64Decode('ChlBZGRFeGlzdGluZ0NvbnRyb2xSZXF1ZXN0EhsKCWxheW91dF9pZBgBIAEoCVIIbGF5b3V0SWQSEgoEbm9kZRgCIAEoCVIEbm9kZRIyCghwb3NpdGlvbhgDIAEoCzIWLm1pemVyLkNvbnRyb2xQb3NpdGlvblIIcG9zaXRpb24=');
@$core.Deprecated('Use layoutsDescriptor instead')
const Layouts$json = const {
  '1': 'Layouts',
  '2': const [
    const {'1': 'layouts', '3': 1, '4': 3, '5': 11, '6': '.mizer.Layout', '10': 'layouts'},
  ],
};

/// Descriptor for `Layouts`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List layoutsDescriptor = $convert.base64Decode('CgdMYXlvdXRzEicKB2xheW91dHMYASADKAsyDS5taXplci5MYXlvdXRSB2xheW91dHM=');
@$core.Deprecated('Use layoutDescriptor instead')
const Layout$json = const {
  '1': 'Layout',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 9, '10': 'id'},
    const {'1': 'controls', '3': 2, '4': 3, '5': 11, '6': '.mizer.LayoutControl', '10': 'controls'},
  ],
};

/// Descriptor for `Layout`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List layoutDescriptor = $convert.base64Decode('CgZMYXlvdXQSDgoCaWQYASABKAlSAmlkEjAKCGNvbnRyb2xzGAIgAygLMhQubWl6ZXIuTGF5b3V0Q29udHJvbFIIY29udHJvbHM=');
@$core.Deprecated('Use layoutControlDescriptor instead')
const LayoutControl$json = const {
  '1': 'LayoutControl',
  '2': const [
    const {'1': 'node', '3': 1, '4': 1, '5': 9, '10': 'node'},
    const {'1': 'position', '3': 2, '4': 1, '5': 11, '6': '.mizer.ControlPosition', '10': 'position'},
    const {'1': 'size', '3': 3, '4': 1, '5': 11, '6': '.mizer.ControlSize', '10': 'size'},
    const {'1': 'label', '3': 4, '4': 1, '5': 9, '10': 'label'},
    const {'1': 'decoration', '3': 5, '4': 1, '5': 11, '6': '.mizer.ControlDecorations', '10': 'decoration'},
  ],
};

/// Descriptor for `LayoutControl`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List layoutControlDescriptor = $convert.base64Decode('Cg1MYXlvdXRDb250cm9sEhIKBG5vZGUYASABKAlSBG5vZGUSMgoIcG9zaXRpb24YAiABKAsyFi5taXplci5Db250cm9sUG9zaXRpb25SCHBvc2l0aW9uEiYKBHNpemUYAyABKAsyEi5taXplci5Db250cm9sU2l6ZVIEc2l6ZRIUCgVsYWJlbBgEIAEoCVIFbGFiZWwSOQoKZGVjb3JhdGlvbhgFIAEoCzIZLm1pemVyLkNvbnRyb2xEZWNvcmF0aW9uc1IKZGVjb3JhdGlvbg==');
@$core.Deprecated('Use controlPositionDescriptor instead')
const ControlPosition$json = const {
  '1': 'ControlPosition',
  '2': const [
    const {'1': 'x', '3': 1, '4': 1, '5': 4, '10': 'x'},
    const {'1': 'y', '3': 2, '4': 1, '5': 4, '10': 'y'},
  ],
};

/// Descriptor for `ControlPosition`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List controlPositionDescriptor = $convert.base64Decode('Cg9Db250cm9sUG9zaXRpb24SDAoBeBgBIAEoBFIBeBIMCgF5GAIgASgEUgF5');
@$core.Deprecated('Use controlSizeDescriptor instead')
const ControlSize$json = const {
  '1': 'ControlSize',
  '2': const [
    const {'1': 'width', '3': 1, '4': 1, '5': 4, '10': 'width'},
    const {'1': 'height', '3': 2, '4': 1, '5': 4, '10': 'height'},
  ],
};

/// Descriptor for `ControlSize`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List controlSizeDescriptor = $convert.base64Decode('CgtDb250cm9sU2l6ZRIUCgV3aWR0aBgBIAEoBFIFd2lkdGgSFgoGaGVpZ2h0GAIgASgEUgZoZWlnaHQ=');
@$core.Deprecated('Use controlDecorationsDescriptor instead')
const ControlDecorations$json = const {
  '1': 'ControlDecorations',
  '2': const [
    const {'1': 'hasColor', '3': 1, '4': 1, '5': 8, '10': 'hasColor'},
    const {'1': 'color', '3': 2, '4': 1, '5': 11, '6': '.mizer.Color', '10': 'color'},
  ],
};

/// Descriptor for `ControlDecorations`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List controlDecorationsDescriptor = $convert.base64Decode('ChJDb250cm9sRGVjb3JhdGlvbnMSGgoIaGFzQ29sb3IYASABKAhSCGhhc0NvbG9yEiIKBWNvbG9yGAIgASgLMgwubWl6ZXIuQ29sb3JSBWNvbG9y');
@$core.Deprecated('Use colorDescriptor instead')
const Color$json = const {
  '1': 'Color',
  '2': const [
    const {'1': 'red', '3': 1, '4': 1, '5': 1, '10': 'red'},
    const {'1': 'green', '3': 2, '4': 1, '5': 1, '10': 'green'},
    const {'1': 'blue', '3': 3, '4': 1, '5': 1, '10': 'blue'},
  ],
};

/// Descriptor for `Color`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List colorDescriptor = $convert.base64Decode('CgVDb2xvchIQCgNyZWQYASABKAFSA3JlZBIUCgVncmVlbhgCIAEoAVIFZ3JlZW4SEgoEYmx1ZRgDIAEoAVIEYmx1ZQ==');
