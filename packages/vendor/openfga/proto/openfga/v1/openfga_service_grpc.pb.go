// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.3.0
// - protoc             (unknown)
// source: openfga/v1/openfga_service.proto

package openfgav1

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

const (
	OpenFGAService_Read_FullMethodName                    = "/openfga.v1.OpenFGAService/Read"
	OpenFGAService_Write_FullMethodName                   = "/openfga.v1.OpenFGAService/Write"
	OpenFGAService_Check_FullMethodName                   = "/openfga.v1.OpenFGAService/Check"
	OpenFGAService_Expand_FullMethodName                  = "/openfga.v1.OpenFGAService/Expand"
	OpenFGAService_ReadAuthorizationModels_FullMethodName = "/openfga.v1.OpenFGAService/ReadAuthorizationModels"
	OpenFGAService_ReadAuthorizationModel_FullMethodName  = "/openfga.v1.OpenFGAService/ReadAuthorizationModel"
	OpenFGAService_WriteAuthorizationModel_FullMethodName = "/openfga.v1.OpenFGAService/WriteAuthorizationModel"
	OpenFGAService_WriteAssertions_FullMethodName         = "/openfga.v1.OpenFGAService/WriteAssertions"
	OpenFGAService_ReadAssertions_FullMethodName          = "/openfga.v1.OpenFGAService/ReadAssertions"
	OpenFGAService_ReadChanges_FullMethodName             = "/openfga.v1.OpenFGAService/ReadChanges"
	OpenFGAService_CreateStore_FullMethodName             = "/openfga.v1.OpenFGAService/CreateStore"
	OpenFGAService_UpdateStore_FullMethodName             = "/openfga.v1.OpenFGAService/UpdateStore"
	OpenFGAService_DeleteStore_FullMethodName             = "/openfga.v1.OpenFGAService/DeleteStore"
	OpenFGAService_GetStore_FullMethodName                = "/openfga.v1.OpenFGAService/GetStore"
	OpenFGAService_ListStores_FullMethodName              = "/openfga.v1.OpenFGAService/ListStores"
	OpenFGAService_StreamedListObjects_FullMethodName     = "/openfga.v1.OpenFGAService/StreamedListObjects"
	OpenFGAService_ListObjects_FullMethodName             = "/openfga.v1.OpenFGAService/ListObjects"
	OpenFGAService_ListUsers_FullMethodName               = "/openfga.v1.OpenFGAService/ListUsers"
)

// OpenFGAServiceClient is the client API for OpenFGAService service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type OpenFGAServiceClient interface {
	Read(ctx context.Context, in *ReadRequest, opts ...grpc.CallOption) (*ReadResponse, error)
	Write(ctx context.Context, in *WriteRequest, opts ...grpc.CallOption) (*WriteResponse, error)
	Check(ctx context.Context, in *CheckRequest, opts ...grpc.CallOption) (*CheckResponse, error)
	Expand(ctx context.Context, in *ExpandRequest, opts ...grpc.CallOption) (*ExpandResponse, error)
	ReadAuthorizationModels(ctx context.Context, in *ReadAuthorizationModelsRequest, opts ...grpc.CallOption) (*ReadAuthorizationModelsResponse, error)
	ReadAuthorizationModel(ctx context.Context, in *ReadAuthorizationModelRequest, opts ...grpc.CallOption) (*ReadAuthorizationModelResponse, error)
	WriteAuthorizationModel(ctx context.Context, in *WriteAuthorizationModelRequest, opts ...grpc.CallOption) (*WriteAuthorizationModelResponse, error)
	WriteAssertions(ctx context.Context, in *WriteAssertionsRequest, opts ...grpc.CallOption) (*WriteAssertionsResponse, error)
	ReadAssertions(ctx context.Context, in *ReadAssertionsRequest, opts ...grpc.CallOption) (*ReadAssertionsResponse, error)
	ReadChanges(ctx context.Context, in *ReadChangesRequest, opts ...grpc.CallOption) (*ReadChangesResponse, error)
	CreateStore(ctx context.Context, in *CreateStoreRequest, opts ...grpc.CallOption) (*CreateStoreResponse, error)
	UpdateStore(ctx context.Context, in *UpdateStoreRequest, opts ...grpc.CallOption) (*UpdateStoreResponse, error)
	DeleteStore(ctx context.Context, in *DeleteStoreRequest, opts ...grpc.CallOption) (*DeleteStoreResponse, error)
	GetStore(ctx context.Context, in *GetStoreRequest, opts ...grpc.CallOption) (*GetStoreResponse, error)
	ListStores(ctx context.Context, in *ListStoresRequest, opts ...grpc.CallOption) (*ListStoresResponse, error)
	StreamedListObjects(ctx context.Context, in *StreamedListObjectsRequest, opts ...grpc.CallOption) (OpenFGAService_StreamedListObjectsClient, error)
	ListObjects(ctx context.Context, in *ListObjectsRequest, opts ...grpc.CallOption) (*ListObjectsResponse, error)
	ListUsers(ctx context.Context, in *ListUsersRequest, opts ...grpc.CallOption) (*ListUsersResponse, error)
}

type openFGAServiceClient struct {
	cc grpc.ClientConnInterface
}

func NewOpenFGAServiceClient(cc grpc.ClientConnInterface) OpenFGAServiceClient {
	return &openFGAServiceClient{cc}
}

func (c *openFGAServiceClient) Read(ctx context.Context, in *ReadRequest, opts ...grpc.CallOption) (*ReadResponse, error) {
	out := new(ReadResponse)
	err := c.cc.Invoke(ctx, OpenFGAService_Read_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *openFGAServiceClient) Write(ctx context.Context, in *WriteRequest, opts ...grpc.CallOption) (*WriteResponse, error) {
	out := new(WriteResponse)
	err := c.cc.Invoke(ctx, OpenFGAService_Write_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *openFGAServiceClient) Check(ctx context.Context, in *CheckRequest, opts ...grpc.CallOption) (*CheckResponse, error) {
	out := new(CheckResponse)
	err := c.cc.Invoke(ctx, OpenFGAService_Check_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *openFGAServiceClient) Expand(ctx context.Context, in *ExpandRequest, opts ...grpc.CallOption) (*ExpandResponse, error) {
	out := new(ExpandResponse)
	err := c.cc.Invoke(ctx, OpenFGAService_Expand_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *openFGAServiceClient) ReadAuthorizationModels(ctx context.Context, in *ReadAuthorizationModelsRequest, opts ...grpc.CallOption) (*ReadAuthorizationModelsResponse, error) {
	out := new(ReadAuthorizationModelsResponse)
	err := c.cc.Invoke(ctx, OpenFGAService_ReadAuthorizationModels_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *openFGAServiceClient) ReadAuthorizationModel(ctx context.Context, in *ReadAuthorizationModelRequest, opts ...grpc.CallOption) (*ReadAuthorizationModelResponse, error) {
	out := new(ReadAuthorizationModelResponse)
	err := c.cc.Invoke(ctx, OpenFGAService_ReadAuthorizationModel_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *openFGAServiceClient) WriteAuthorizationModel(ctx context.Context, in *WriteAuthorizationModelRequest, opts ...grpc.CallOption) (*WriteAuthorizationModelResponse, error) {
	out := new(WriteAuthorizationModelResponse)
	err := c.cc.Invoke(ctx, OpenFGAService_WriteAuthorizationModel_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *openFGAServiceClient) WriteAssertions(ctx context.Context, in *WriteAssertionsRequest, opts ...grpc.CallOption) (*WriteAssertionsResponse, error) {
	out := new(WriteAssertionsResponse)
	err := c.cc.Invoke(ctx, OpenFGAService_WriteAssertions_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *openFGAServiceClient) ReadAssertions(ctx context.Context, in *ReadAssertionsRequest, opts ...grpc.CallOption) (*ReadAssertionsResponse, error) {
	out := new(ReadAssertionsResponse)
	err := c.cc.Invoke(ctx, OpenFGAService_ReadAssertions_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *openFGAServiceClient) ReadChanges(ctx context.Context, in *ReadChangesRequest, opts ...grpc.CallOption) (*ReadChangesResponse, error) {
	out := new(ReadChangesResponse)
	err := c.cc.Invoke(ctx, OpenFGAService_ReadChanges_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *openFGAServiceClient) CreateStore(ctx context.Context, in *CreateStoreRequest, opts ...grpc.CallOption) (*CreateStoreResponse, error) {
	out := new(CreateStoreResponse)
	err := c.cc.Invoke(ctx, OpenFGAService_CreateStore_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *openFGAServiceClient) UpdateStore(ctx context.Context, in *UpdateStoreRequest, opts ...grpc.CallOption) (*UpdateStoreResponse, error) {
	out := new(UpdateStoreResponse)
	err := c.cc.Invoke(ctx, OpenFGAService_UpdateStore_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *openFGAServiceClient) DeleteStore(ctx context.Context, in *DeleteStoreRequest, opts ...grpc.CallOption) (*DeleteStoreResponse, error) {
	out := new(DeleteStoreResponse)
	err := c.cc.Invoke(ctx, OpenFGAService_DeleteStore_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *openFGAServiceClient) GetStore(ctx context.Context, in *GetStoreRequest, opts ...grpc.CallOption) (*GetStoreResponse, error) {
	out := new(GetStoreResponse)
	err := c.cc.Invoke(ctx, OpenFGAService_GetStore_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *openFGAServiceClient) ListStores(ctx context.Context, in *ListStoresRequest, opts ...grpc.CallOption) (*ListStoresResponse, error) {
	out := new(ListStoresResponse)
	err := c.cc.Invoke(ctx, OpenFGAService_ListStores_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *openFGAServiceClient) StreamedListObjects(ctx context.Context, in *StreamedListObjectsRequest, opts ...grpc.CallOption) (OpenFGAService_StreamedListObjectsClient, error) {
	stream, err := c.cc.NewStream(ctx, &OpenFGAService_ServiceDesc.Streams[0], OpenFGAService_StreamedListObjects_FullMethodName, opts...)
	if err != nil {
		return nil, err
	}
	x := &openFGAServiceStreamedListObjectsClient{stream}
	if err := x.ClientStream.SendMsg(in); err != nil {
		return nil, err
	}
	if err := x.ClientStream.CloseSend(); err != nil {
		return nil, err
	}
	return x, nil
}

type OpenFGAService_StreamedListObjectsClient interface {
	Recv() (*StreamedListObjectsResponse, error)
	grpc.ClientStream
}

type openFGAServiceStreamedListObjectsClient struct {
	grpc.ClientStream
}

func (x *openFGAServiceStreamedListObjectsClient) Recv() (*StreamedListObjectsResponse, error) {
	m := new(StreamedListObjectsResponse)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

func (c *openFGAServiceClient) ListObjects(ctx context.Context, in *ListObjectsRequest, opts ...grpc.CallOption) (*ListObjectsResponse, error) {
	out := new(ListObjectsResponse)
	err := c.cc.Invoke(ctx, OpenFGAService_ListObjects_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *openFGAServiceClient) ListUsers(ctx context.Context, in *ListUsersRequest, opts ...grpc.CallOption) (*ListUsersResponse, error) {
	out := new(ListUsersResponse)
	err := c.cc.Invoke(ctx, OpenFGAService_ListUsers_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// OpenFGAServiceServer is the server API for OpenFGAService service.
// All implementations must embed UnimplementedOpenFGAServiceServer
// for forward compatibility
type OpenFGAServiceServer interface {
	Read(context.Context, *ReadRequest) (*ReadResponse, error)
	Write(context.Context, *WriteRequest) (*WriteResponse, error)
	Check(context.Context, *CheckRequest) (*CheckResponse, error)
	Expand(context.Context, *ExpandRequest) (*ExpandResponse, error)
	ReadAuthorizationModels(context.Context, *ReadAuthorizationModelsRequest) (*ReadAuthorizationModelsResponse, error)
	ReadAuthorizationModel(context.Context, *ReadAuthorizationModelRequest) (*ReadAuthorizationModelResponse, error)
	WriteAuthorizationModel(context.Context, *WriteAuthorizationModelRequest) (*WriteAuthorizationModelResponse, error)
	WriteAssertions(context.Context, *WriteAssertionsRequest) (*WriteAssertionsResponse, error)
	ReadAssertions(context.Context, *ReadAssertionsRequest) (*ReadAssertionsResponse, error)
	ReadChanges(context.Context, *ReadChangesRequest) (*ReadChangesResponse, error)
	CreateStore(context.Context, *CreateStoreRequest) (*CreateStoreResponse, error)
	UpdateStore(context.Context, *UpdateStoreRequest) (*UpdateStoreResponse, error)
	DeleteStore(context.Context, *DeleteStoreRequest) (*DeleteStoreResponse, error)
	GetStore(context.Context, *GetStoreRequest) (*GetStoreResponse, error)
	ListStores(context.Context, *ListStoresRequest) (*ListStoresResponse, error)
	StreamedListObjects(*StreamedListObjectsRequest, OpenFGAService_StreamedListObjectsServer) error
	ListObjects(context.Context, *ListObjectsRequest) (*ListObjectsResponse, error)
	ListUsers(context.Context, *ListUsersRequest) (*ListUsersResponse, error)
	mustEmbedUnimplementedOpenFGAServiceServer()
}

// UnimplementedOpenFGAServiceServer must be embedded to have forward compatible implementations.
type UnimplementedOpenFGAServiceServer struct {
}

func (UnimplementedOpenFGAServiceServer) Read(context.Context, *ReadRequest) (*ReadResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Read not implemented")
}
func (UnimplementedOpenFGAServiceServer) Write(context.Context, *WriteRequest) (*WriteResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Write not implemented")
}
func (UnimplementedOpenFGAServiceServer) Check(context.Context, *CheckRequest) (*CheckResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Check not implemented")
}
func (UnimplementedOpenFGAServiceServer) Expand(context.Context, *ExpandRequest) (*ExpandResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Expand not implemented")
}
func (UnimplementedOpenFGAServiceServer) ReadAuthorizationModels(context.Context, *ReadAuthorizationModelsRequest) (*ReadAuthorizationModelsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ReadAuthorizationModels not implemented")
}
func (UnimplementedOpenFGAServiceServer) ReadAuthorizationModel(context.Context, *ReadAuthorizationModelRequest) (*ReadAuthorizationModelResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ReadAuthorizationModel not implemented")
}
func (UnimplementedOpenFGAServiceServer) WriteAuthorizationModel(context.Context, *WriteAuthorizationModelRequest) (*WriteAuthorizationModelResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method WriteAuthorizationModel not implemented")
}
func (UnimplementedOpenFGAServiceServer) WriteAssertions(context.Context, *WriteAssertionsRequest) (*WriteAssertionsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method WriteAssertions not implemented")
}
func (UnimplementedOpenFGAServiceServer) ReadAssertions(context.Context, *ReadAssertionsRequest) (*ReadAssertionsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ReadAssertions not implemented")
}
func (UnimplementedOpenFGAServiceServer) ReadChanges(context.Context, *ReadChangesRequest) (*ReadChangesResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ReadChanges not implemented")
}
func (UnimplementedOpenFGAServiceServer) CreateStore(context.Context, *CreateStoreRequest) (*CreateStoreResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method CreateStore not implemented")
}
func (UnimplementedOpenFGAServiceServer) UpdateStore(context.Context, *UpdateStoreRequest) (*UpdateStoreResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method UpdateStore not implemented")
}
func (UnimplementedOpenFGAServiceServer) DeleteStore(context.Context, *DeleteStoreRequest) (*DeleteStoreResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method DeleteStore not implemented")
}
func (UnimplementedOpenFGAServiceServer) GetStore(context.Context, *GetStoreRequest) (*GetStoreResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetStore not implemented")
}
func (UnimplementedOpenFGAServiceServer) ListStores(context.Context, *ListStoresRequest) (*ListStoresResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ListStores not implemented")
}
func (UnimplementedOpenFGAServiceServer) StreamedListObjects(*StreamedListObjectsRequest, OpenFGAService_StreamedListObjectsServer) error {
	return status.Errorf(codes.Unimplemented, "method StreamedListObjects not implemented")
}
func (UnimplementedOpenFGAServiceServer) ListObjects(context.Context, *ListObjectsRequest) (*ListObjectsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ListObjects not implemented")
}
func (UnimplementedOpenFGAServiceServer) ListUsers(context.Context, *ListUsersRequest) (*ListUsersResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ListUsers not implemented")
}
func (UnimplementedOpenFGAServiceServer) mustEmbedUnimplementedOpenFGAServiceServer() {}

// UnsafeOpenFGAServiceServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to OpenFGAServiceServer will
// result in compilation errors.
type UnsafeOpenFGAServiceServer interface {
	mustEmbedUnimplementedOpenFGAServiceServer()
}

func RegisterOpenFGAServiceServer(s grpc.ServiceRegistrar, srv OpenFGAServiceServer) {
	s.RegisterService(&OpenFGAService_ServiceDesc, srv)
}

func _OpenFGAService_Read_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ReadRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(OpenFGAServiceServer).Read(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: OpenFGAService_Read_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(OpenFGAServiceServer).Read(ctx, req.(*ReadRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _OpenFGAService_Write_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(WriteRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(OpenFGAServiceServer).Write(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: OpenFGAService_Write_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(OpenFGAServiceServer).Write(ctx, req.(*WriteRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _OpenFGAService_Check_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(CheckRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(OpenFGAServiceServer).Check(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: OpenFGAService_Check_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(OpenFGAServiceServer).Check(ctx, req.(*CheckRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _OpenFGAService_Expand_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ExpandRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(OpenFGAServiceServer).Expand(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: OpenFGAService_Expand_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(OpenFGAServiceServer).Expand(ctx, req.(*ExpandRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _OpenFGAService_ReadAuthorizationModels_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ReadAuthorizationModelsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(OpenFGAServiceServer).ReadAuthorizationModels(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: OpenFGAService_ReadAuthorizationModels_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(OpenFGAServiceServer).ReadAuthorizationModels(ctx, req.(*ReadAuthorizationModelsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _OpenFGAService_ReadAuthorizationModel_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ReadAuthorizationModelRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(OpenFGAServiceServer).ReadAuthorizationModel(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: OpenFGAService_ReadAuthorizationModel_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(OpenFGAServiceServer).ReadAuthorizationModel(ctx, req.(*ReadAuthorizationModelRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _OpenFGAService_WriteAuthorizationModel_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(WriteAuthorizationModelRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(OpenFGAServiceServer).WriteAuthorizationModel(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: OpenFGAService_WriteAuthorizationModel_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(OpenFGAServiceServer).WriteAuthorizationModel(ctx, req.(*WriteAuthorizationModelRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _OpenFGAService_WriteAssertions_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(WriteAssertionsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(OpenFGAServiceServer).WriteAssertions(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: OpenFGAService_WriteAssertions_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(OpenFGAServiceServer).WriteAssertions(ctx, req.(*WriteAssertionsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _OpenFGAService_ReadAssertions_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ReadAssertionsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(OpenFGAServiceServer).ReadAssertions(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: OpenFGAService_ReadAssertions_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(OpenFGAServiceServer).ReadAssertions(ctx, req.(*ReadAssertionsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _OpenFGAService_ReadChanges_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ReadChangesRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(OpenFGAServiceServer).ReadChanges(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: OpenFGAService_ReadChanges_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(OpenFGAServiceServer).ReadChanges(ctx, req.(*ReadChangesRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _OpenFGAService_CreateStore_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(CreateStoreRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(OpenFGAServiceServer).CreateStore(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: OpenFGAService_CreateStore_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(OpenFGAServiceServer).CreateStore(ctx, req.(*CreateStoreRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _OpenFGAService_UpdateStore_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(UpdateStoreRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(OpenFGAServiceServer).UpdateStore(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: OpenFGAService_UpdateStore_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(OpenFGAServiceServer).UpdateStore(ctx, req.(*UpdateStoreRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _OpenFGAService_DeleteStore_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(DeleteStoreRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(OpenFGAServiceServer).DeleteStore(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: OpenFGAService_DeleteStore_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(OpenFGAServiceServer).DeleteStore(ctx, req.(*DeleteStoreRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _OpenFGAService_GetStore_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(GetStoreRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(OpenFGAServiceServer).GetStore(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: OpenFGAService_GetStore_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(OpenFGAServiceServer).GetStore(ctx, req.(*GetStoreRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _OpenFGAService_ListStores_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ListStoresRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(OpenFGAServiceServer).ListStores(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: OpenFGAService_ListStores_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(OpenFGAServiceServer).ListStores(ctx, req.(*ListStoresRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _OpenFGAService_StreamedListObjects_Handler(srv interface{}, stream grpc.ServerStream) error {
	m := new(StreamedListObjectsRequest)
	if err := stream.RecvMsg(m); err != nil {
		return err
	}
	return srv.(OpenFGAServiceServer).StreamedListObjects(m, &openFGAServiceStreamedListObjectsServer{stream})
}

type OpenFGAService_StreamedListObjectsServer interface {
	Send(*StreamedListObjectsResponse) error
	grpc.ServerStream
}

type openFGAServiceStreamedListObjectsServer struct {
	grpc.ServerStream
}

func (x *openFGAServiceStreamedListObjectsServer) Send(m *StreamedListObjectsResponse) error {
	return x.ServerStream.SendMsg(m)
}

func _OpenFGAService_ListObjects_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ListObjectsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(OpenFGAServiceServer).ListObjects(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: OpenFGAService_ListObjects_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(OpenFGAServiceServer).ListObjects(ctx, req.(*ListObjectsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _OpenFGAService_ListUsers_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ListUsersRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(OpenFGAServiceServer).ListUsers(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: OpenFGAService_ListUsers_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(OpenFGAServiceServer).ListUsers(ctx, req.(*ListUsersRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// OpenFGAService_ServiceDesc is the grpc.ServiceDesc for OpenFGAService service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var OpenFGAService_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "openfga.v1.OpenFGAService",
	HandlerType: (*OpenFGAServiceServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Read",
			Handler:    _OpenFGAService_Read_Handler,
		},
		{
			MethodName: "Write",
			Handler:    _OpenFGAService_Write_Handler,
		},
		{
			MethodName: "Check",
			Handler:    _OpenFGAService_Check_Handler,
		},
		{
			MethodName: "Expand",
			Handler:    _OpenFGAService_Expand_Handler,
		},
		{
			MethodName: "ReadAuthorizationModels",
			Handler:    _OpenFGAService_ReadAuthorizationModels_Handler,
		},
		{
			MethodName: "ReadAuthorizationModel",
			Handler:    _OpenFGAService_ReadAuthorizationModel_Handler,
		},
		{
			MethodName: "WriteAuthorizationModel",
			Handler:    _OpenFGAService_WriteAuthorizationModel_Handler,
		},
		{
			MethodName: "WriteAssertions",
			Handler:    _OpenFGAService_WriteAssertions_Handler,
		},
		{
			MethodName: "ReadAssertions",
			Handler:    _OpenFGAService_ReadAssertions_Handler,
		},
		{
			MethodName: "ReadChanges",
			Handler:    _OpenFGAService_ReadChanges_Handler,
		},
		{
			MethodName: "CreateStore",
			Handler:    _OpenFGAService_CreateStore_Handler,
		},
		{
			MethodName: "UpdateStore",
			Handler:    _OpenFGAService_UpdateStore_Handler,
		},
		{
			MethodName: "DeleteStore",
			Handler:    _OpenFGAService_DeleteStore_Handler,
		},
		{
			MethodName: "GetStore",
			Handler:    _OpenFGAService_GetStore_Handler,
		},
		{
			MethodName: "ListStores",
			Handler:    _OpenFGAService_ListStores_Handler,
		},
		{
			MethodName: "ListObjects",
			Handler:    _OpenFGAService_ListObjects_Handler,
		},
		{
			MethodName: "ListUsers",
			Handler:    _OpenFGAService_ListUsers_Handler,
		},
	},
	Streams: []grpc.StreamDesc{
		{
			StreamName:    "StreamedListObjects",
			Handler:       _OpenFGAService_StreamedListObjects_Handler,
			ServerStreams: true,
		},
	},
	Metadata: "openfga/v1/openfga_service.proto",
}