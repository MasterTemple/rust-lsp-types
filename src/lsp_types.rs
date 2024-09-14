#![allow(
    dead_code,
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types
)]

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum IntegerOrString {
    String(String),
    Integer(Integer),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ArrayOrObject {
    Array(LSPArray),
    Object(LSPObject),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Value {
    Boolean(Boolean),
    Integer(Integer),
    String(String),
}

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub type Boolean = bool;

/**
 * Defines an Integer number in the range of -2^31 to 2^31 - 1.
 */
pub type Integer = i32;

/**
 * Defines an unsigned Integer number in the range of 0 to 2^31 - 1.
 */
pub type UInteger = u32;

/**
 * Defines a Decimal number. Since Decimal numbers are very
 * rare in the language server specification we denote the
 * exact range with every Decimal using the mathematics
 * interval notation (e.g. [0, 1] denotes all decimals d with
 * 0 <= d <= 1.
 */
pub type Decimal = f64;

/**
 * The LSP any type
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub enum LSPAny {
    LSPObject(LSPObject),
    LSPArray(LSPArray),
    String(String),
    Integer(Integer),
    UInteger(UInteger),
    Decimal(Decimal),
    Boolean(Boolean),
    // Null
}

/**
 * LSP object definition.
 *
 * @since 3.17.0
 */
pub type LSPObject = BTreeMap<String, LSPAny>;

/**
 * LSP arrays.
 *
 * @since 3.17.0
 */
pub type LSPArray = Vec<LSPAny>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub jsonrpc: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestMessage {
    /// extends Message
    pub jsonrpc: String,

    /**
     * The request id.
     */
    pub id: IntegerOrString,

    /**
     * The method to be invoked.
     */
    pub method: String,

    /**
     * The method's params.
     */
    // params: Option<array> | object,
    pub params: Option<ArrayOrObject>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMessage {
    /// extends Message
    pub jsonrpc: String,
    /**
     * The request id.
     */
    pub id: Option<IntegerOrString>,

    /**
     * The result of a request. This member is REQUIRED on success.
     * This member MUST NOT exist if there was an error invoking the method.
     */
    pub result: Option<LSPAny>,

    /**
     * The error object in case a request fails.
     */
    pub error: Option<ResponseError>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseError {
    /**
     * A number indicating the error type that occurred.
     */
    pub code: Integer,

    /**
     * A String providing a short description of the error.
     */
    pub message: String,

    /**
     * A primitive or structured value that contains additional
     * information about the error. Can be omitted.
     */
    pub data: Option<LSPAny>,
}

pub mod ErrorCodes {
    use super::*;
    // Defined by JSON-RPC
    pub const ParseError: Integer = -32700;
    pub const InvalidRequest: Integer = -32600;
    pub const MethodNotFound: Integer = -32601;
    pub const InvalidParams: Integer = -32602;
    pub const InternalError: Integer = -32603;

    /**
     * This is the start range of JSON-RPC reserved error codes.
     * It doesn't denote a real error code. No LSP error codes should
     * be defined between the start and end range. For backwards
     * compatibility the `ServerNotInitialized` and the `UnknownErrorCode`
     * are left in the range.
     *
     * @since 3.16.0
     */
    pub const jsonrpcReservedErrorRangeStart: Integer = -32099;
    /** @deprecated use jsonrpcReservedErrorRangeStart */
    pub const serverErrorStart: Integer = jsonrpcReservedErrorRangeStart;

    /**
     * Error code indicating that a server received a notification or
     * request before the server has received the `initialize` request.
     */
    pub const ServerNotInitialized: Integer = -32002;
    pub const UnknownErrorCode: Integer = -32001;
    /**
     * This is the end range of JSON-RPC reserved error codes.
     * It doesn't denote a real error code.
     *
     * @since 3.16.0
     */
    pub const jsonrpcReservedErrorRangeEnd: Integer = -32000;
    /** @deprecated use jsonrpcReservedErrorRangeEnd */
    pub const serverErrorEnd: Integer = jsonrpcReservedErrorRangeEnd;

    /**
     * This is the start range of LSP reserved error codes.
     * It doesn't denote a real error code.
     *
     * @since 3.16.0
     */
    pub const lspReservedErrorRangeStart: Integer = -32899;

    /**
     * A request failed but it was syntactically correct, e.g the
     * method name was known and the parameters were valid. The error
     * message should contain human readable information about why
     * the request failed.
     *
     * @since 3.17.0
     */
    pub const RequestFailed: Integer = -32803;

    /**
     * The server cancelled the request. This error code should
     * only be used for requests that explicitly support being
     * server cancellable.
     *
     * @since 3.17.0
     */
    pub const ServerCancelled: Integer = -32802;

    /**
     * The server detected that the content of a document got
     * modified outside normal conditions. A server should
     * NOT send this error code if it detects a content change
     * in it unprocessed messages. The result even computed
     * on an older state might still be useful for the client.
     *
     * If a client decides that a result is not of any use anymore
     * the client should cancel the request.
     */
    pub const ContentModified: Integer = -32801;

    /**
     * The client has canceled a request and a server has detected
     * the cancel.
     */
    pub const RequestCancelled: Integer = -32800;

    /**
     * This is the end range of LSP reserved error codes.
     * It doesn't denote a real error code.
     *
     * @since 3.16.0
     */
    pub const lspReservedErrorRangeEnd: Integer = -32800;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationMessage {
    /// extends Message
    pub jsonrpc: String,
    /**
     * The method to be invoked.
     */
    pub method: String,

    /**
     * The notification's params.
     */
    pub params: Option<ArrayOrObject>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CancelParams {
    /**
     * The request id to cancel.
     */
    pub id: IntegerOrString,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ProgressToken {
    Integer(Integer),
    String(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgressParams<T> {
    /**
     * The progress token provided by the client or server.
     */
    pub token: ProgressToken,

    /**
     * The progress data.
     */
    pub value: T,
}

/// extracted out for [HoverParams1::position]
#[derive(Serialize, Deserialize, Debug)]
pub struct HoverParamsPosition {
    pub line: UInteger,
    pub character: UInteger,
}

/// there are 2 HoverParams
#[derive(Serialize, Deserialize, Debug)]
pub struct HoverParams1 {
    /** The text document's URI in String form */
    pub textDocument: String,
    pub position: HoverParamsPosition,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HoverResult {
    pub value: String,
}

/// ```
///   foo://example.com:8042/over/there?name=ferret#nose
///   \_/   \______________/\_________/ \_________/ \__/
///    |           |            |            |        |
/// scheme     authority       path        query   fragment
///    |   _____________________|__
///   / \ /                        \
///   urn:example:animal:ferret:nose
/// ```
///
/// ```
/// file:///c:/project/readme.md
/// file:///C%3A/project/readme.md
/// ```
type DocumentUri = String;

type URI = String;

/**
 * Client capabilities specific to regular expressions.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct RegularExpressionsClientCapabilities {
    /**
     * The engine's name.
     */
    pub engine: String,

    /**
     * The engine's version.
     */
    pub version: Option<String>,
}

/// const EOL: String[] = ['\n', '\r\n', '\r'];
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum EOL {
    #[serde(rename = "\n")]
    /// "\n"
    LF,
    #[serde(rename = "\r\n")]
    /// "\r\n"
    CRLF,
    /// "\r"
    #[serde(rename = "\r")]
    CR,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    /**
     * Line position in a document (zero-based).
     */
    pub line: UInteger,

    /**
     * Character offset on a line in a document (zero-based). The meaning of this
     * offset is determined by the negotiated `PositionEncodingKind`.
     *
     * If the character value is greater than the line length it defaults back
     * to the line length.
     */
    pub character: UInteger,
}

/**
 * A type indicating how positions are encoded,
 * specifically what column offsets mean.
 *
 * @since 3.17.0
 */
/// pub type PositionEncodingKind = String;
/**
 * A set of predefined position encoding kinds.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub enum PositionEncodingKind {
    /**
     * Character offsets count UTF-8 code units (e.g bytes).
     */
    #[serde(rename = "utf-8")]
    UTF8,

    /**
     * Character offsets count UTF-16 code units.
     *
     * This is the default and must always be supported
     * by servers
     */
    #[serde(rename = "utf-16")]
    UTF16,

    /**
     * Character offsets count UTF-32 code units.
     *
     * Implementation note: these are the same as Unicode code points,
     * so this `PositionEncodingKind` may also be used for an
     * encoding-agnostic representation of character offsets.
     */
    #[serde(rename = "utf-32")]
    UTF32,
}

///  {
///      pub start: { line: 5, character: 23 },
///      end : { line: 6, character: 0 }
///  }
#[derive(Serialize, Deserialize, Debug)]
pub struct Range {
    /**
     * The range's start position.
     */
    pub start: Position,

    /**
     * The range's end position.
     */
    pub end: Position,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentItem {
    /**
     * The text document's URI.
     */
    pub uri: DocumentUri,

    /**
     * The text document's language identifier.
     */
    pub languageId: String,

    /**
     * The version number of this document (it will increase after each
     * change, including undo/redo).
     */
    pub version: Integer,

    /**
     * The content of the opened text document.
     */
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentIdentifier {
    /**
     * The text document's URI.
     */
    pub uri: DocumentUri,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionedTextDocumentIdentifier {
    /// extends TextDocumentIdentifier
    /**
     * The text document's URI.
     */
    pub uri: DocumentUri,
    /**
     * The version number of this document.
     *
     * The version number of a document will increase after each change,
     * including undo/redo. The number doesn't need to be consecutive.
     */
    pub version: Integer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OptionalVersionedTextDocumentIdentifier {
    /// extends TextDocumentIdentifier
    /**
     * The text document's URI.
     */
    pub uri: DocumentUri,
    /**
     * The version number of this document. If an optional versioned text document
     * identifier is sent from the server to the client and the file is not
     * open in the editor (the server has not received an open notification
     * before) the server can send `null` to indicate that the version is
     * known and the content on disk is the master (as specified with document
     * content ownership).
     *
     * The version number of a document will increase after each change,
     * including undo/redo. The number doesn't need to be consecutive.
     */
    pub version: Option<Integer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentPositionParams {
    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /**
     * The position inside the text document.
     */
    pub position: Position,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentFilter {
    /**
     * A language id, like `typescript`.
     */
    pub language: Option<String>,

    /**
     * A Uri scheme, like `file` or `untitled`.
     */
    pub scheme: Option<String>,

    /**
     * A glob pattern, like `*.{ts,js}`.
     *
     * Glob patterns can have the following syntax:
     * - `*` to match one or more characters in a path segment
     * - `?` to match on one character in a path segment
     * - `**` to match any number of path segments, including none
     * - `{}` to group sub patterns into an OR expression. (e.g. `**[FORWARD_SLASH]*.{ts,js}` matches all TypeScript and JavaScript files)
     * - `[]` to declare a range of characters to match in a path segment
     *   (e.g., `example.[0-9]` to match on `example.0`, `example.1`, …)
     * - `[!...]` to negate a range of characters to match in a path segment
     *   (e.g., `example.[!0-9]` to match on `example.a`, `example.b`, but
     *   not `example.0`)
     */
    pub pattern: Option<String>,
}

pub type DocumentSelector = Vec<DocumentFilter>;

#[derive(Serialize, Deserialize, Debug)]
pub struct TextEdit {
    /**
     * The range of the text document to be manipulated. To insert
     * text into a document create a range where start === end.
     */
    pub range: Range,

    /**
     * The String to be inserted. For delete operations use an
     * empty String.
     */
    pub newText: String,
}

/**
 * Additional information that describes document changes.
 *
 * @since 3.16.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeAnnotation {
    /**
     * A human-readable String describing the actual change. The String
     * is rendered prominent in the user interface.
     */
    pub label: String,

    /**
     * A flag which indicates that user confirmation is needed
     * before applying the change.
     */
    pub needsConfirmation: Option<Boolean>,

    /**
     * A human-readable String which is rendered less prominent in
     * the user interface.
     */
    pub description: Option<String>,
}

/**
 * An identifier referring to a change annotation managed by a workspace
 * edit.
 *
 * @since 3.16.0.
 */
pub type ChangeAnnotationIdentifier = String;

/**
 * A special text edit with an additional change annotation.
 *
 * @since 3.16.0.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct AnnotatedTextEdit {
    /// extends TextEdit
    /**
     * The range of the text document to be manipulated. To insert
     * text into a document create a range where start === end.
     */
    pub range: Range,

    /// extends TextEdit
    /**
     * The String to be inserted. For delete operations use an
     * empty String.
     */
    pub newText: String,

    /**
     * The actual annotation identifier.
     */
    pub annotationId: ChangeAnnotationIdentifier,
}

/// extracted out for [TextDocumentEdit::edits]
#[derive(Serialize, Deserialize, Debug)]
pub enum TextEditOrAnnotatedTextEdit {
    TextEdit(TextEdit),
    AnnotatedTextEdit(AnnotatedTextEdit),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentEdit {
    /**
     * The text document to change.
     */
    pub textDocument: OptionalVersionedTextDocumentIdentifier,

    /**
     * The edits to be applied.
     *
     * @since 3.16.0 - support for AnnotatedTextEdit. This is guarded by the
     * client capability `workspace.workspaceEdit.changeAnnotationSupport`
     */
    pub edits: Vec<TextEditOrAnnotatedTextEdit>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub uri: DocumentUri,
    pub range: Range,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationLink {
    /**
     * Span of the origin of this link.
     *
     * Used as the underlined span for mouse interaction. Defaults to the word
     * range at the mouse position.
     */
    pub originSelectionRange: Option<Range>,

    /**
     * The target resource identifier of this link.
     */
    pub targetUri: DocumentUri,

    /**
     * The full target range of this link. If the target for example is a symbol
     * then target range is the range enclosing this symbol not including
     * leading/trailing whitespace but everything else like comments. This
     * information is typically used to highlight the range in the editor.
     */
    pub targetRange: Range,

    /**
     * The range that should be selected and revealed when this link is being
     * followed, e.g the name of a function. Must be contained by the
     * `targetRange`. See also `DocumentSymbol#range`
     */
    pub targetSelectionRange: Range,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Diagnostic {
    /**
     * The range at which the message applies.
     */
    pub range: Range,

    /**
     * The diagnostic's severity. To avoid interpretation mismatches when a
     * server is used with different clients it is highly recommended that
     * servers always provide a severity value. If omitted, it’s recommended
     * for the client to interpret it as an Error severity.
     */
    pub severity: Option<DiagnosticSeverity>,

    /**
     * The diagnostic's code, which might appear in the user interface.
     */
    pub code: Option<IntegerOrString>,

    /**
     * An optional property to describe the error code.
     *
     * @since 3.16.0
     */
    pub codeDescription: Option<CodeDescription>,

    /**
     * A human-readable String describing the source of this
     * diagnostic, e.g. 'typescript' or 'super lint'.
     */
    pub source: Option<String>,

    /**
     * The diagnostic's message.
     */
    pub message: String,

    /**
     * Additional metadata about the diagnostic.
     *
     * @since 3.15.0
     */
    pub tags: Option<Vec<DiagnosticTag>>,

    /**
     * An array of related diagnostic information, e.g. when symbol-names within
     * a scope collide all definitions can be marked via this property.
     */
    pub relatedInformation: Option<Vec<DiagnosticRelatedInformation>>,

    /**
     * A data entry field that is preserved between a
     * `textDocument/publishDiagnostics` notification and
     * `textDocument/codeAction` request.
     *
     * @since 3.16.0
     */
    pub data: Option<LSPAny>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum DiagnosticSeverity {
    /**
     * Reports an error.
     */
    Error = 1,
    /**
     * Reports a warning.
     */
    Warning = 2,
    /**
     * Reports an information.
     */
    Information = 3,
    /**
     * Reports a hint.
     */
    Hint = 4,
}

/**
 * The diagnostic tags.
 *
 * @since 3.15.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub enum DiagnosticTag {
    /**
     * Unused or unnecessary code.
     *
     * Clients are allowed to render diagnostics with this tag faded out
     * instead of having an error squiggle.
     */
    Unnecessary = 1,
    /**
     * Deprecated or obsolete code.
     *
     * Clients are allowed to rendered diagnostics with this tag strike through.
     */
    Deprecated = 2,
}

/**
 * Represents a related message and source code location for a diagnostic.
 * This should be used to point to code locations that cause or are related to
 * a diagnostics, e.g when duplicating a symbol in a scope.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct DiagnosticRelatedInformation {
    /**
     * The location of this related diagnostic information.
     */
    pub location: Location,

    /**
     * The message of this related diagnostic information.
     */
    pub message: String,
}

/**
 * Structure to capture a description for an error code.
 *
 * @since 3.16.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct CodeDescription {
    /**
     * An URI to open with more information about the diagnostic error.
     */
    pub href: URI,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    /**
     * Title of the command, like `save`.
     */
    pub title: String,
    /**
     * The identifier of the actual command handler.
     */
    pub command: String,
    /**
     * Arguments that the command handler should be
     * invoked with.
     */
    pub arguments: Option<Vec<LSPAny>>,
}

/**
 * Describes the content type that a client supports in various
 * result literals like `Hover`, `ParameterInfo` or `CompletionItem`.
 *
 * Please note that `MarkupKinds` must not start with a `$`. This kinds
 * are reserved for internal usage.
 */
#[derive(Serialize, Deserialize, Debug)]
pub enum MarkupKind {
    /**
     * Plain text is supported as a content format
     */
    #[serde(rename = "plaintext")]
    PlainText,

    /**
     * Markdown is supported as a content format
     */
    #[serde(rename = "markdown")]
    Markdown,
}

/**
 * A `MarkupContent` literal represents a String value which content is
 * interpreted base on its kind flag. Currently the protocol supports
 * `plaintext` and `markdown` as markup kinds.
 *
 * If the kind is `markdown` then the value can contain fenced code blocks like
 * in GitHub issues.
 *
 * Here is an example how such a String can be constructed using
 * JavaScript / TypeScript:
 * typescript
 * let markdown: MarkdownContent = {
 *     pub kind: MarkupKind.Markdown,
 *     pub value: [
 *         '# Header',
 *         'Some text',
 *         'typescript',
 *         'someCode();',
 *         ''
 *     ].join('\n')
 * },
 *
 *
 * *Please Note* that clients might sanitize the return markdown. A client could
 * decide to remove HTML from the markdown to avoid script execution.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct MarkupContent {
    /**
     * The type of the Markup
     */
    pub kind: MarkupKind,

    /**
     * The content itself
     */
    pub value: String,
}

/**
 * Client capabilities specific to the used markdown parser.
 *
 * @since 3.16.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct MarkdownClientCapabilities {
    /**
     * The name of the parser.
     */
    pub parser: String,

    /**
     * The version of the parser.
     */
    pub version: Option<String>,

    /**
     * A list of HTML tags that the client allows / supports in
     * Markdown.
     *
     * @since 3.17.0
     */
    pub allowedTags: Option<Vec<String>>,
}

/**
 * Options to create a file.
 */
pub struct CreateFileOptions {
    /**
     * Overwrite existing file. Overwrite wins over `ignoreIfExists`
     */
    pub overwrite: Option<Boolean>,

    /**
     * Ignore if exists.
     */
    pub ignoreIfExists: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FileKind {}

/**
 * Create file operation
 */
pub struct CreateFile {
    /**
     * A create
     */
    /// kind: 'create',
    pub kind: ResourceOperationKind,

    /**
     * The resource to create.
     */
    pub uri: DocumentUri,

    /**
     * Additional options
     */
    pub options: Option<CreateFileOptions>,

    /**
     * An optional annotation identifier describing the operation.
     *
     * @since 3.16.0
     */
    pub annotationId: Option<ChangeAnnotationIdentifier>,
}

/**
 * Rename file options
 */
pub struct RenameFileOptions {
    /**
     * Overwrite target if existing. Overwrite wins over `ignoreIfExists`
     */
    pub overwrite: Option<Boolean>,

    /**
     * Ignores if target exists.
     */
    pub ignoreIfExists: Option<Boolean>,
}

/**
 * Rename file operation
 */
pub struct RenameFile {
    /**
     * A rename
     */
    /// kind: 'rename',
    pub kind: ResourceOperationKind,

    /**
     * The old (existing) location.
     */
    pub oldUri: DocumentUri,

    /**
     * The new location.
     */
    pub newUri: DocumentUri,

    /**
     * Rename options.
     */
    pub options: Option<RenameFileOptions>,

    /**
     * An optional annotation identifier describing the operation.
     *
     * @since 3.16.0
     */
    pub annotationId: Option<ChangeAnnotationIdentifier>,
}

/**
 * Delete file options
 */
pub struct DeleteFileOptions {
    /**
     * Delete the content recursively if a folder is denoted.
     */
    pub recursive: Option<Boolean>,

    /**
     * Ignore the operation if the file doesn't exist.
     */
    pub ignoreIfNotExists: Option<Boolean>,
}

/**
 * Delete file operation
 */
pub struct DeleteFile {
    /**
     * A delete
     */
    /// kind: 'delete',
    pub kind: ResourceOperationKind,

    /**
     * The file to delete.
     */
    pub uri: DocumentUri,

    /**
     * Delete options.
     */
    pub options: Option<DeleteFileOptions>,

    /**
     * An optional annotation identifier describing the operation.
     *
     * @since 3.16.0
     */
    pub annotationId: Option<ChangeAnnotationIdentifier>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum WorkspaceEditDocumentChanges {
    TextDocumentEdit(Vec<TextDocumentEdit>),
    // (TextDocumentEdit | CreateFile | RenameFile | DeleteFile)[]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceEdit {
    /**
     * Holds changes to existing resources.
     */
    // changes?: { [uri: DocumentUri]: TextEdit[]; },
    pub changes: Option<BTreeMap<DocumentUri, Vec<TextEdit>>>,

    /**
     * Depending on the client capability
     * `workspace.workspaceEdit.resourceOperations` document changes are either
     * an array of `TextDocumentEdit`s to express changes to n different text
     * documents where each text document edit addresses a specific version of
     * a text document. Or it can contain above `TextDocumentEdit`s mixed with
     * create, rename and delete file / folder operations.
     *
     * Whether a client supports versioned document edits is expressed via
     * `workspace.workspaceEdit.documentChanges` client capability.
     *
     * If a client neither supports `documentChanges` nor
     * `workspace.workspaceEdit.resourceOperations` then only plain `TextEdit`s
     * using the `changes` property are supported.
     */
    // documentChanges?: (
    //     TextDocumentEdit[] |
    //     (TextDocumentEdit | CreateFile | RenameFile | DeleteFile)[]
    // ),
    pub documentChanges: Option<WorkspaceEditDocumentChanges>,

    /**
     * A map of change annotations that can be referenced in
     * `AnnotatedTextEdit`s or create, rename and delete file / folder
     * operations.
     *
     * Whether clients honor this property depends on the client capability
     * `workspace.changeAnnotationSupport`.
     *
     * @since 3.16.0
     */
    // changeAnnotations?: {
    //     [id: String /* ChangeAnnotationIdentifier */]: ChangeAnnotation,
    // },
    pub changeAnnotations: Option<BTreeMap<ChangeAnnotationIdentifier, ChangeAnnotation>>,
}

/// extends from [WorkspaceEditClientCapabilities::changeAnnotationSupport]
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceEditClientCapabilitiesChangeAnnotationSupport {
    /**
     * Whether the client groups edits with equal labels into tree nodes,
     * for instance all edits labelled with "Changes in Strings" would
     * be a tree node.
     */
    pub groupsOnLabel: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceEditClientCapabilities {
    /**
     * The client supports versioned document changes in `WorkspaceEdit`s
     */
    pub documentChanges: Option<Boolean>,

    /**
     * The resource operations the client supports. Clients should at least
     * support 'create', 'rename' and 'delete' files and folders.
     *
     * @since 3.13.0
     */
    pub resourceOperations: Option<Vec<ResourceOperationKind>>,

    /**
     * The failure handling strategy of a client if applying the workspace edit
     * fails.
     *
     * @since 3.13.0
     */
    pub failureHandling: Option<FailureHandlingKind>,

    /**
     * Whether the client normalizes line endings to the client specific
     * setting.
     * If set to `true` the client will normalize line ending characters
     * in a workspace edit to the client specific new line character(s).
     *
     * @since 3.16.0
     */
    pub normalizesLineEndings: Option<Boolean>,

    /**
     * Whether the client in general supports change annotations on text edits,
     * create file, rename file and delete file changes.
     *
     * @since 3.16.0
     */
    pub changeAnnotationSupport: Option<WorkspaceEditClientCapabilitiesChangeAnnotationSupport>,
}

/**
 * The kind of resource operations supported by the client.
 */
#[derive(Serialize, Deserialize, Debug)]
pub enum ResourceOperationKind {
    /**
     * Supports creating new files and folders.
     */
    #[serde(rename = "create")]
    Create,
    /**
     * Supports renaming existing files and folders.
     */
    #[serde(rename = "rename")]
    Rename,
    /**
     * Supports deleting existing files and folders.
     */
    #[serde(rename = "delete")]
    Delete,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FailureHandlingKind {
    /**
     * Applying the workspace change is simply aborted if one of the changes
     * provided fails. All operations executed before the failing operation
     * stay executed.
     */
    #[serde(rename = "abort")]
    Abort,

    /**
     * All operations are executed transactional. That means they either all
     * succeed or no changes at all are applied to the workspace.
     */
    #[serde(rename = "transactional")]
    Transactional,
    /**
     * If the workspace edit contains only textual file changes they are
     * executed transactional. If resource changes (create, rename or delete
     * file) are part of the change the failure handling strategy is abort.
     */
    #[serde(rename = "textOnlyTransactional")]
    TextOnlyTransactional,

    /**
     * The client tries to undo the operations already executed. But there is no
     * guarantee that this is succeeding.
     */
    #[serde(rename = "undo")]
    Undo,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WorkDoneProgress {
    #[serde(rename = "begin")]
    Begin,
    #[serde(rename = "report")]
    Report,
    #[serde(rename = "end")]
    End,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkDoneProgressBegin {
    /// kind: 'begin',
    pub kind: WorkDoneProgress,

    /**
     * Mandatory title of the progress operation. Used to briefly inform about
     * the kind of operation being performed.
     *
     * Examples: "Indexing" or "Linking dependencies".
     */
    pub title: String,

    /**
     * Controls if a cancel button should show to allow the user to cancel the
     * long running operation. Clients that don't support cancellation are
     * allowed to ignore the setting.
     */
    pub cancellable: Option<Boolean>,

    /**
     * Optional, more detailed associated progress message. Contains
     * complementary information to the `title`.
     *
     * Examples: "3/25 files", "project/src/module2", "node_modules/some_dep".
     * If unset, the previous progress message (if any) is still valid.
     */
    pub message: Option<String>,

    /**
     * Optional progress percentage to display (value 100 is considered 100%).
     * If not provided infinite progress is assumed and clients are allowed
     * to ignore the `percentage` value in subsequent report notifications.
     *
     * The value should be steadily rising. Clients are free to ignore values
     * that are not following this rule. The value range is [0, 100].
     */
    pub percentage: Option<UInteger>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkDoneProgressReport {
    /// kind: 'report',
    pub kind: WorkDoneProgress,

    /**
     * Controls enablement state of a cancel button. This property is only valid
     * if a cancel button got requested in the `WorkDoneProgressBegin` payload.
     *
     * Clients that don't support cancellation or don't support control the
     * button's enablement state are allowed to ignore the setting.
     */
    pub cancellable: Option<Boolean>,

    /**
     * Optional, more detailed associated progress message. Contains
     * complementary information to the `title`.
     *
     * Examples: "3/25 files", "project/src/module2", "node_modules/some_dep".
     * If unset, the previous progress message (if any) is still valid.
     */
    pub message: Option<String>,

    /**
     * Optional progress percentage to display (value 100 is considered 100%).
     * If not provided infinite progress is assumed and clients are allowed
     * to ignore the `percentage` value in subsequent report notifications.
     *
     * The value should be steadily rising. Clients are free to ignore values
     * that are not following this rule. The value range is [0, 100].
     */
    pub percentage: Option<UInteger>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkDoneProgressEnd {
    /// kind: 'end',
    pub kind: WorkDoneProgress,

    /**
     * Optional, a final message indicating to for example indicate the outcome
     * of the operation.
     */
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkDoneProgressParams {
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkDoneProgressOptions {
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartialResultParams {
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TraceValue {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "messages")]
    Messages,
    #[serde(rename = "verbose")]
    Verbose,
}

/// extracts from [InitializeParams::clientInfo]
#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeParamsClientInfo {
    /**
     * The name of the client as defined by the client.
     */
    pub name: String,

    /**
     * The client's version as defined by the client.
     */
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /**
     * The process Id of the parent process that started the server. Is null if
     * the process has not been started by another process. If the parent
     * process is not alive then the server should exit (see exit notification)
     * its process.
     */
    pub processId: Option<Integer>,

    /**
     * Information about the client
     *
     * @since 3.15.0
     */
    pub clientInfo: Option<InitializeParamsClientInfo>,

    /**
     * The locale the client is currently showing the user interface
     * in. This must not necessarily be the locale of the operating
     * system.
     *
     * Uses IETF language tags as the value's syntax
     * (See https://en.wikipedia.org/wiki/IETF_language_tag)
     *
     * @since 3.16.0
     */
    pub locale: Option<String>,

    /**
     * The rootPath of the workspace. Is null
     * if no folder is open.
     *
     * @deprecated in favour of `rootUri`.
     */
    pub rootPath: Option<String>,

    /**
     * The rootUri of the workspace. Is null if no
     * folder is open. If both `rootPath` and `rootUri` are set
     * `rootUri` wins.
     *
     * @deprecated in favour of `workspaceFolders`
     */
    pub rootUri: Option<DocumentUri>,

    /**
     * User provided initialization options.
     */
    pub initializationOptions: Option<LSPAny>,

    /**
     * The capabilities provided by the client (editor or tool)
     */
    pub capabilities: ClientCapabilities,

    /**
     * The initial trace setting. If omitted trace is disabled ('off').
     */
    pub trace: Option<TraceValue>,

    /**
     * The workspace folders configured in the client when the server starts.
     * This property is only available if the client supports workspace folders.
     * It can be `null` if the client supports workspace folders but none are
     * configured.
     *
     * @since 3.6.0
     */
    pub workspaceFolders: Option<Vec<WorkspaceFolder>>,
}

/**
 * Text document specific client capabilities.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentClientCapabilities {
    pub synchronization: Option<TextDocumentSyncClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/completion` request.
     */
    pub completion: Option<CompletionClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/hover` request.
     */
    pub hover: Option<HoverClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/signatureHelp` request.
     */
    pub signatureHelp: Option<SignatureHelpClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/declaration` request.
     *
     * @since 3.14.0
     */
    pub declaration: Option<DeclarationClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/definition` request.
     */
    pub definition: Option<DefinitionClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/typeDefinition` request.
     *
     * @since 3.6.0
     */
    pub typeDefinition: Option<TypeDefinitionClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/implementation` request.
     *
     * @since 3.6.0
     */
    pub implementation: Option<ImplementationClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/references` request.
     */
    pub references: Option<ReferenceClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/documentHighlight` request.
     */
    pub documentHighlight: Option<DocumentHighlightClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/documentSymbol` request.
     */
    pub documentSymbol: Option<DocumentSymbolClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/codeAction` request.
     */
    pub codeAction: Option<CodeActionClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/codeLens` request.
     */
    pub codeLens: Option<CodeLensClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/documentLink` request.
     */
    pub documentLink: Option<DocumentLinkClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/documentColor` and the
     * `textDocument/colorPresentation` request.
     *
     * @since 3.6.0
     */
    pub colorProvider: Option<DocumentColorClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/formatting` request.
     */
    pub formatting: Option<DocumentFormattingClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/rangeFormatting` request.
     */
    pub rangeFormatting: Option<DocumentRangeFormattingClientCapabilities>,

    /** request.
     * Capabilities specific to the `textDocument/onTypeFormatting` request.
     */
    pub onTypeFormatting: Option<DocumentOnTypeFormattingClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/rename` request.
     */
    pub rename: Option<RenameClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/publishDiagnostics`
     * notification.
     */
    pub publishDiagnostics: Option<PublishDiagnosticsClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/foldingRange` request.
     *
     * @since 3.10.0
     */
    pub foldingRange: Option<FoldingRangeClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/selectionRange` request.
     *
     * @since 3.15.0
     */
    pub selectionRange: Option<SelectionRangeClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/linkedEditingRange` request.
     *
     * @since 3.16.0
     */
    pub linkedEditingRange: Option<LinkedEditingRangeClientCapabilities>,

    /**
     * Capabilities specific to the various call hierarchy requests.
     *
     * @since 3.16.0
     */
    pub callHierarchy: Option<CallHierarchyClientCapabilities>,

    /**
     * Capabilities specific to the various semantic token requests.
     *
     * @since 3.16.0
     */
    pub semanticTokens: Option<SemanticTokensClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/moniker` request.
     *
     * @since 3.16.0
     */
    pub moniker: Option<MonikerClientCapabilities>,

    /**
     * Capabilities specific to the various type hierarchy requests.
     *
     * @since 3.17.0
     */
    pub typeHierarchy: Option<TypeHierarchyClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/inlineValue` request.
     *
     * @since 3.17.0
     */
    pub inlineValue: Option<InlineValueClientCapabilities>,

    /**
     * Capabilities specific to the `textDocument/inlayHint` request.
     *
     * @since 3.17.0
     */
    pub inlayHint: Option<InlayHintClientCapabilities>,

    /**
     * Capabilities specific to the diagnostic pull model.
     *
     * @since 3.17.0
     */
    pub diagnostic: Option<DiagnosticClientCapabilities>,
}

/**
 * Capabilities specific to the notebook document support.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookDocumentClientCapabilities {
    /**
     * Capabilities specific to notebook document synchronization
     *
     * @since 3.17.0
     */
    pub synchronization: NotebookDocumentSyncClientCapabilities,
}

/// extracts from [ClientCapabilitiesWorkspace::fileOperations]
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientCapabilitiesWorkspaceFileOperations {
    /**
     * Whether the client supports dynamic registration for file
     * requests/notifications.
     */
    pub dynamicRegistration: Option<Boolean>,

    /**
     * The client has support for sending didCreateFiles notifications.
     */
    pub didCreate: Option<Boolean>,

    /**
     * The client has support for sending willCreateFiles requests.
     */
    pub willCreate: Option<Boolean>,

    /**
     * The client has support for sending didRenameFiles notifications.
     */
    pub didRename: Option<Boolean>,

    /**
     * The client has support for sending willRenameFiles requests.
     */
    pub willRename: Option<Boolean>,

    /**
     * The client has support for sending didDeleteFiles notifications.
     */
    pub didDelete: Option<Boolean>,

    /**
     * The client has support for sending willDeleteFiles requests.
     */
    pub willDelete: Option<Boolean>,
}

/// extracts from [ClientCapabilities::workspace]
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientCapabilitiesWorkspace {
    /**
     * The client supports applying batch edits
     * to the workspace by supporting the request
     * 'workspace/applyEdit'
     */
    pub applyEdit: Option<Boolean>,

    /**
     * Capabilities specific to `WorkspaceEdit`s
     */
    pub workspaceEdit: Option<WorkspaceEditClientCapabilities>,

    /**
     * Capabilities specific to the `workspace/didChangeConfiguration`
     * notification.
     */
    pub didChangeConfiguration: Option<DidChangeConfigurationClientCapabilities>,

    /**
     * Capabilities specific to the `workspace/didChangeWatchedFiles`
     * notification.
     */
    pub didChangeWatchedFiles: Option<DidChangeWatchedFilesClientCapabilities>,

    /**
     * Capabilities specific to the `workspace/symbol` request.
     */
    pub symbol: Option<WorkspaceSymbolClientCapabilities>,

    /**
     * Capabilities specific to the `workspace/executeCommand` request.
     */
    pub executeCommand: Option<ExecuteCommandClientCapabilities>,

    /**
     * The client has support for workspace folders.
     *
     * @since 3.6.0
     */
    pub workspaceFolders: Option<Boolean>,

    /**
     * The client supports `workspace/configuration` requests.
     *
     * @since 3.6.0
     */
    pub configuration: Option<Boolean>,

    /**
     * Capabilities specific to the semantic token requests scoped to the
     * workspace.
     *
     * @since 3.16.0
     */
    pub semanticTokens: Option<SemanticTokensWorkspaceClientCapabilities>,

    /**
     * Capabilities specific to the code lens requests scoped to the
     * workspace.
     *
     * @since 3.16.0
     */
    pub codeLens: Option<CodeLensWorkspaceClientCapabilities>,

    /**
     * The client has support for file requests/notifications.
     *
     * @since 3.16.0
     */
    pub fileOperations: Option<ClientCapabilitiesWorkspaceFileOperations>,

    /**
     * Client workspace capabilities specific to inline values.
     *
     * @since 3.17.0
     */
    pub inlineValue: Option<InlineValueWorkspaceClientCapabilities>,

    /**
     * Client workspace capabilities specific to inlay hints.
     *
     * @since 3.17.0
     */
    pub inlayHint: Option<InlayHintWorkspaceClientCapabilities>,

    /**
     * Client workspace capabilities specific to diagnostics.
     *
     * @since 3.17.0.
     */
    pub diagnostics: Option<DiagnosticWorkspaceClientCapabilities>,
}

/// extracted from [ClientCapabilities::window]
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientCapabilitiesWindow {
    /**
     * It indicates whether the client supports server initiated
     * progress using the `window/workDoneProgress/create` request.
     *
     * The capability also controls Whether client supports handling
     * of progress notifications. If set servers are allowed to report a
     * `workDoneProgress` property in the request specific server
     * capabilities.
     *
     * @since 3.15.0
     */
    pub workDoneProgress: Option<Boolean>,

    /**
     * Capabilities specific to the showMessage request
     *
     * @since 3.16.0
     */
    pub showMessage: Option<ShowMessageRequestClientCapabilities>,

    /**
     * Client capabilities for the show document request.
     *
     * @since 3.16.0
     */
    pub showDocument: Option<ShowDocumentClientCapabilities>,
}

/// extends from [ClientCapabilities::general]
#[derive(Serialize, Deserialize, Debug)]
pub struct StaleRequestSupport {
    /**
     * The client will actively cancel the request.
     */
    pub cancel: Boolean,

    /**
     * The list of requests for which the client
     * will retry the request if it receives a
     * response with error code `ContentModified``
     */
    pub retryOnContentModified: Vec<String>,
}

/// extends from [ClientCapabilities::general]
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientCapabilitiesGeneral {
    /**
     * Client capability that signals how the client
     * handles stale requests (e.g. a request
     * for which the client will not process the response
     * anymore since the information is outdated).
     *
     * @since 3.17.0
     */
    pub staleRequestSupport: Option<StaleRequestSupport>,

    /**
     * Client capabilities specific to regular expressions.
     *
     * @since 3.16.0
     */
    pub regularExpressions: Option<RegularExpressionsClientCapabilities>,

    /**
     * Client capabilities specific to the client's markdown parser.
     *
     * @since 3.16.0
     */
    pub markdown: Option<MarkdownClientCapabilities>,

    /**
     * The position encodings supported by the client. Client and server
     * have to agree on the same position encoding to ensure that offsets
     * (e.g. character position in a line) are interpreted the same on both
     * side.
     *
     * To keep the protocol backwards compatible the following applies: if
     * the value 'utf-16' is missing from the array of position encodings
     * servers can assume that the client supports UTF-16. UTF-16 is
     * therefore a mandatory encoding.
     *
     * If omitted it defaults to ['utf-16'].
     *
     * Implementation considerations: since the conversion from one encoding
     * into another requires the content of the file / line the conversion
     * is best done where the file is read which is usually on the server
     * side.
     *
     * @since 3.17.0
     */
    pub positionEncodings: Option<Vec<PositionEncodingKind>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientCapabilities {
    /**
     * Workspace specific client capabilities.
     */
    pub workspace: Option<ClientCapabilitiesWorkspace>,

    /**
     * Text document specific client capabilities.
     */
    pub textDocument: Option<TextDocumentClientCapabilities>,

    /**
     * Capabilities specific to the notebook document support.
     *
     * @since 3.17.0
     */
    pub notebookDocument: Option<NotebookDocumentClientCapabilities>,

    /**
     * Window specific client capabilities.
     */
    pub window: Option<ClientCapabilitiesWindow>,

    /**
     * General client capabilities.
     *
     * @since 3.16.0
     */
    pub general: Option<ClientCapabilitiesGeneral>,

    /**
     * Experimental client capabilities.
     */
    pub experimental: Option<LSPAny>,
}

/// extracted from [InitializeResult::ServerInfo]
#[derive(Serialize, Deserialize, Debug)]
pub struct ServerInfo {
    /**
     * The name of the server as defined by the server.
     */
    pub name: String,

    /**
     * The server's version as defined by the server.
     */
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeResult {
    /**
     * The capabilities the language server provides.
     */
    pub capabilities: ServerCapabilities,

    /**
     * Information about the server.
     *
     * @since 3.15.0
     */
    pub serverInfo: Option<ServerInfo>,
}

/**
 * Known error codes for an `InitializeErrorCodes`,
 */
#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum InitializeErrorCodes {
    /**
     * If the protocol version provided by the client can't be handled by
     * the server.
     *
     * @deprecated This initialize error got replaced by client capabilities.
     * There is no version handshake in version 3.0x
     */
    unknownProtocolVersion = 1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeError {
    /**
     * Indicates whether the client execute the following retry logic:
     * (1) show the message provided by the ResponseError to the user
     * (2) user selects retry or cancel
     * (3) if user selected retry the initialize method is sent again.
     */
    pub retry: Boolean,
}

pub mod ServerCapabilitiesProviders {
    use super::*;

    /// extracted from [ServerCapabilities::textDocumentSync]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum TextDocumentSync {
        TextDocumentSyncOptions(TextDocumentSyncOptions),
        TextDocumentSyncKind(TextDocumentSyncKind),
    }

    /// extracted from [ServerCapabilities::notebookDocumentSync]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum NotebookDocumentSync {
        NotebookDocumentSyncOptions(NotebookDocumentSyncOptions),
        NotebookDocumentSyncRegistrationOptions(NotebookDocumentSyncRegistrationOptions),
    }

    /// extracted from [ServerCapabilities::hoverProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum HoverProvider {
        Boolean(Boolean),
        HoverOptions(HoverOptions),
    }

    /// extracted from [ServerCapabilities::declarationProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum DeclarationProvider {
        Boolean(Boolean),
        DeclarationOptions(DeclarationOptions),
        DeclarationRegistrationOptions(DeclarationRegistrationOptions),
    }

    /// extracted from [ServerCapabilities::definitionProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum DefinitionProvider {
        Boolean(Boolean),
        DefinitionOptions(DefinitionOptions),
    }

    /// extracted from [ServerCapabilities::typeDefinitionProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum TypeDefinitionProvider {
        Boolean(Boolean),
        TypeDefinitionOptions(TypeDefinitionOptions),
        TypeDefinitionRegistrationOptions(TypeDefinitionRegistrationOptions),
    }

    /// extracted from [ServerCapabilities::implementationProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum ImplementationProvider {
        Boolean(Boolean),
        ImplementationOptions(ImplementationOptions),
        ImplementationRegistrationOptions(ImplementationRegistrationOptions),
    }

    /// extracted from [ServerCapabilities::referencesProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum ReferencesProvider {
        Boolean(Boolean),
        ReferenceOptions(ReferenceOptions),
    }

    /// extracted from [ServerCapabilities::documentHighlightProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum DocumentHighlightProvider {
        Boolean(Boolean),
        DocumentHighlightOptions(DocumentHighlightOptions),
    }

    /// extracted from [ServerCapabilities::documentSymbolProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum DocumentSymbolProvider {
        Boolean(Boolean),
        DocumentSymbolOptions(DocumentSymbolOptions),
    }

    /// extracted from [ServerCapabilities::codeActionProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum CodeActionProvider {
        Boolean(Boolean),
        CodeActionOptions(CodeActionOptions),
    }

    /// extracted from [ServerCapabilities::colorProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum ColorProvider {
        Boolean(Boolean),
        DocumentColorOptions(DocumentColorOptions),
        DocumentColorRegistrationOptions(DocumentColorRegistrationOptions),
    }

    /// extracted from [ServerCapabilities::documentFormattingProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum DocumentFormattingProvider {
        Boolean(Boolean),
        DocumentFormattingOptions(DocumentFormattingOptions),
    }

    /// extracted from [ServerCapabilities::renameProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum RenameProvider {
        Boolean(Boolean),
        RenameOptions(RenameOptions),
    }

    /// extracted from [ServerCapabilities::foldingRangeProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum FoldingRangeProvider {
        Boolean(Boolean),
        FoldingRangeOptions(FoldingRangeOptions),
        FoldingRangeRegistrationOptions(FoldingRangeRegistrationOptions),
    }

    /// extracted from [ServerCapabilities::selectionRangeProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum SelectionRangeProvider {
        Boolean(Boolean),
        SelectionRangeOptions(SelectionRangeOptions),
        SelectionRangeRegistrationOptions(SelectionRangeRegistrationOptions),
    }

    /// extracted from [ServerCapabilities::linkedEditingRangeProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum LinkedEditingRangeProvider {
        Boolean(Boolean),
        LinkedEditingRangeOptions(LinkedEditingRangeOptions),
        LinkedEditingRangeRegistrationOptions(LinkedEditingRangeRegistrationOptions),
    }

    /// extracted from [ServerCapabilities::callHierarchyProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum CallHierarchyProvider {
        Boolean(Boolean),
        CallHierarchyOptions(CallHierarchyOptions),
        CallHierarchyRegistrationOptions(CallHierarchyRegistrationOptions),
    }

    /// extracted from [ServerCapabilities::semanticTokensProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum SemanticTokensProvider {
        Boolean(Boolean),
        SemanticTokensOptions(SemanticTokensOptions),
        SemanticTokensRegistrationOptions(SemanticTokensRegistrationOptions),
    }

    /// extracted from [ServerCapabilities::monikerProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum MonikerProvider {
        Boolean(Boolean),
        MonikerOptions(MonikerOptions),
        MonikerRegistrationOptions(MonikerRegistrationOptions),
    }

    /// extracted from [ServerCapabilities::typeHierarchyProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum TypeHierarchyProvider {
        Boolean(Boolean),
        TypeHierarchyOptions(TypeHierarchyOptions),
        TypeHierarchyRegistrationOptions(TypeHierarchyRegistrationOptions),
    }

    /// extracted from [ServerCapabilities::inlineValueProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum InlineValueProvider {
        Boolean(Boolean),
        InlineValueOptions(InlineValueOptions),
        InlineValueRegistrationOptions(InlineValueRegistrationOptions),
    }

    /// extracted from [ServerCapabilities::inlayHintProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum InlayHintProvider {
        Boolean(Boolean),
        InlayHintOptions(InlayHintOptions),
        InlayHintRegistrationOptions(InlayHintRegistrationOptions),
    }

    /// extracted from [ServerCapabilities::diagnosticProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum DiagnosticProvider {
        DiagnosticOptions(DiagnosticOptions),
        DiagnosticRegistrationOptions(DiagnosticRegistrationOptions),
    }

    /// extracted from [ServerCapabilities::workspaceSymbolProvider]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum WorkspaceSymbolProvider {
        Boolean(Boolean),
        WorkspaceSymbolOptions(WorkspaceSymbolOptions),
    }
}

/// extracted from [ServerCapabilitiesWorkspace::fileOperations]
/// extracted from [ServerCapabilities::workspace]
#[derive(Serialize, Deserialize, Debug)]
pub struct ServerCapabilitiesWorkspaceFileOperations {
    /**
     * The server is interested in receiving didCreateFiles
     * notifications.
     */
    pub didCreate: Option<FileOperationRegistrationOptions>,

    /**
     * The server is interested in receiving willCreateFiles requests.
     */
    pub willCreate: Option<FileOperationRegistrationOptions>,

    /**
     * The server is interested in receiving didRenameFiles
     * notifications.
     */
    pub didRename: Option<FileOperationRegistrationOptions>,

    /**
     * The server is interested in receiving willRenameFiles requests.
     */
    pub willRename: Option<FileOperationRegistrationOptions>,

    /**
     * The server is interested in receiving didDeleteFiles file
     * notifications.
     */
    pub didDelete: Option<FileOperationRegistrationOptions>,

    /**
     * The server is interested in receiving willDeleteFiles file
     * requests.
     */
    pub willDelete: Option<FileOperationRegistrationOptions>,
}

/// extracted from [ServerCapabilities::workspace]
#[derive(Serialize, Deserialize, Debug)]
pub struct ServerCapabilitiesWorkspace {
    /**
     * The server supports workspace folder.
     *
     * @since 3.6.0
     */
    pub workspaceFolders: Option<WorkspaceFoldersServerCapabilities>,

    /**
     * The server is interested in file notifications/requests.
     *
     * @since 3.16.0
     */
    pub fileOperations: Option<ServerCapabilitiesWorkspaceFileOperations>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerCapabilities {
    /**
     * The position encoding the server picked from the encodings offered
     * by the client via the client capability `general.positionEncodings`.
     *
     * If the client didn't provide any position encodings the only valid
     * value that a server can return is 'utf-16'.
     *
     * If omitted it defaults to 'utf-16'.
     *
     * @since 3.17.0
     */
    pub positionEncoding: Option<PositionEncodingKind>,

    /**
     * Defines how text documents are synced. Is either a detailed structure
     * defining each notification or for backwards compatibility the
     * TextDocumentSyncKind number. If omitted it defaults to
     * `TextDocumentSyncKind.None`.
     */
    pub textDocumentSync: Option<ServerCapabilitiesProviders::TextDocumentSync>,

    /**
     * Defines how notebook documents are synced.
     *
     * @since 3.17.0
     */
    pub notebookDocumentSync: Option<ServerCapabilitiesProviders::NotebookDocumentSync>,

    /**
     * The server provides completion support.
     */
    pub completionProvider: Option<CompletionOptions>,

    /**
     * The server provides hover support.
     */
    pub hoverProvider: Option<ServerCapabilitiesProviders::HoverProvider>,

    /**
     * The server provides signature help support.
     */
    pub signatureHelpProvider: Option<SignatureHelpOptions>,

    /**
     * The server provides go to declaration support.
     *
     * @since 3.14.0
     */
    pub declarationProvider: Option<ServerCapabilitiesProviders::DeclarationProvider>,

    /**
     * The server provides goto definition support.
     */
    pub definitionProvider: Option<ServerCapabilitiesProviders::DefinitionProvider>,

    /**
     * The server provides goto type definition support.
     *
     * @since 3.6.0
     */
    pub typeDefinitionProvider: Option<ServerCapabilitiesProviders::TypeDefinitionProvider>,

    /**
     * The server provides goto implementation support.
     *
     * @since 3.6.0
     */
    pub implementationProvider: Option<ServerCapabilitiesProviders::ImplementationProvider>,

    /**
     * The server provides find references support.
     */
    pub referencesProvider: Option<ServerCapabilitiesProviders::ReferencesProvider>,

    /**
     * The server provides document highlight support.
     */
    pub documentHighlightProvider: Option<ServerCapabilitiesProviders::DocumentHighlightProvider>,

    /**
     * The server provides document symbol support.
     */
    pub documentSymbolProvider: Option<ServerCapabilitiesProviders::DocumentSymbolProvider>,

    /**
     * The server provides code actions. The `CodeActionOptions` return type is
     * only valid if the client signals code action literal support via the
     * property `textDocument.codeAction.codeActionLiteralSupport`.
     */
    pub codeActionProvider: Option<ServerCapabilitiesProviders::CodeActionProvider>,

    /**
     * The server provides code lens.
     */
    pub codeLensProvider: Option<CodeLensOptions>,

    /**
     * The server provides document link support.
     */
    pub documentLinkProvider: Option<DocumentLinkOptions>,

    /**
     * The server provides color provider support.
     *
     * @since 3.6.0
     */
    pub colorProvider: Option<ServerCapabilitiesProviders::ColorProvider>,

    /**
     * The server provides document formatting.
     */
    pub documentFormattingProvider: Option<ServerCapabilitiesProviders::DocumentFormattingProvider>,

    /**
     * The server provides document range formatting.
     */
    pub documentRangeFormattingProvider:
        Option<ServerCapabilitiesProviders::DocumentFormattingProvider>,

    /**
     * The server provides document formatting on typing.
     */
    pub documentOnTypeFormattingProvider: Option<DocumentOnTypeFormattingOptions>,

    /**
     * The server provides rename support. RenameOptions may only be
     * specified if the client states that it supports
     * `prepareSupport` in its initial `initialize` request.
     */
    pub renameProvider: Option<ServerCapabilitiesProviders::RenameProvider>,

    /**
     * The server provides folding provider support.
     *
     * @since 3.10.0
     */
    pub foldingRangeProvider: Option<ServerCapabilitiesProviders::FoldingRangeProvider>,

    /**
     * The server provides execute command support.
     */
    pub executeCommandProvider: Option<ExecuteCommandOptions>,

    /**
     * The server provides selection range support.
     *
     * @since 3.15.0
     */
    pub selectionRangeProvider: Option<ServerCapabilitiesProviders::SelectionRangeProvider>,

    /**
     * The server provides linked editing range support.
     *
     * @since 3.16.0
     */
    pub linkedEditingRangeProvider: Option<ServerCapabilitiesProviders::LinkedEditingRangeProvider>,

    /**
     * The server provides call hierarchy support.
     *
     * @since 3.16.0
     */
    pub callHierarchyProvider: Option<ServerCapabilitiesProviders::CallHierarchyProvider>,

    /**
     * The server provides semantic tokens support.
     *
     * @since 3.16.0
     */
    pub semanticTokensProvider: Option<ServerCapabilitiesProviders::SemanticTokensProvider>,

    /**
     * Whether server provides moniker support.
     *
     * @since 3.16.0
     */
    pub monikerProvider: Option<ServerCapabilitiesProviders::MonikerProvider>,

    /**
     * The server provides type hierarchy support.
     *
     * @since 3.17.0
     */
    pub typeHierarchyProvider: Option<ServerCapabilitiesProviders::TypeHierarchyProvider>,

    /**
     * The server provides inline values.
     *
     * @since 3.17.0
     */
    pub inlineValueProvider: Option<ServerCapabilitiesProviders::InlineValueProvider>,

    /**
     * The server provides inlay hints.
     *
     * @since 3.17.0
     */
    pub inlayHintProvider: Option<ServerCapabilitiesProviders::InlayHintProvider>,

    /**
     * The server has support for pull model diagnostics.
     *
     * @since 3.17.0
     */
    pub diagnosticProvider: Option<ServerCapabilitiesProviders::DiagnosticProvider>,

    /**
     * The server provides workspace symbol support.
     */
    pub workspaceSymbolProvider: Option<ServerCapabilitiesProviders::WorkspaceSymbolProvider>,

    /**
     * Workspace specific server capabilities
     */
    pub workspace: Option<ServerCapabilitiesWorkspace>,

    /**
     * Experimental server capabilities.
     */
    pub experimental: Option<LSPAny>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializedParams {}

/**
 * General parameters to register for a capability.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Registration {
    /**
     * The id used to register the request. The id can be used to deregister
     * the request again.
     */
    pub id: String,

    /**
     * The method / capability to register for.
     */
    pub method: String,

    /**
     * Options necessary for the registration.
     */
    pub registerOptions: Option<LSPAny>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegistrationParams {
    pub registrations: Vec<Registration>,
}

/**
 * Static registration options to be returned in the initialize request.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct StaticRegistrationOptions {
    /**
     * The id used to register the request. The id can be used to deregister
     * the request again. See also Registration#id.
     */
    pub id: Option<String>,
}

/**
 * General text document registration options.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentRegistrationOptions {
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,
}

/**
 * General parameters to unregister a capability.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Unregistration {
    /**
     * The id used to unregister the request or notification. Usually an id
     * provided during the register request.
     */
    pub id: String,

    /**
     * The method / capability to unregister for.
     */
    pub method: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnregistrationParams {
    /// This should correctly be named `unregistrations`. However changing this
    /// is a breaking change and needs to wait until we deliver a 4.x version
    /// of the specification.
    pub unregisterations: Vec<Unregistration>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetTraceParams {
    /**
     * The new value that should be assigned to the trace setting.
     */
    pub value: TraceValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogTraceParams {
    /**
     * The message to be logged.
     */
    pub message: String,
    /**
     * Additional information that can be computed if the `trace` configuration
     * is set to `'verbose'`
     */
    pub verbose: Option<String>,
}

/**
 * Defines how the host (editor) should sync document changes to the language
 * server.
 */
#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum TextDocumentSyncKind {
    /**
     * Documents should not be synced at all.
     */
    None = 0,

    /**
     * Documents are synced by always sending the full content
     * of the document.
     */
    Full = 1,

    /**
     * Documents are synced by sending the full content on open.
     * After that only incremental updates to the document are
     * sent.
     */
    Incremental = 2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DidOpenTextDocumentParams {
    /**
     * The document that was opened.
     */
    pub textDocument: TextDocumentItem,
}

/**
 * Describe options to be used when registering for text document change events.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentChangeRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,
    /**
     * How documents are synced to the server. See TextDocumentSyncKind.Full
     * and TextDocumentSyncKind.Incremental.
     */
    pub syncKind: TextDocumentSyncKind,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DidChangeTextDocumentParams {
    /**
     * The document that did change. The version number points
     * to the version after all provided content changes have
     * been applied.
     */
    pub textDocument: VersionedTextDocumentIdentifier,

    /**
     * The actual content changes. The content changes describe single state
     * changes to the document. So if there are two content changes c1 (at
     * array index 0) and c2 (at array index 1) for a document in state S then
     * c1 moves the document from S to S' and c2 from S' to S''. So c1 is
     * computed on the state S and c2 is computed on the state S'.
     *
     * To mirror the content of a document using change events use the following
     * approach:
     * - start with the same initial content
     * - apply the 'textDocument/didChange' notifications in the order you
     *   receive them.
     * - apply the `TextDocumentContentChangeEvent`s in a single notification
     *   in the order you receive them.
     */
    pub contentChanges: Vec<TextDocumentContentChangeEvent>,
}

/// extends from [TextDocumentContentChangeEvent]
#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentContentChangeEventWithRange {
    /**
     * The range of the document that changed.
     */
    pub range: Range,

    /**
     * The optional length of the range that got replaced.
     *
     * @deprecated use range instead.
     */
    pub rangeLength: Option<UInteger>,

    /**
     * The new text for the provided range.
     */
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentContentChangeEventWithoutRange {
    /**
     * The new text of the whole document.
     */
    pub text: String,
}

/**
 * An event describing a change to a text document. If only a text is provided
 * it is considered to be the full content of the document.
 */
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TextDocumentContentChangeEvent {
    TextDocumentContentChangeEventWithRange(TextDocumentContentChangeEventWithRange),
    TextDocumentContentChangeEventWithoutRange(TextDocumentContentChangeEventWithoutRange),
}

/**
 * The parameters send in a will save text document notification.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct WillSaveTextDocumentParams {
    /**
     * The document that will be saved.
     */
    pub textDocument: TextDocumentIdentifier,

    /**
     * The 'TextDocumentSaveReason'.
     */
    pub reason: TextDocumentSaveReason,
}

/**
 * Represents reasons why a text document is saved.
 */
#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum TextDocumentSaveReason {
    /**
     * Manually triggered, e.g. by the user pressing save, by starting
     * debugging, or by an API call.
     */
    Manual = 1,

    /**
     * Automatic after a delay.
     */
    AfterDelay = 2,

    /**
     * When the editor lost focus.
     */
    FocusOut = 3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveOptions {
    /**
     * The client is supposed to include the content on save.
     */
    pub includeText: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentSaveRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /**
     * The client is supposed to include the content on save.
     */
    pub includeText: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DidSaveTextDocumentParams {
    /**
     * The document that was saved.
     */
    pub textDocument: TextDocumentIdentifier,

    /**
     * Optional the content when saved. Depends on the includeText value
     * when the save notification was requested.
     */
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DidCloseTextDocumentParams {
    /**
     * The document that was closed.
     */
    pub textDocument: TextDocumentIdentifier,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentSyncClientCapabilities {
    /**
     * Whether text document synchronization supports dynamic registration.
     */
    pub dynamicRegistration: Option<Boolean>,

    /**
     * The client supports sending will save notifications.
     */
    pub willSave: Option<Boolean>,

    /**
     * The client supports sending a will save request and
     * waits for a response providing text edits which will
     * be applied to the document before it is saved.
     */
    pub willSaveWaitUntil: Option<Boolean>,

    /**
     * The client supports did save notifications.
     */
    pub didSave: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BooleanOrSaveOptions {
    Boolean(Boolean),
    SaveOptions(SaveOptions),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentSyncOptions {
    /**
     * Open and close notifications are sent to the server. If omitted open
     * close notification should not be sent.
     */
    pub openClose: Option<Boolean>,
    /**
     * Change notifications are sent to the server. See
     * TextDocumentSyncKind.None, TextDocumentSyncKind.Full and
     * TextDocumentSyncKind.Incremental. If omitted it defaults to
     * TextDocumentSyncKind.None.
     */
    pub change: Option<TextDocumentSyncKind>,
    /**
     * If present will save notifications are sent to the server. If omitted
     * the notification should not be sent.
     */
    pub willSave: Option<Boolean>,
    /**
     * If present will save wait until requests are sent to the server. If
     * omitted the request should not be sent.
     */
    pub willSaveWaitUntil: Option<Boolean>,
    /**
     * If present save notifications are sent to the server. If omitted the
     * notification should not be sent.
     */
    pub save: Option<BooleanOrSaveOptions>,
}

/**
 * A notebook document.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookDocument {
    /**
     * The notebook document's URI.
     */
    pub uri: URI,

    /**
     * The type of the notebook.
     */
    pub notebookType: String,

    /**
     * The version number of this document (it will increase after each
     * change, including undo/redo).
     */
    pub version: Integer,

    /**
     * Additional metadata stored with the notebook
     * document.
     */
    pub metadata: Option<LSPObject>,

    /**
     * The cells of a notebook.
     */
    pub cells: Vec<NotebookCell>,
}

/**
 * A notebook cell.
 *
 * A cell's document URI must be unique across ALL notebook
 * cells and can therefore be used to uniquely identify a
 * notebook cell or the cell's text document.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookCell {
    /**
     * The cell's kind
     */
    pub kind: NotebookCellKind,

    /**
     * The URI of the cell's text document
     * content.
     */
    pub document: DocumentUri,

    /**
     * Additional metadata stored with the cell.
     */
    pub metadata: Option<LSPObject>,

    /**
     * Additional execution summary information
     * if supported by the client.
     */
    pub executionSummary: Option<ExecutionSummary>,
}

/**
 * A notebook cell kind.
 *
 * @since 3.17.0
 */
#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum NotebookCellKind {
    /**
     * A markup-cell is formatted source that is used for display.
     */
    Markup = 1,

    /**
     * A code-cell is source code.
     */
    Code = 2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecutionSummary {
    /**
     * A strict monotonically increasing value
     * indicating the execution order of a cell
     * inside a notebook.
     */
    pub executionOrder: UInteger,

    /**
     * Whether the execution was successful or
     * not if known by the client.
     */
    pub success: Option<Boolean>,
}

/// String | NotebookDocumentFilter
#[derive(Serialize, Deserialize, Debug)]
pub enum StringOrNotebookDocumentFilter {
    String(String),
    NotebookDocumentFilter(NotebookDocumentFilter),
}

/**
 * A notebook cell text document filter denotes a cell text
 * document by different properties.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookCellTextDocumentFilter {
    /**
     * A filter that matches against the notebook
     * containing the notebook cell. If a String
     * value is provided it matches against the
     * notebook type. '*' matches every notebook.
     */
    pub notebook: StringOrNotebookDocumentFilter,

    /**
     * A language id like `python`.
     *
     * Will be matched against the language id of the
     * notebook cell document. '*' matches every language.
     */
    pub language: Option<String>,
}

/**
 * A notebook document filter denotes a notebook document by
 * different properties.
 *
 * @since 3.17.0
 */
/// the TypeScript signatures indicate that at least 1 will be a string, the rest can undefined
#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookDocumentFilter {
    /** The type of the enclosing notebook. */
    pub notebookType: Option<String>,

    /** A Uri scheme, like `file` or `untitled`. */
    pub scheme: Option<String>,

    /** A glob pattern. */
    pub pattern: Option<String>,
}

/**
 * Notebook specific client capabilities.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookDocumentSyncClientCapabilities {
    /**
     * Whether implementation supports dynamic registration. If this is
     * set to `true` the client supports the new
     * `(NotebookDocumentSyncRegistrationOptions & NotebookDocumentSyncOptions)`
     * return value for the corresponding server capability as well.
     */
    pub dynamicRegistration: Option<Boolean>,

    /**
     * The client supports sending execution summary data per cell.
     */
    pub executionSummarySupport: Option<Boolean>,
}

/// extracted from [NotebookDocumentSyncOptions::notebookSelector]
#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookDocumentSyncOptionsNotebookSelectorNotebookCell {
    pub language: String,
}

/// extracted from [NotebookDocumentSyncOptions::notebookSelector]
#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookDocumentSyncOptionsNotebookSelectorNotebook {
    /**
     * The notebook to be synced. If a String
     * value is provided it matches against the
     * notebook type. '*' matches every notebook.
     */
    pub notebook: StringOrNotebookDocumentFilter,

    /**
     * The cells of the matching notebook to be synced.
     */
    pub cells: Option<Vec<NotebookDocumentSyncOptionsNotebookSelectorNotebookCell>>,
}

/// extracted from [NotebookDocumentSyncOptions::notebookSelector]
#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookDocumentSyncOptionsNotebookSelectorCells {
    /**
     * The notebook to be synced. If a String
     * value is provided it matches against the
     * notebook type. '*' matches every notebook.
     */
    pub notebook: Option<StringOrNotebookDocumentFilter>,

    /**
     * The cells of the matching notebook to be synced.
     */
    pub cells: Vec<NotebookDocumentSyncOptionsNotebookSelectorNotebookCell>,
}

/// extracted from [NotebookDocumentSyncOptions::notebookSelector]
#[derive(Serialize, Deserialize, Debug)]
pub enum NotebookDocumentSyncOptionsNotebookSelector {
    NotebookDocumentSyncOptionsNotebookSelectorNotebook(
        NotebookDocumentSyncOptionsNotebookSelectorNotebook,
    ),
    NotebookDocumentSyncOptionsNotebookSelectorCells(
        NotebookDocumentSyncOptionsNotebookSelectorCells,
    ),
}

/**
 * Options specific to a notebook plus its cells
 * to be synced to the server.
 *
 * If a selector provides a notebook document
 * filter but no cell selector all cells of a
 * matching notebook document will be synced.
 *
 * If a selector provides no notebook document
 * filter but only a cell selector all notebook
 * documents that contain at least one matching
 * cell will be synced.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookDocumentSyncOptions {
    /**
     * The notebooks to be synced
     */
    pub notebookSelector: Vec<NotebookDocumentSyncOptionsNotebookSelector>,

    /**
     * Whether save notification should be forwarded to
     * the server. Will only be honored if mode === `notebook`.
     */
    pub save: Option<Boolean>,
}

/**
 * Registration options specific to a notebook.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookDocumentSyncRegistrationOptions {
    /// extends NotebookDocumentSyncOptions
    /**
     * The notebooks to be synced
     */
    pub notebookSelector: Vec<NotebookDocumentSyncOptionsNotebookSelector>,

    /// extends NotebookDocumentSyncOptions
    /**
     * Whether save notification should be forwarded to
     * the server. Will only be honored if mode === `notebook`.
     */
    pub save: Option<Boolean>,

    /// extends StaticRegistrationOptions
    /**
     * The id used to register the request. The id can be used to deregister
     * the request again. See also Registration#id.
     */
    pub id: Option<String>,
}

/**
 * The params sent in an open notebook document notification.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct DidOpenNotebookDocumentParams {
    /**
     * The notebook document that got opened.
     */
    pub notebookDocument: NotebookDocument,

    /**
     * The text documents that represent the content
     * of a notebook cell.
     */
    pub cellTextDocuments: Vec<TextDocumentItem>,
}

/**
 * The params sent in a change notebook document notification.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct DidChangeNotebookDocumentParams {
    /**
     * The notebook document that did change. The version number points
     * to the version after all provided changes have been applied.
     */
    pub notebookDocument: VersionedNotebookDocumentIdentifier,

    /**
     * The actual changes to the notebook document.
     *
     * The change describes single state change to the notebook document.
     * So it moves a notebook document, its cells and its cell text document
     * contents from state S to S'.
     *
     * To mirror the content of a notebook using change events use the
     * following approach:
     * - start with the same initial content
     * - apply the 'notebookDocument/didChange' notifications in the order
     *   you receive them.
     */
    pub change: NotebookDocumentChangeEvent,
}

/**
 * A versioned notebook document identifier.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct VersionedNotebookDocumentIdentifier {
    /**
     * The version number of this notebook document.
     */
    pub version: Integer,

    /**
     * The notebook document's URI.
     */
    pub uri: URI,
}

/// extracted from [NotebookDocumentChangeEventCells::structure]
#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookDocumentChangeEventCellsStructure {
    /**
     * The change to the cell array.
     */
    pub array: NotebookCellArrayChange,

    /**
     * Additional opened cell text documents.
     */
    pub didOpen: Option<Vec<TextDocumentItem>>,

    /**
     * Additional closed cell text documents.
     */
    pub didClose: Option<Vec<TextDocumentIdentifier>>,
}

/// extracted from [NotebookDocumentChangeEventCells::textContent]
#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookDocumentChangeEventCellsTextContent {
    pub document: VersionedTextDocumentIdentifier,
    pub changes: Vec<TextDocumentContentChangeEvent>,
}

/// extracted from [NotebookDocumentChangeEvent::cells]
#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookDocumentChangeEventCells {
    /**
     * Changes to the cell structure to add or
     * remove cells.
     */
    pub structure: Option<NotebookDocumentChangeEventCellsStructure>,

    /**
     * Changes to notebook cells properties like its
     * kind, execution summary or metadata.
     */
    pub data: Option<Vec<NotebookCell>>,

    /**
     * Changes to the text content of notebook cells.
     */
    pub textContent: Option<Vec<NotebookDocumentChangeEventCellsTextContent>>,
}

/**
 * A change event for a notebook document.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookDocumentChangeEvent {
    /**
     * The changed meta data if any.
     */
    pub metadata: Option<LSPObject>,

    /**
     * Changes to cells
     */
    pub cells: Option<NotebookDocumentChangeEventCells>,
}

/**
 * A change describing how to move a `NotebookCell`
 * array from state S to S'.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookCellArrayChange {
    /**
     * The start offset of the cell that changed.
     */
    pub start: UInteger,

    /**
     * The deleted cells
     */
    pub deleteCount: UInteger,

    /**
     * The new cells, if any
     */
    pub cells: Option<Vec<NotebookCell>>,
}

/**
 * The params sent in a save notebook document notification.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct DidSaveNotebookDocumentParams {
    /**
     * The notebook document that got saved.
     */
    pub notebookDocument: NotebookDocumentIdentifier,
}

/**
 * The params sent in a close notebook document notification.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct DidCloseNotebookDocumentParams {
    /**
     * The notebook document that got closed.
     */
    pub notebookDocument: NotebookDocumentIdentifier,

    /**
     * The text documents that represent the content
     * of a notebook cell that got closed.
     */
    pub cellTextDocuments: Vec<TextDocumentIdentifier>,
}

/**
 * A literal to identify a notebook document in the client.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookDocumentIdentifier {
    /**
     * The notebook document's URI.
     */
    pub uri: URI,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationClientCapabilities {
    /**
     * Whether declaration supports dynamic registration. If this is set to
     * `true` the client supports the new `DeclarationRegistrationOptions`
     * return value for the corresponding server capability as well.
     */
    pub dynamicRegistration: Option<Boolean>,

    /**
     * The client supports additional metadata in the form of declaration links.
     */
    pub linkSupport: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationRegistrationOptions {
    /// extends DeclarationOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends StaticRegistrationOptions
    /**
     * The id used to register the request. The id can be used to deregister
     * the request again. See also Registration#id.
     */
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeclarationParams {
    /// extends TextDocumentPositionParams
    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /// extends TextDocumentPositionParams
    /**
     * The position inside the text document.
     */
    pub position: Position,

    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefinitionClientCapabilities {
    /**
     * Whether definition supports dynamic registration.
     */
    pub dynamicRegistration: Option<Boolean>,

    /**
     * The client supports additional metadata in the form of definition links.
     *
     * @since 3.14.0
     */
    pub linkSupport: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefinitionOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefinitionRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends DefinitionOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefinitionParams {
    /// extends TextDocumentPositionParams
    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /// extends TextDocumentPositionParams
    /**
     * The position inside the text document.
     */
    pub position: Position,

    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypeDefinitionClientCapabilities {
    /**
     * Whether implementation supports dynamic registration. If this is set to
     * `true` the client supports the new `TypeDefinitionRegistrationOptions`
     * return value for the corresponding server capability as well.
     */
    pub dynamicRegistration: Option<Boolean>,

    /**
     * The client supports additional metadata in the form of definition links.
     *
     * @since 3.14.0
     */
    pub linkSupport: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypeDefinitionOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypeDefinitionRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends TypeDefinitionOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /// extends StaticRegistrationOptions
    /**
     * The id used to register the request. The id can be used to deregister
     * the request again. See also Registration#id.
     */
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypeDefinitionParams {
    /// extends TextDocumentPositionParams
    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /// extends TextDocumentPositionParams
    /**
     * The position inside the text document.
     */
    pub position: Position,

    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImplementationClientCapabilities {
    /**
     * Whether implementation supports dynamic registration. If this is set to
     * `true` the client supports the new `ImplementationRegistrationOptions`
     * return value for the corresponding server capability as well.
     */
    pub dynamicRegistration: Option<Boolean>,

    /**
     * The client supports additional metadata in the form of definition links.
     *
     * @since 3.14.0
     */
    pub linkSupport: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImplementationOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImplementationRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends ImplementationOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /// extends StaticRegistrationOptions
    /**
     * The id used to register the request. The id can be used to deregister
     * the request again. See also Registration#id.
     */
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImplementationParams {
    /// extends TextDocumentPositionParams
    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /// extends TextDocumentPositionParams
    /**
     * The position inside the text document.
     */
    pub position: Position,

    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReferenceClientCapabilities {
    /**
     * Whether references supports dynamic registration.
     */
    pub dynamicRegistration: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReferenceOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReferenceRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends ReferenceOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReferenceParams {
    /// extends TextDocumentPositionParams
    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /// extends TextDocumentPositionParams
    /**
     * The position inside the text document.
     */
    pub position: Position,

    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,

    pub context: ReferenceContext,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReferenceContext {
    /**
     * Include the declaration of the current symbol.
     */
    pub includeDeclaration: Boolean,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallHierarchyClientCapabilities {
    /**
     * Whether implementation supports dynamic registration. If this is set to
     * `true` the client supports the new `(TextDocumentRegistrationOptions &
     * StaticRegistrationOptions)` return value for the corresponding server
     * capability as well.
     */
    pub dynamicRegistration: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallHierarchyOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallHierarchyRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends CallHierarchyOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /// extends StaticRegistrationOptions
    /**
     * The id used to register the request. The id can be used to deregister
     * the request again. See also Registration#id.
     */
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallHierarchyPrepareParams {
    /// extends TextDocumentPositionParams
    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /// extends TextDocumentPositionParams
    /**
     * The position inside the text document.
     */
    pub position: Position,
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallHierarchyItem {
    /**
     * The name of this item.
     */
    pub name: String,

    /**
     * The kind of this item.
     */
    pub kind: SymbolKind,

    /**
     * Tags for this item.
     */
    pub tags: Option<Vec<SymbolTag>>,

    /**
     * More detail for this item, e.g. the signature of a function.
     */
    pub detail: Option<String>,

    /**
     * The resource identifier of this item.
     */
    pub uri: DocumentUri,

    /**
     * The range enclosing this symbol not including leading/trailing whitespace
     * but everything else, e.g. comments and code.
     */
    pub range: Range,

    /**
     * The range that should be selected and revealed when this symbol is being
     * picked, e.g. the name of a function. Must be contained by the
     * [`range`](#CallHierarchyItem.range).
     */
    pub selectionRange: Range,

    /**
     * A data entry field that is preserved between a call hierarchy prepare and
     * incoming calls or outgoing calls requests.
     */
    pub data: Option<LSPAny>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallHierarchyIncomingCallsParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,

    pub item: CallHierarchyItem,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallHierarchyIncomingCall {
    /**
     * The item that makes the call.
     */
    pub from: CallHierarchyItem,

    /**
     * The ranges at which the calls appear. This is relative to the caller
     * denoted by [`this.from`](#CallHierarchyIncomingCall.from).
     */
    pub fromRanges: Vec<Range>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallHierarchyOutgoingCallsParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,

    pub item: CallHierarchyItem,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallHierarchyOutgoingCall {
    /**
     * The item that is called.
     */
    pub to: CallHierarchyItem,

    /**
     * The range at which this item is called. This is the range relative to
     * the caller, e.g the item passed to `callHierarchy/outgoingCalls` request.
     */
    pub fromRanges: Vec<Range>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypeHierarchyClientCapabilities {
    /**
     * Whether implementation supports dynamic registration. If this is set to
     * `true` the client supports the new `(TextDocumentRegistrationOptions &
     * StaticRegistrationOptions)` return value for the corresponding server
     * capability as well.
     */
    pub dynamicRegistration: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypeHierarchyOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypeHierarchyRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends TypeHierarchyOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /// extends StaticRegistrationOptions
    /**
     * The id used to register the request. The id can be used to deregister
     * the request again. See also Registration#id.
     */
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypeHierarchyPrepareParams {
    /// extends TextDocumentPositionParams
    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /// extends TextDocumentPositionParams
    /**
     * The position inside the text document.
     */
    pub position: Position,

    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypeHierarchyItem {
    /**
     * The name of this item.
     */
    pub name: String,

    /**
     * The kind of this item.
     */
    pub kind: SymbolKind,

    /**
     * Tags for this item.
     */
    pub tags: Option<Vec<SymbolTag>>,

    /**
     * More detail for this item, e.g. the signature of a function.
     */
    pub detail: Option<String>,

    /**
     * The resource identifier of this item.
     */
    pub uri: DocumentUri,

    /**
     * The range enclosing this symbol not including leading/trailing whitespace
     * but everything else, e.g. comments and code.
     */
    pub range: Range,

    /**
     * The range that should be selected and revealed when this symbol is being
     * picked, e.g. the name of a function. Must be contained by the
     * [`range`](#TypeHierarchyItem.range).
     */
    pub selectionRange: Range,

    /**
     * A data entry field that is preserved between a type hierarchy prepare and
     * supertypes or subtypes requests. It could also be used to identify the
     * type hierarchy in the server, helping improve the performance on
     * resolving supertypes and subtypes.
     */
    pub data: Option<LSPAny>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypeHierarchySupertypesParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,

    pub item: TypeHierarchyItem,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypeHierarchySubtypesParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,

    pub item: TypeHierarchyItem,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentHighlightClientCapabilities {
    /**
     * Whether document highlight supports dynamic registration.
     */
    pub dynamicRegistration: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentHighlightOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentHighlightRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends DocumentHighlightOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentHighlightParams {
    /// extends TextDocumentPositionParams
    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /// extends TextDocumentPositionParams
    /**
     * The position inside the text document.
     */
    pub position: Position,

    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,
}

/**
 * A document highlight is a range inside a text document which deserves
 * special attention. Usually a document highlight is visualized by changing
 * the background color of its range.
 *
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentHighlight {
    /**
     * The range this highlight applies to.
     */
    pub range: Range,

    /**
     * The highlight kind, default is DocumentHighlightKind.Text.
     */
    pub kind: Option<DocumentHighlightKind>,
}

/**
 * A document highlight kind.
 */
#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum DocumentHighlightKind {
    /**
     * A textual occurrence.
     */
    Text = 1,

    /**
     * Read-access of a symbol, like reading a variable.
     */
    Read = 2,

    /**
     * Write-access of a symbol, like writing to a variable.
     */
    Write = 3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentLinkClientCapabilities {
    /**
     * Whether document link supports dynamic registration.
     */
    pub dynamicRegistration: Option<Boolean>,

    /**
     * Whether the client supports the `tooltip` property on `DocumentLink`.
     *
     * @since 3.15.0
     */
    pub tooltipSupport: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentLinkOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /**
     * Document links have a resolve provider as well.
     */
    pub resolveProvider: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentLinkRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends DocumentLinkOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /**
     * Document links have a resolve provider as well.
     */
    pub resolveProvider: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentLinkParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,

    /**
     * The document to provide document links for.
     */
    pub textDocument: TextDocumentIdentifier,
}

/**
 * A document link is a range in a text document that links to an internal or
 * external resource, like another text document or a web site.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentLink {
    /**
     * The range this link applies to.
     */
    pub range: Range,

    /**
     * The uri this link points to. If missing a resolve request is sent later.
     */
    pub target: Option<URI>,

    /**
     * The tooltip text when you hover over this link.
     *
     * If a tooltip is provided, is will be displayed in a String that includes
     * instructions on how to trigger the link, such as `{0} (ctrl + click)`.
     * The specific instructions vary depending on OS, user settings, and
     * localization.
     *
     * @since 3.15.0
     */
    pub tooltip: Option<String>,

    /**
     * A data entry field that is preserved on a document link between a
     * DocumentLinkRequest and a DocumentLinkResolveRequest.
     */
    pub data: Option<LSPAny>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HoverClientCapabilities {
    /**
     * Whether hover supports dynamic registration.
     */
    pub dynamicRegistration: Option<Boolean>,

    /**
     * Client supports the follow content formats if the content
     * property refers to a `literal of type MarkupContent`.
     * The order describes the preferred format of the client.
     */
    pub contentFormat: Option<Vec<MarkupKind>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HoverOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HoverRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends HoverOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

/// there are 2 HoverParams
#[derive(Serialize, Deserialize, Debug)]
pub struct HoverParams2 {
    /// extends TextDocumentPositionParams
    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /// extends TextDocumentPositionParams
    /**
     * The position inside the text document.
     */
    pub position: Position,

    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,
}

/// extracted from [Hover::contents]
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum HoverContents {
    MarkedString(MarkedString),
    MarkedStringArray(Vec<MarkedString>),
    MarkupContent(MarkupContent),
}
/**
 * The result of a hover request.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Hover {
    /**
     * The hover's content
     */
    pub contents: HoverContents,

    /**
     * An optional range is a range inside a text document
     * that is used to visualize a hover, e.g. by changing the background color.
     */
    pub range: Option<Range>,
}

/**
 * MarkedString can be used to render human readable text. It is either a
 * markdown String or a code-block that provides a language and a code snippet.
 * The language identifier is semantically equal to the optional language
 * identifier in fenced code blocks in GitHub issues.
 *
 * The pair of a language and a value is an equivalent to markdown:
 * ${language}
 * ${value}
 *
 *
 * Note that markdown strings will be sanitized - that means html will be
 * escaped.
 *
 * @deprecated use MarkupContent instead.
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MarkedString {
    String(String),
    LanguageString { language: String, value: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeLensClientCapabilities {
    /**
     * Whether code lens supports dynamic registration.
     */
    pub dynamicRegistration: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeLensOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /**
     * Code lens has a resolve provider as well.
     */
    pub resolveProvider: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeLensRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends CodeLensOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /// extends CodeLensOptions
    /**
     * Code lens has a resolve provider as well.
     */
    pub resolveProvider: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeLensParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,

    /**
     * The document to request code lens for.
     */
    pub textDocument: TextDocumentIdentifier,
}

/**
 * A code lens represents a command that should be shown along with
 * source text, like the number of references, a way to run tests, etc.
 *
 * A code lens is _unresolved_ when no command is associated to it. For
 * performance reasons the creation of a code lens and resolving should be done
 * in two stages.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct CodeLens {
    /**
     * The range in which this code lens is valid. Should only span a single
     * line.
     */
    pub range: Range,

    /**
     * The command this code lens represents.
     */
    pub command: Option<Command>,

    /**
     * A data entry field that is preserved on a code lens item between
     * a code lens and a code lens resolve request.
     */
    pub data: Option<LSPAny>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeLensWorkspaceClientCapabilities {
    /**
     * Whether the client implementation supports a refresh request sent from the
     * server to the client.
     *
     * Note that this event is global and will force the client to refresh all
     * code lenses currently shown. It should be used with absolute care and is
     * useful for situation where a server for example detect a project wide
     * change that requires such a calculation.
     */
    pub refreshSupport: Option<Boolean>,
}

/// extracted from [FoldingRangeClientCapabilities::foldingRangeKing]
#[derive(Serialize, Deserialize, Debug)]
pub struct FoldingRangeKindStruct {
    /**
     * The folding range kind values the client supports. When this
     * property exists the client also guarantees that it will
     * handle values outside its set gracefully and falls back
     * to a default value when unknown.
     */
    pub valueSet: Option<Vec<FoldingRangeKind>>,
}

/// extracted from [FoldingRangeClientCapabilities::foldingRange]
#[derive(Serialize, Deserialize, Debug)]
pub struct FoldingRangeStruct {
    /**
     * If set, the client signals that it supports setting collapsedText on
     * folding ranges to display custom labels instead of the default text.
     *
     * @since 3.17.0
     */
    pub collapsedText: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FoldingRangeClientCapabilities {
    /**
     * Whether implementation supports dynamic registration for folding range
     * providers. If this is set to `true` the client supports the new
     * `FoldingRangeRegistrationOptions` return value for the corresponding
     * server capability as well.
     */
    pub dynamicRegistration: Option<Boolean>,

    /**
     * The maximum number of folding ranges that the client prefers to receive
     * per document. The value serves as a hint, servers are free to follow the
     * limit.
     */
    pub rangeLimit: Option<UInteger>,

    /**
     * If set, the client signals that it only supports folding complete lines.
     * If set, client will ignore specified `startCharacter` and `endCharacter`
     * properties in a FoldingRange.
     */
    pub lineFoldingOnly: Option<Boolean>,

    /**
     * Specific options for the folding range kind.
     *
     * @since 3.17.0
     */
    pub foldingRangeKind: Option<FoldingRangeKindStruct>,

    /**
     * Specific options for the folding range.
     * @since 3.17.0
     */
    pub foldingRange: Option<FoldingRangeStruct>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FoldingRangeOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FoldingRangeRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends FoldingRangeOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /// extends StaticRegistrationOptions
    /**
     * The id used to register the request. The id can be used to deregister
     * the request again. See also Registration#id.
     */
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FoldingRangeParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,

    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,
}

/**
 * A set of predefined range kinds.
 */
/**
 * The type is a String since the value set is extensible
 */
#[derive(Serialize, Deserialize, Debug)]
pub enum FoldingRangeKind {
    /**
     * Folding range for a comment
     */
    #[serde(rename = "comment")]
    Comment,

    /**
     * Folding range for imports or includes
     */
    #[serde(rename = "imports")]
    Imports,

    /**
     * Folding range for a region (e.g. `#region`)
     */
    #[serde(rename = "region")]
    Region,
}

/**
 * Represents a folding range. To be valid, start and end line must be bigger
 * than zero and smaller than the number of lines in the document. Clients
 * are free to ignore invalid ranges.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct FoldingRange {
    /**
     * The zero-based start line of the range to fold. The folded area starts
     * after the line's last character. To be valid, the end must be zero or
     * larger and smaller than the number of lines in the document.
     */
    pub startLine: UInteger,

    /**
     * The zero-based character offset from where the folded range starts. If
     * not defined, defaults to the length of the start line.
     */
    pub startCharacter: Option<UInteger>,

    /**
     * The zero-based end line of the range to fold. The folded area ends with
     * the line's last character. To be valid, the end must be zero or larger
     * and smaller than the number of lines in the document.
     */
    pub endLine: UInteger,

    /**
     * The zero-based character offset before the folded range ends. If not
     * defined, defaults to the length of the end line.
     */
    pub endCharacter: Option<UInteger>,

    /**
     * Describes the kind of the folding range such as `comment` or `region`.
     * The kind is used to categorize folding ranges and used by commands like
     * 'Fold all comments'. See [FoldingRangeKind](#FoldingRangeKind) for an
     * enumeration of standardized kinds.
     */
    pub kind: Option<FoldingRangeKind>,

    /**
     * The text that the client should show when the specified range is
     * collapsed. If not defined or not supported by the client, a default
     * will be chosen by the client.
     *
     * @since 3.17.0 - proposed
     */
    pub collapsedText: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectionRangeClientCapabilities {
    /**
     * Whether implementation supports dynamic registration for selection range
     * providers. If this is set to `true` the client supports the new
     * `SelectionRangeRegistrationOptions` return value for the corresponding
     * server capability as well.
     */
    pub dynamicRegistration: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectionRangeOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectionRangeRegistrationOptions {
    /// extends SelectionRangeOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends StaticRegistrationOptions
    /**
     * The id used to register the request. The id can be used to deregister
     * the request again. See also Registration#id.
     */
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectionRangeParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,

    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /**
     * The positions inside the text document.
     */
    pub positions: Vec<Position>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectionRange {
    /**
     * The [range](#Range) of this selection range.
     */
    pub range: Range,
    /**
     * The parent selection range containing this range. Therefore
     * `parent.range` must contain `this.range`.
     */
    // parent: Option<SelectionRange>,
    pub parent: Option<Box<SelectionRange>>,
}

/// extracted from [DocumentSymbolClientCapabilities::symbolKind]
#[derive(Serialize, Deserialize, Debug)]
pub struct SymbolKindStruct {
    /**
     * The symbol kind values the client supports. When this
     * property exists the client also guarantees that it will
     * handle values outside its set gracefully and falls back
     * to a default value when unknown.
     *
     * If this property is not present the client only supports
     * the symbol kinds from `File` to `Array` as defined in
     * the initial version of the protocol.
     */
    pub valueSet: Option<Vec<SymbolKind>>,
}

/// extracted from [DocumentSymbolClientCapabilities::tagSupport]
#[derive(Serialize, Deserialize, Debug)]
pub struct TagSupportStruct {
    /**
     * The tags supported by the client.
     */
    pub valueSet: Vec<SymbolTag>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentSymbolClientCapabilities {
    /**
     * Whether document symbol supports dynamic registration.
     */
    pub dynamicRegistration: Option<Boolean>,

    /**
     * Specific capabilities for the `SymbolKind` in the
     * `textDocument/documentSymbol` request.
     */
    pub symbolKind: Option<SymbolKindStruct>,

    /**
     * The client supports hierarchical document symbols.
     */
    pub hierarchicalDocumentSymbolSupport: Option<Boolean>,

    /**
     * The client supports tags on `SymbolInformation`. Tags are supported on
     * `DocumentSymbol` if `hierarchicalDocumentSymbolSupport` is set to true.
     * Clients supporting tags have to handle unknown tags gracefully.
     *
     * @since 3.16.0
     */
    pub tagSupport: Option<TagSupportStruct>,

    /**
     * The client supports an additional label presented in the UI when
     * registering a document symbol provider.
     *
     * @since 3.16.0
     */
    pub labelSupport: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentSymbolOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /**
     * A human-readable String that is shown when multiple outlines trees
     * are shown for the same document.
     *
     * @since 3.16.0
     */
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentSymbolRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends DocumentSymbolOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /// extends DocumentSymbolOptions
    /**
     * A human-readable String that is shown when multiple outlines trees
     * are shown for the same document.
     *
     * @since 3.16.0
     */
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentSymbolParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,

    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,
}

/**
 * A symbol kind.
 */
#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum SymbolKind {
    File = 1,
    Module = 2,
    Namespace = 3,
    Package = 4,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Enum = 10,
    Interface = 11,
    Function = 12,
    Variable = 13,
    Constant = 14,
    String = 15,
    Number = 16,
    Boolean = 17,
    Array = 18,
    Object = 19,
    Key = 20,
    Null = 21,
    EnumMember = 22,
    Struct = 23,
    Event = 24,
    Operator = 25,
    TypeParameter = 26,
}

/**
 * Symbol tags are extra annotations that tweak the rendering of a symbol.
 *
 * @since 3.16
 */
#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum SymbolTag {
    /**
     * Render a symbol as obsolete, usually using a strike-out.
     */
    Deprecated = 1,
}

/**
 * Represents programming constructs like variables, classes, interfaces etc.
 * that appear in a document. Document symbols can be hierarchical and they
 * have two ranges: one that encloses its definition and one that points to its
 * most interesting range, e.g. the range of an identifier.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentSymbol {
    /**
     * The name of this symbol. Will be displayed in the user struct and
     * therefore must not be an empty String or a String only consisting of
     * white spaces.
     */
    pub name: String,

    /**
     * More detail for this symbol, e.g the signature of a function.
     */
    pub detail: Option<String>,

    /**
     * The kind of this symbol.
     */
    pub kind: SymbolKind,

    /**
     * Tags for this document symbol.
     *
     * @since 3.16.0
     */
    pub tags: Option<Vec<SymbolTag>>,

    /**
     * Indicates if this symbol is deprecated.
     *
     * @deprecated Use tags instead
     */
    pub deprecated: Option<Boolean>,

    /**
     * The range enclosing this symbol not including leading/trailing whitespace
     * but everything else like comments. This information is typically used to
     * determine if the clients cursor is inside the symbol to reveal in the
     * symbol in the UI.
     */
    pub range: Range,

    /**
     * The range that should be selected and revealed when this symbol is being
     * picked, e.g. the name of a function. Must be contained by the `range`.
     */
    pub selectionRange: Range,

    /**
     * Children of this symbol, e.g. properties of a class.
     */
    pub children: Option<Vec<DocumentSymbol>>,
}

/**
 * Represents information about programming constructs like variables, classes;
 * interfaces etc.
 *
 * @deprecated use DocumentSymbol or WorkspaceSymbol instead.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct SymbolInformation {
    /**
     * The name of this symbol.
     */
    pub name: String,

    /**
     * The kind of this symbol.
     */
    pub kind: SymbolKind,

    /**
     * Tags for this symbol.
     *
     * @since 3.16.0
     */
    pub tags: Option<Vec<SymbolTag>>,

    /**
     * Indicates if this symbol is deprecated.
     *
     * @deprecated Use tags instead
     */
    pub deprecated: Option<Boolean>,

    /**
     * The location of this symbol. The location's range is used by a tool
     * to reveal the location in the editor. If the symbol is selected in the
     * tool the range's start information is used to position the cursor. So
     * the range usually spans more then the actual symbol's name and does
     * normally include things like visibility modifiers.
     *
     * The range doesn't have to denote a node range in the sense of an abstract
     * syntax tree. It can therefore not be used to re-construct a hierarchy of
     * the symbols.
     */
    pub location: Location,

    /**
     * The name of the symbol containing this symbol. This information is for
     * user struct purposes (e.g. to render a qualifier in the user interface
     * if necessary). It can't be used to re-infer a hierarchy for the document
     * symbols.
     */
    pub containerName: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SemanticTokenTypes {
    #[serde(rename = "namespace")]
    Namespace,
    /**
     * Represents a generic type. Acts as a fallback for types which
     * can't be mapped to a specific type like class or enum.
     */
    #[serde(rename = "type")]
    Type,
    #[serde(rename = "class")]
    Class,
    #[serde(rename = "enum")]
    Enum,
    #[serde(rename = "interface")]
    Interface,
    #[serde(rename = "struct")]
    Struct,
    #[serde(rename = "typeParameter")]
    TypeParameter,
    #[serde(rename = "parameter")]
    Parameter,
    #[serde(rename = "variable")]
    Variable,
    #[serde(rename = "property")]
    Property,
    #[serde(rename = "enumMember")]
    EnumMember,
    #[serde(rename = "event")]
    Event,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "method")]
    Method,
    #[serde(rename = "macro")]
    Macro,
    #[serde(rename = "keyword")]
    Keyword,
    #[serde(rename = "modifier")]
    Modifier,
    #[serde(rename = "comment")]
    Comment,
    #[serde(rename = "String")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "regexp")]
    Regexp,
    #[serde(rename = "operator")]
    Operator,
    /**
     * @since 3.17.0
     */
    #[serde(rename = "decorator")]
    Decorator,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SemanticTokenModifiers {
    #[serde(rename = "declaration")]
    Declaration,
    #[serde(rename = "definition")]
    Definition,
    #[serde(rename = "readonly")]
    Readonly,
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "deprecated")]
    Deprecated,
    #[serde(rename = "abstract")]
    Abstract,
    #[serde(rename = "async")]
    Async,
    #[serde(rename = "modification")]
    Modification,
    #[serde(rename = "documentation")]
    Documentation,
    #[serde(rename = "defaultLibrary")]
    DefaultLibrary,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TokenFormat {
    #[serde(rename = "relative")]
    Relative,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokensLegend {
    /**
     * The token types a server uses.
     */
    pub tokenTypes: Vec<String>,

    /**
     * The token modifiers a server uses.
     */
    pub tokenModifiers: Vec<String>,
}

/// extracted from [SemanticTokensClientCapabilitiesRequests::full]
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SemanticTokensClientCapabilitiesRequestsFull {
    OptionBoolean(Option<Boolean>),
    OptionDelta {
        /**
         * The client will send the `textDocument/semanticTokens/full/delta`
         * request if the server provides a corresponding handler.
         */
        pub delta: Option<Boolean>,
    },
}

/// extracted from [SemanticTokensClientCapabilities::requests]
#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokensClientCapabilitiesRequests {
    /**
     * The client will send the `textDocument/semanticTokens/range` request
     * if the server provides a corresponding handler.
     */
    pub range: Option<Boolean>,

    /**
     * The client will send the `textDocument/semanticTokens/full` request
     * if the server provides a corresponding handler.
     */
    pub full: SemanticTokensClientCapabilitiesRequestsFull,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokensClientCapabilities {
    /**
     * Whether implementation supports dynamic registration. If this is set to
     * `true` the client supports the new `(TextDocumentRegistrationOptions &
     * StaticRegistrationOptions)` return value for the corresponding server
     * capability as well.
     */
    pub dynamicRegistration: Option<Boolean>,

    /**
     * Which requests the client supports and might send to the server
     * depending on the server's capability. Please note that clients might not
     * show semantic tokens or degrade some of the user experience if a range
     * or full request is advertised by the client but not provided by the
     * server. If for example the client capability `requests.full` and
     * `request.range` are both set to true but the server only provides a
     * range provider the client might not render a minimap correctly or might
     * even decide to not show any semantic tokens at all.
     */
    pub requests: SemanticTokensClientCapabilitiesRequests,

    /**
     * The token types that the client supports.
     */
    pub tokenTypes: Vec<String>,

    /**
     * The token modifiers that the client supports.
     */
    pub tokenModifiers: Vec<String>,

    /**
     * The formats the clients supports.
     */
    pub formats: Vec<TokenFormat>,

    /**
     * Whether the client supports tokens that can overlap each other.
     */
    pub overlappingTokenSupport: Option<Boolean>,

    /**
     * Whether the client supports tokens that can span multiple lines.
     */
    pub multilineTokenSupport: Option<Boolean>,

    /**
     * Whether the client allows the server to actively cancel a
     * semantic token request, e.g. supports returning
     * ErrorCodes.ServerCancelled. If a server does the client
     * needs to retrigger the request.
     *
     * @since 3.17.0
     */
    pub serverCancelSupport: Option<Boolean>,

    /**
     * Whether the client uses semantic tokens to augment existing
     * syntax tokens. If set to `true` client side created syntax
     * tokens and semantic tokens are both used for colorization. If
     * set to `false` the client only uses the returned semantic tokens
     * for colorization.
     *
     * If the value is `undefined` then the client behavior is not
     * specified.
     *
     * @since 3.17.0
     */
    pub augmentsSyntaxTokens: Option<Boolean>,
}

/// extended from [SemanticTokensOptions::full]
#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokensOptionsFullDelta {
    pub delta: Option<Boolean>,
}

/// extended from [SemanticTokensOptions::full]
#[derive(Serialize, Deserialize, Debug)]
pub enum SemanticTokensOptionsFull {
    Boolean(Boolean),
    SemanticTokensOptionsFullDelta(SemanticTokensOptionsFullDelta),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokensOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /**
     * The legend used by the server
     */
    pub legend: SemanticTokensLegend,

    /**
     * Server supports providing semantic tokens for a specific range
     * of a document.
     */
    /// idk why the docs say `range?: boolean | { };`
    pub range: Option<Boolean>,

    /**
     * Server supports providing semantic tokens for a full document.
     */
    pub full: Option<SemanticTokensOptionsFull>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokensRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends SemanticTokensOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /// extends SemanticTokensOptions
    /**
     * The legend used by the server
     */
    pub legend: SemanticTokensLegend,
    /// extends SemanticTokensOptions

    /// extends SemanticTokensOptions
    /**
     * Server supports providing semantic tokens for a specific range
     * of a document.
     */
    /// idk why the docs say `range?: boolean | { };`
    pub range: Option<Boolean>,

    /// extends SemanticTokensOptions
    /**
     * Server supports providing semantic tokens for a full document.
     */
    pub full: Option<SemanticTokensOptionsFull>,

    /// extends StaticRegistrationOptions
    /**
     * The id used to register the request. The id can be used to deregister
     * the request again. See also Registration#id.
     */
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokensParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,

    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokens {
    /**
     * An optional result id. If provided and clients support delta updating
     * the client will include the result id in the next semantic token request.
     * A server can then instead of computing all semantic tokens again simply
     * send a delta.
     */
    pub resultId: Option<String>,

    /**
     * The actual tokens.
     */
    pub data: Vec<UInteger>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokensPartialResult {
    pub data: Vec<UInteger>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokensDeltaParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,

    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /**
     * The result id of a previous response. The result Id can either point to
     * a full response or a delta response depending on what was received last.
     */
    pub previousResultId: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokensDelta {
    /// readonly
    pub resultId: Option<String>,
    /**
     * The semantic token edits to transform a previous result into a new
     * result.
     */
    pub edits: Vec<SemanticTokensEdit>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokensEdit {
    /**
     * The start offset of the edit.
     */
    pub start: UInteger,

    /**
     * The count of elements to remove.
     */
    pub deleteCount: UInteger,

    /**
     * The elements to insert.
     */
    pub data: Option<Vec<UInteger>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokensDeltaPartialResult {
    pub edits: Vec<SemanticTokensEdit>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokensRangeParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,

    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /**
     * The range the semantic tokens are requested for.
     */
    pub range: Range,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokensWorkspaceClientCapabilities {
    /**
     * Whether the client implementation supports a refresh request sent from
     * the server to the client.
     *
     * Note that this event is global and will force the client to refresh all
     * semantic tokens currently shown. It should be used with absolute care
     * and is useful for situation where a server for example detect a project
     * wide change that requires such a calculation.
     */
    pub refreshSupport: Option<Boolean>,
}

/// extracted from [InlayHintClientCapabilities::resolveSupport]
#[derive(Serialize, Deserialize, Debug)]
pub struct InlayHintClientCapabilitiesResolveSupport {
    /**
     * The properties that a client can resolve lazily.
     */
    pub properties: Vec<String>,
}

/**
 * Inlay hint client capabilities.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct InlayHintClientCapabilities {
    /**
     * Whether inlay hints support dynamic registration.
     */
    pub dynamicRegistration: Option<Boolean>,

    /**
     * Indicates which properties a client can resolve lazily on an inlay
     * hint.
     */
    pub resolveSupport: Option<InlayHintClientCapabilitiesResolveSupport>,
}

/**
 * Inlay hint options used during static registration.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct InlayHintOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /**
     * The server provides support to resolve additional
     * information for an inlay hint item.
     */
    pub resolveProvider: Option<Boolean>,
}

/**
 * Inlay hint options used during static or dynamic registration.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct InlayHintRegistrationOptions {
    /// extends InlayHintOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /// extends InlayHintOptions
    /**
     * The server provides support to resolve additional
     * information for an inlay hint item.
     */
    pub resolveProvider: Option<Boolean>,

    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends StaticRegistrationOptions
    /**
     * The id used to register the request. The id can be used to deregister
     * the request again. See also Registration#id.
     */
    pub id: Option<String>,
}

/**
 * A parameter literal used in inlay hint requests.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct InlayHintParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /**
     * The visible document range for which inlay hints should be computed.
     */
    pub range: Range,
}

/// extracted from [InlayHint::label]
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum InlayHintLabel {
    String(String),
    InlayHintLabelPartArray(Vec<InlayHintLabelPart>),
}

/**
 * Inlay hint information.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct InlayHint {
    /**
     * The position of this hint.
     *
     * If multiple hints have the same position, they will be shown in the order
     * they appear in the response.
     */
    pub position: Position,

    /**
     * The label of this hint. A human readable String or an array of
     * InlayHintLabelPart label parts.
     *
     * *Note* that neither the String nor the label part can be empty.
     */
    pub label: InlayHintLabel,

    /**
     * The kind of this hint. Can be omitted in which case the client
     * should fall back to a reasonable default.
     */
    pub kind: Option<InlayHintKind>,

    /**
     * Optional text edits that are performed when accepting this inlay hint.
     *
     * *Note* that edits are expected to change the document so that the inlay
     * hint (or its nearest variant) is now part of the document and the inlay
     * hint itself is now obsolete.
     *
     * Depending on the client capability `inlayHint.resolveSupport` clients
     * might resolve this property late using the resolve request.
     */
    pub textEdits: Option<Vec<TextEdit>>,

    /**
     * The tooltip text when you hover over this item.
     *
     * Depending on the client capability `inlayHint.resolveSupport` clients
     * might resolve this property late using the resolve request.
     */
    pub tooltip: Option<MarkupContentOrString>,

    /**
     * Render padding before the hint.
     *
     * Note: Padding should use the editor's background color, not the
     * background color of the hint itself. That means padding can be used
     * to visually align/separate an inlay hint.
     */
    pub paddingLeft: Option<Boolean>,

    /**
     * Render padding after the hint.
     *
     * Note: Padding should use the editor's background color, not the
     * background color of the hint itself. That means padding can be used
     * to visually align/separate an inlay hint.
     */
    pub paddingRight: Option<Boolean>,
    /**
     * A data entry field that is preserved on an inlay hint between
     * a `textDocument/inlayHint` and a `inlayHint/resolve` request.
     */
    pub data: Option<LSPAny>,
}

/**
 * An inlay hint label part allows for interactive and composite labels
 * of inlay hints.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct InlayHintLabelPart {
    /**
     * The value of this label part.
     */
    pub value: String,

    /**
     * The tooltip text when you hover over this label part. Depending on
     * the client capability `inlayHint.resolveSupport` clients might resolve
     * this property late using the resolve request.
     */
    pub tooltip: Option<MarkupContentOrString>,

    /**
     * An optional source code location that represents this
     * label part.
     *
     * The editor will use this location for the hover and for code navigation
     * features: This part will become a clickable link that resolves to the
     * definition of the symbol at the given location (not necessarily the
     * location itself), it shows the hover that shows at the given location,
     * and it shows a context menu with further code navigation commands.
     *
     * Depending on the client capability `inlayHint.resolveSupport` clients
     * might resolve this property late using the resolve request.
     */
    pub location: Option<Location>,

    /**
     * An optional command for this label part.
     *
     * Depending on the client capability `inlayHint.resolveSupport` clients
     * might resolve this property late using the resolve request.
     */
    pub command: Option<Command>,
}

/**
 * Inlay hint kinds.
 *
 * @since 3.17.0
 */
#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum InlayHintKind {
    /**
     * An inlay hint that for a type annotation.
     */
    Type = 1,

    /**
     * An inlay hint that is for a parameter.
     */
    Parameter = 2,
}

/**
 * Client workspace capabilities specific to inlay hints.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct InlayHintWorkspaceClientCapabilities {
    /**
     * Whether the client implementation supports a refresh request sent from
     * the server to the client.
     *
     * Note that this event is global and will force the client to refresh all
     * inlay hints currently shown. It should be used with absolute care and
     * is useful for situation where a server for example detects a project wide
     * change that requires such a calculation.
     */
    pub refreshSupport: Option<Boolean>,
}

/**
 * Client capabilities specific to inline values.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineValueClientCapabilities {
    /**
     * Whether implementation supports dynamic registration for inline
     * value providers.
     */
    pub dynamicRegistration: Option<Boolean>,
}

/**
 * Inline value options used during static registration.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineValueOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

/**
 * Inline value options used during static or dynamic registration.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineValueRegistrationOptions {
    /// extends InlineValueOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends StaticRegistrationOptions
    /**
     * The id used to register the request. The id can be used to deregister
     * the request again. See also Registration#id.
     */
    pub id: Option<String>,
}

/**
 * A parameter literal used in inline value requests.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineValueParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /**
     * The document range for which inline values should be computed.
     */
    pub range: Range,

    /**
     * Additional information about the context in which inline values were
     * requested.
     */
    pub context: InlineValueContext,
}

/**
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineValueContext {
    /**
     * The stack frame (as a DAP Id) where the execution has stopped.
     */
    pub frameId: Integer,

    /**
     * The document range where execution has stopped.
     * Typically the end position of the range denotes the line where the
     * inline values are shown.
     */
    pub stoppedLocation: Range,
}

/**
 * Provide inline value as text.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineValueText {
    /**
     * The document range for which the inline value applies.
     */
    pub range: Range,

    /**
     * The text of the inline value.
     */
    pub text: String,
}

/**
 * Provide inline value through a variable lookup.
 *
 * If only a range is specified, the variable name will be extracted from
 * the underlying document.
 *
 * An optional variable name can be used to override the extracted name.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineValueVariableLookup {
    /**
     * The document range for which the inline value applies.
     * The range is used to extract the variable name from the underlying
     * document.
     */
    pub range: Range,

    /**
     * If specified the name of the variable to look up.
     */
    pub variableName: Option<String>,

    /**
     * How to perform the lookup.
     */
    pub caseSensitiveLookup: Boolean,
}

/**
 * Provide an inline value through an expression evaluation.
 *
 * If only a range is specified, the expression will be extracted from the
 * underlying document.
 *
 * An optional expression can be used to override the extracted expression.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineValueEvaluatableExpression {
    /**
     * The document range for which the inline value applies.
     * The range is used to extract the evaluatable expression from the
     * underlying document.
     */
    pub range: Range,

    /**
     * If specified the expression overrides the extracted expression.
     */
    pub expression: Option<String>,
}

/**
 * Inline value information can be provided by different means:
 * - directly as a text value (class InlineValueText).
 * - as a name to use for a variable lookup (class InlineValueVariableLookup)
 * - as an evaluatable expression (class InlineValueEvaluatableExpression)
 * The InlineValue types combines all inline value types into one type.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub enum InlineValue {
    InlineValueText(InlineValueText),
    InlineValueVariableLookup(InlineValueVariableLookup),
    InlineValueEvaluatableExpression(InlineValueEvaluatableExpression),
}

/**
 * Client workspace capabilities specific to inline values.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineValueWorkspaceClientCapabilities {
    /**
     * Whether the client implementation supports a refresh request sent from
     * the server to the client.
     *
     * Note that this event is global and will force the client to refresh all
     * inline values currently shown. It should be used with absolute care and
     * is useful for situation where a server for example detect a project wide
     * change that requires such a calculation.
     */
    pub refreshSupport: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonikerClientCapabilities {
    /**
     * Whether implementation supports dynamic registration. If this is set to
     * `true` the client supports the new `(TextDocumentRegistrationOptions &
     * StaticRegistrationOptions)` return value for the corresponding server
     * capability as well.
     */
    pub dynamicRegistration: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonikerOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonikerRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends MonikerOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonikerParams {
    /// extends TextDocumentPositionParams
    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /// extends TextDocumentPositionParams
    /**
     * The position inside the text document.
     */
    pub position: Position,

    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,
}

/**
 * Moniker uniqueness level to define scope of the moniker.
 */
#[derive(Serialize, Deserialize, Debug)]
pub enum UniquenessLevel {
    /**
     * The moniker is only unique inside a document
     */
    #[serde(rename = "document")]
    Document,

    /**
     * The moniker is unique inside a project for which a dump got created
     */
    #[serde(rename = "project")]
    Project,

    /**
     * The moniker is unique inside the group to which a project belongs
     */
    #[serde(rename = "group")]
    Group,

    /**
     * The moniker is unique inside the moniker scheme.
     */
    #[serde(rename = "scheme")]
    Scheme,

    /**
     * The moniker is globally unique
     */
    #[serde(rename = "global")]
    Global,
}

/**
 * The moniker kind.
 */
#[derive(Serialize, Deserialize, Debug)]
pub enum MonikerKind {
    /**
     * The moniker represent a symbol that is imported into a project
     */
    #[serde(rename = "import")]
    Import,

    /**
     * The moniker represents a symbol that is exported from a project
     */
    #[serde(rename = "export")]
    Export,

    /**
     * The moniker represents a symbol that is local to a project (e.g. a local
     * variable of a function, a class not visible outside the project, ...)
     */
    #[serde(rename = "local")]
    Local,
}

/**
 * Moniker definition to match LSIF 0.5 moniker definition.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Moniker {
    /**
     * The scheme of the moniker. For example tsc or .Net
     */
    pub scheme: String,

    /**
     * The identifier of the moniker. The value is opaque in LSIF however
     * schema owners are allowed to define the structure if they want.
     */
    pub identifier: String,

    /**
     * The scope in which the moniker is unique
     */
    pub unique: UniquenessLevel,

    /**
     * The moniker kind if known.
     */
    pub kind: Option<MonikerKind>,
}

/// extracts from [CompletionClientCapabilitiesCompletionItem::tagSupport]
#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionItemTagValueSet {
    /**
     * The tags supported by the client.
     */
    pub valueSet: Vec<CompletionItemTag>,
}

/// extracts from [CompletionClientCapabilitiesCompletionItem::resolveSupport]
#[derive(Serialize, Deserialize, Debug)]
pub struct ResolveSupportProperties {
    /**
     * The properties that a client can resolve lazily.
     */
    pub properties: Vec<String>,
}

/// extracts from [CompletionClientCapabilitiesCompletionItem::insertTextModeSupport]
#[derive(Serialize, Deserialize, Debug)]
pub struct InsertTextModeValueSet {
    pub valueSet: Vec<InsertTextMode>,
}

/// extracts from [CompletionClientCapabilities::completionItem]
#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionClientCapabilitiesCompletionItem {
    /**
     * Client supports snippets as insert text.
     *
     * A snippet can define tab stops and placeholders with `$1`, `$2`
     * and `${3:foo}`. `$0` defines the final tab stop, it defaults to
     * the end of the snippet. Placeholders with equal identifiers are
     * linked, that is typing in one will update others too.
     */
    pub snippetSupport: Option<Boolean>,

    /**
     * Client supports commit characters on a completion item.
     */
    pub commitCharactersSupport: Option<Boolean>,

    /**
     * Client supports the follow content formats for the documentation
     * property. The order describes the preferred format of the client.
     */
    pub documentationFormat: Option<Vec<MarkupKind>>,

    /**
     * Client supports the deprecated property on a completion item.
     */
    pub deprecatedSupport: Option<Boolean>,

    /**
     * Client supports the preselect property on a completion item.
     */
    pub preselectSupport: Option<Boolean>,

    /**
     * Client supports the tag property on a completion item. Clients
     * supporting tags have to handle unknown tags gracefully. Clients
     * especially need to preserve unknown tags when sending a completion
     * item back to the server in a resolve call.
     *
     * @since 3.15.0
     */
    pub tagSupport: Option<CompletionItemTagValueSet>,

    /**
     * Client supports insert replace edit to control different behavior if
     * a completion item is inserted in the text or should replace text.
     *
     * @since 3.16.0
     */
    pub insertReplaceSupport: Option<Boolean>,

    /**
     * Indicates which properties a client can resolve lazily on a
     * completion item. Before version 3.16.0 only the predefined properties
     * `documentation` and `detail` could be resolved lazily.
     *
     * @since 3.16.0
     */
    pub resolveSupport: Option<ResolveSupportProperties>,

    /**
     * The client supports the `insertTextMode` property on
     * a completion item to override the whitespace handling mode
     * as defined by the client (see `insertTextMode`).
     *
     * @since 3.16.0
     */
    pub insertTextModeSupport: Option<InsertTextModeValueSet>,

    /**
     * The client has support for completion item label
     * details (see also `CompletionItemLabelDetails`).
     *
     * @since 3.17.0
     */
    pub labelDetailsSupport: Option<Boolean>,
}

/// extracts from [CompletionClientCapabilities::completionItemKind]
#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionItemKindValueSet {
    /**
     * The completion item kind values the client supports. When this
     * property exists the client also guarantees that it will
     * handle values outside its set gracefully and falls back
     * to a default value when unknown.
     *
     * If this property is not present the client only supports
     * the completion items kinds from `Text` to `Reference` as defined in
     * the initial version of the protocol.
     */
    pub valueSet: Option<Vec<CompletionItemKind>>,
}

/// extracts from [CompletionClientCapabilities::completionList]
#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionClientCapabilitiesCompletionListItemDefaults {
    /**
     * The client supports the following itemDefaults on
     * a completion list.
     *
     * The value lists the supported property names of the
     * `CompletionList.itemDefaults` object. If omitted
     * no properties are supported.
     *
     * @since 3.17.0
     */
    pub itemDefaults: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionClientCapabilities {
    /**
     * Whether completion supports dynamic registration.
     */
    pub dynamicRegistration: Option<Boolean>,

    /**
     * The client supports the following `CompletionItem` specific
     * capabilities.
     */
    pub completionItem: Option<CompletionClientCapabilitiesCompletionItem>,

    pub completionItemKind: Option<CompletionItemKindValueSet>,

    /**
     * The client supports to send additional context information for a
     * `textDocument/completion` request.
     */
    pub contextSupport: Option<Boolean>,

    /**
     * The client's default when the completion item doesn't provide a
     * `insertTextMode` property.
     *
     * @since 3.17.0
     */
    pub insertTextMode: Option<InsertTextMode>,

    /**
     * The client supports the following `CompletionList` specific
     * capabilities.
     *
     * @since 3.17.0
     */
    pub completionList: Option<CompletionClientCapabilitiesCompletionListItemDefaults>,
}

/// extracted from [CompletionOptions::labelDetailsSupport]
#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionItemLabelDetailsSupport {
    /**
     * The server has support for completion item label
     * details (see also `CompletionItemLabelDetails`) when receiving
     * a completion item in a resolve call.
     *
     * @since 3.17.0
     */
    pub labelDetailsSupport: Option<Boolean>,
}

/**
 * Completion options.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /**
     * The additional characters, beyond the defaults provided by the client (typically
     * [a-zA-Z]), that should automatically trigger a completion request. For example
     * `.` in JavaScript represents the beginning of an object property or method and is
     * thus a good candidate for triggering a completion request.
     *
     * Most tools trigger a completion request automatically without explicitly
     * requesting it using a keyboard shortcut (e.g. Ctrl+Space). Typically they
     * do so when the user starts to type an identifier. For example if the user
     * types `c` in a JavaScript file code complete will automatically pop up
     * present `console` besides others as a completion item. Characters that
     * make up identifiers don't need to be listed here.
     */
    pub triggerCharacters: Option<Vec<String>>,

    /**
     * The list of all possible characters that commit a completion. This field
     * can be used if clients don't support individual commit characters per
     * completion item. See client capability
     * `completion.completionItem.commitCharactersSupport`.
     *
     * If a server provides both `allCommitCharacters` and commit characters on
     * an individual completion item the ones on the completion item win.
     *
     * @since 3.2.0
     */
    pub allCommitCharacters: Option<Vec<String>>,

    /**
     * The server provides support to resolve additional
     * information for a completion item.
     */
    pub resolveProvider: Option<Boolean>,

    /**
     * The server supports the following `CompletionItem` specific
     * capabilities.
     *
     * @since 3.17.0
     */
    pub completionItem: Option<CompletionItemLabelDetailsSupport>,
}

pub struct CompletionRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends CompletionOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /// extends CompletionOptions
    /**
     * The additional characters, beyond the defaults provided by the client (typically
     * [a-zA-Z]), that should automatically trigger a completion request. For example
     * `.` in JavaScript represents the beginning of an object property or method and is
     * thus a good candidate for triggering a completion request.
     *
     * Most tools trigger a completion request automatically without explicitly
     * requesting it using a keyboard shortcut (e.g. Ctrl+Space). Typically they
     * do so when the user starts to type an identifier. For example if the user
     * types `c` in a JavaScript file code complete will automatically pop up
     * present `console` besides others as a completion item. Characters that
     * make up identifiers don't need to be listed here.
     */
    pub triggerCharacters: Option<Vec<String>>,

    /// extends CompletionOptions
    /**
     * The list of all possible characters that commit a completion. This field
     * can be used if clients don't support individual commit characters per
     * completion item. See client capability
     * `completion.completionItem.commitCharactersSupport`.
     *
     * If a server provides both `allCommitCharacters` and commit characters on
     * an individual completion item the ones on the completion item win.
     *
     * @since 3.2.0
     */
    pub allCommitCharacters: Option<Vec<String>>,

    /// extends CompletionOptions
    /**
     * The server provides support to resolve additional
     * information for a completion item.
     */
    pub resolveProvider: Option<Boolean>,

    /// extends CompletionOptions
    /**
     * The server supports the following `CompletionItem` specific
     * capabilities.
     *
     * @since 3.17.0
     */
    pub completionItem: Option<CompletionItemLabelDetailsSupport>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionParams {
    /// extends TextDocumentPositionParams
    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /// extends TextDocumentPositionParams
    /**
     * The position inside the text document.
     */
    pub position: Position,

    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,

    /**
     * The completion context. This is only available if the client specifies
     * to send this using the client capability
     * `completion.contextSupport === true`
     */
    pub context: Option<CompletionContext>,
}

/**
 * How a completion was triggered
 */
#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum CompletionTriggerKind {
    /**
     * Completion was triggered by typing an identifier (24x7 code
     * complete), manual invocation (e.g Ctrl+Space) or via API.
     */
    Invoked = 1,

    /**
     * Completion was triggered by a trigger character specified by
     * the `triggerCharacters` properties of the
     * `CompletionRegistrationOptions`.
     */
    TriggerCharacter = 2,

    /**
     * Completion was re-triggered as the current completion list is incomplete.
     */
    TriggerForIncompleteCompletions = 3,
}

/**
 * Contains additional information about the context in which a completion
 * request is triggered.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionContext {
    /**
     * How the completion was triggered.
     */
    pub triggerKind: CompletionTriggerKind,

    /**
     * The trigger character (a single character) that has trigger code
     * complete. Is undefined if
     * `triggerKind !== CompletionTriggerKind.TriggerCharacter`
     */
    pub triggerCharacter: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CompletionListItemDefaultsEditRange {
    Range(Range),
    InsertReplace { insert: Range, replace: Range },
}

/// extracted from [CompletionList::itemDefaults]
#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionListItemDefaults {
    /**
     * A default commit character set.
     *
     * @since 3.17.0
     */
    pub commitCharacters: Option<Vec<String>>,

    /**
     * A default edit range
     *
     * @since 3.17.0
     */
    pub editRange: Option<CompletionListItemDefaultsEditRange>,

    /**
     * A default insert text format
     *
     * @since 3.17.0
     */
    pub insertTextFormat: Option<InsertTextFormat>,

    /**
     * A default insert text mode
     *
     * @since 3.17.0
     */
    pub insertTextMode: Option<InsertTextMode>,

    /**
     * A default data value.
     *
     * @since 3.17.0
     */
    pub data: Option<LSPAny>,
}

/**
 * Represents a collection of [completion items](#CompletionItem) to be
 * presented in the editor.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionList {
    /**
     * This list is not complete. Further typing should result in recomputing
     * this list.
     *
     * Recomputed lists have all their items replaced (not appended) in the
     * incomplete completion sessions.
     */
    pub isIncomplete: Boolean,

    /**
     * In many cases the items of an actual completion result share the same
     * value for properties like `commitCharacters` or the range of a text
     * edit. A completion list can therefore define item defaults which will
     * be used if a completion item itself doesn't specify the value.
     *
     * If a completion list specifies a default value and a completion item
     * also specifies a corresponding value the one from the item is used.
     *
     * Servers are only allowed to return default values if the client
     * signals support for this via the `completionList.itemDefaults`
     * capability.
     *
     * @since 3.17.0
     */
    pub itemDefaults: Option<CompletionListItemDefaults>,

    /**
     * The completion items.
     */
    pub items: Vec<CompletionItem>,
}

/**
 * Defines whether the insert text in a completion item should be interpreted as
 * plain text or a snippet.
 */
#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum InsertTextFormat {
    /**
     * The primary text to be inserted is treated as a plain String.
     */
    PlainText = 1,

    /**
     * The primary text to be inserted is treated as a snippet.
     *
     * A snippet can define tab stops and placeholders with `$1`, `$2`
     * and `${3:foo}`. `$0` defines the final tab stop, it defaults to
     * the end of the snippet. Placeholders with equal identifiers are linked,
     * that is typing in one will update others too.
     */
    Snippet = 2,
}

/**
 * Completion item tags are extra annotations that tweak the rendering of a
 * completion item.
 *
 * @since 3.15.0
 */
#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum CompletionItemTag {
    /**
     * Render a completion as obsolete, usually using a strike-out.
     */
    Deprecated = 1,
}

/**
 * A special text edit to provide an insert and a replace operation.
 *
 * @since 3.16.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct InsertReplaceEdit {
    /**
     * The String to be inserted.
     */
    pub newText: String,

    /**
     * The range if the insert is requested
     */
    pub insert: Range,

    /**
     * The range if the replace is requested.
     */
    pub replace: Range,
}

/**
 * How whitespace and indentation is handled during completion
 * item insertion.
 *
 * @since 3.16.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub enum InsertTextMode {
    /**
     * The insertion or replace strings is taken as it is. If the
     * value is multi line the lines below the cursor will be
     * inserted using the indentation defined in the String value.
     * The client will not apply any kind of adjustments to the
     * String.
     */
    asIs = 1,

    /**
     * The editor adjusts leading whitespace of new lines so that
     * they match the indentation up to the cursor of the line for
     * which the item is accepted.
     *
     * Consider a line like this: <2tabs><cursor><3tabs>foo. Accepting a
     * multi line completion item is indented using 2 tabs and all
     * following lines inserted will be indented using 2 tabs as well.
     */
    adjustIndentation = 2,
}

/**
 * Additional details for a completion item label.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionItemLabelDetails {
    /**
     * An optional String which is rendered less prominently directly after
     * {@link CompletionItem.label label}, without any spacing. Should be
     * used for function signatures or type annotations.
     */
    pub detail: Option<String>,

    /**
     * An optional String which is rendered less prominently after
     * {@link CompletionItemLabelDetails.detail}. Should be used for fully qualified
     * names or file path.
     */
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CompletionItemEditKind {
    TextEdit(TextEdit),
    InsertReplaceEdit(InsertReplaceEdit),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionItem {
    /**
     * The label of this completion item.
     *
     * The label property is also by default the text that
     * is inserted when selecting this completion.
     *
     * If label details are provided the label itself should
     * be an unqualified name of the completion item.
     */
    pub label: String,

    /**
     * Additional details for the label
     *
     * @since 3.17.0
     */
    pub labelDetails: Option<CompletionItemLabelDetails>,
    /**
     * The kind of this completion item. Based of the kind
     * an icon is chosen by the editor. The standardized set
     * of available values is defined in `CompletionItemKind`.
     */
    pub kind: Option<CompletionItemKind>,

    /**
     * Tags for this completion item.
     *
     * @since 3.15.0
     */
    pub tags: Option<Vec<CompletionItemTag>>,

    /**
     * A human-readable String with additional information
     * about this item, like type or symbol information.
     */
    pub detail: Option<String>,

    /**
     * A human-readable String that represents a doc-comment.
     */
    pub documentation: Option<MarkupContentOrString>,

    /**
     * Indicates if this item is deprecated.
     *
     * @deprecated Use `tags` instead if supported.
     */
    pub deprecated: Option<Boolean>,

    /**
     * Select this item when showing.
     *
     * *Note* that only one completion item can be selected and that the
     * tool / client decides which item that is. The rule is that the *first*
     * item of those that match best is selected.
     */
    pub preselect: Option<Boolean>,

    /**
     * A String that should be used when comparing this item
     * with other items. When omitted the label is used
     * as the sort text for this item.
     */
    pub sortText: Option<String>,

    /**
     * A String that should be used when filtering a set of
     * completion items. When omitted the label is used as the
     * filter text for this item.
     */
    pub filterText: Option<String>,

    /**
     * A String that should be inserted into a document when selecting
     * this completion. When omitted the label is used as the insert text
     * for this item.
     *
     * The `insertText` is subject to interpretation by the client side.
     * Some tools might not take the String literally. For example
     * VS Code when code complete is requested in this example
     * `con<cursor position>` and a completion item with an `insertText` of
     * `console` is provided it will only insert `sole`. Therefore it is
     * recommended to use `textEdit` instead since it avoids additional client
     * side interpretation.
     */
    pub insertText: Option<String>,

    /**
     * The format of the insert text. The format applies to both the
     * `insertText` property and the `newText` property of a provided
     * `textEdit`. If omitted defaults to `InsertTextFormat.PlainText`.
     *
     * Please note that the insertTextFormat doesn't apply to
     * `additionalTextEdits`.
     */
    pub insertTextFormat: Option<InsertTextFormat>,

    /**
     * How whitespace and indentation is handled during completion
     * item insertion. If not provided the client's default value depends on
     * the `textDocument.completion.insertTextMode` client capability.
     *
     * @since 3.16.0
     * @since 3.17.0 - support for `textDocument.completion.insertTextMode`
     */
    pub insertTextMode: Option<InsertTextMode>,

    /**
     * An edit which is applied to a document when selecting this completion.
     * When an edit is provided the value of `insertText` is ignored.
     *
     * *Note:* The range of the edit must be a single line range and it must
     * contain the position at which completion has been requested.
     *
     * Most editors support two different operations when accepting a completion
     * item. One is to insert a completion text and the other is to replace an
     * existing text with a completion text. Since this can usually not be
     * predetermined by a server it can report both ranges. Clients need to
     * signal support for `InsertReplaceEdit`s via the
     * `textDocument.completion.completionItem.insertReplaceSupport` client
     * capability property.
     *
     * *Note 1:* The text edit's range as well as both ranges from an insert
     * replace edit must be a [single line] and they must contain the position
     * at which completion has been requested.
     * *Note 2:* If an `InsertReplaceEdit` is returned the edit's insert range
     * must be a prefix of the edit's replace range, that means it must be
     * contained and starting at the same position.
     *
     * @since 3.16.0 additional type `InsertReplaceEdit`
     */
    pub textEdit: Option<CompletionItemKind>,

    /**
     * The edit text used if the completion item is part of a CompletionList and
     * CompletionList defines an item default for the text edit range.
     *
     * Clients will only honor this property if they opt into completion list
     * item defaults using the capability `completionList.itemDefaults`.
     *
     * If not provided and a list's default range is provided the label
     * property is used as a text.
     *
     * @since 3.17.0
     */
    pub textEditText: Option<String>,

    /**
     * An optional array of additional text edits that are applied when
     * selecting this completion. Edits must not overlap (including the same
     * insert position) with the main edit nor with themselves.
     *
     * Additional text edits should be used to change text unrelated to the
     * current cursor position (for example adding an import statement at the
     * top of the file if the completion item will insert an unqualified type).
     */
    pub additionalTextEdits: Option<Vec<TextEdit>>,

    /**
     * An optional set of characters that when pressed while this completion is
     * active will accept it first and then type that character. *Note* that all
     * commit characters should have `length=1` and that superfluous characters
     * will be ignored.
     */
    pub commitCharacters: Option<Vec<String>>,

    /**
     * An optional command that is executed *after* inserting this completion.
     * *Note* that additional modifications to the current document should be
     * described with the additionalTextEdits-property.
     */
    pub command: Option<Command>,

    /**
     * A data entry field that is preserved on a completion item between
     * a completion and a completion resolve request.
     */
    pub data: Option<LSPAny>,
}

/**
 * The kind of a completion entry.
 */
#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Unit = 11,
    Value = 12,
    Enum = 13,
    Keyword = 14,
    Snippet = 15,
    Color = 16,
    File = 17,
    Reference = 18,
    Folder = 19,
    EnumMember = 20,
    Constant = 21,
    Struct = 22,
    Event = 23,
    Operator = 24,
    TypeParameter = 25,
}

/// exctracted from [PublishDiagnosticsClientCapabilities::tagSupport]
#[derive(Serialize, Deserialize, Debug)]
pub struct PublishDiagnosticsClientCapabilitiesTagSupport {
    /**
     * The tags supported by the client.
     */
    pub valueSet: Vec<DiagnosticTag>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublishDiagnosticsClientCapabilities {
    /**
     * Whether the clients accepts diagnostics with related information.
     */
    pub relatedInformation: Option<Boolean>,

    /**
     * Client supports the tag property to provide meta data about a diagnostic.
     * Clients supporting tags have to handle unknown tags gracefully.
     *
     * @since 3.15.0
     */
    pub tagSupport: Option<PublishDiagnosticsClientCapabilitiesTagSupport>,

    /**
     * Whether the client interprets the version property of the
     * `textDocument/publishDiagnostics` notification's parameter.
     *
     * @since 3.15.0
     */
    pub versionSupport: Option<Boolean>,

    /**
     * Client supports a codeDescription property
     *
     * @since 3.16.0
     */
    pub codeDescriptionSupport: Option<Boolean>,

    /**
     * Whether code action supports the `data` property which is
     * preserved between a `textDocument/publishDiagnostics` and
     * `textDocument/codeAction` request.
     *
     * @since 3.16.0
     */
    pub dataSupport: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublishDiagnosticsParams {
    /**
     * The URI for which diagnostic information is reported.
     */
    pub uri: DocumentUri,

    /**
     * Optional the version number of the document the diagnostics are published
     * for.
     *
     * @since 3.15.0
     */
    pub version: Option<Integer>,

    /**
     * An array of diagnostic information items.
     */
    pub diagnostics: Vec<Diagnostic>,
}

/**
 * Client capabilities specific to diagnostic pull requests.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct DiagnosticClientCapabilities {
    /**
     * Whether implementation supports dynamic registration. If this is set to
     * `true` the client supports the new
     * `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
     * return value for the corresponding server capability as well.
     */
    pub dynamicRegistration: Option<Boolean>,

    /**
     * Whether the clients supports related documents for document diagnostic
     * pulls.
     */
    pub relatedDocumentSupport: Option<Boolean>,
}

/**
 * Diagnostic options.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct DiagnosticOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /**
     * An optional identifier under which the diagnostics are
     * managed by the client.
     */
    pub identifier: Option<String>,

    /**
     * Whether the language has inter file dependencies meaning that
     * editing code in one file can result in a different diagnostic
     * set in another file. Inter file dependencies are common for
     * most programming languages and typically uncommon for linters.
     */
    pub interFileDependencies: Boolean,

    /**
     * The server provides support for workspace diagnostics as well.
     */
    pub workspaceDiagnostics: Boolean,
}

/**
 * Diagnostic registration options.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct DiagnosticRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends DiagnosticOptions,
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /// extends DiagnosticOptions,
    /**
     * An optional identifier under which the diagnostics are
     * managed by the client.
     */
    pub identifier: Option<String>,

    /// extends DiagnosticOptions,
    /**
     * Whether the language has inter file dependencies meaning that
     * editing code in one file can result in a different diagnostic
     * set in another file. Inter file dependencies are common for
     * most programming languages and typically uncommon for linters.
     */
    pub interFileDependencies: Boolean,

    /// extends DiagnosticOptions,
    /**
     * The server provides support for workspace diagnostics as well.
     */
    pub workspaceDiagnostics: Boolean,

    /// extends StaticRegistrationOptions
    /**
     * The id used to register the request. The id can be used to deregister
     * the request again. See also Registration#id.
     */
    pub id: Option<String>,
}

/**
 * Parameters of the document diagnostic request.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentDiagnosticParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,

    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /**
     * The additional identifier  provided during registration.
     */
    pub identifier: Option<String>,

    /**
     * The result id of a previous response if provided.
     */
    pub previousResultId: Option<String>,
}

/**
 * The result of a document diagnostic pull request. A report can
 * either be a full report containing all diagnostics for the
 * requested document or a unchanged report indicating that nothing
 * has changed in terms of diagnostics in comparison to the last
 * pull request.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub enum DocumentDiagnosticReport {
    RelatedFullDocumentDiagnosticReport(RelatedFullDocumentDiagnosticReport),
    RelatedUnchangedDocumentDiagnosticReport(RelatedUnchangedDocumentDiagnosticReport),
}

/**
 * The document diagnostic report kinds.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub enum DocumentDiagnosticReportKind {
    /**
     * A diagnostic report with a full
     * set of problems.
     */
    #[serde(rename = "full")]
    Full,

    /**
     * A report indicating that the last
     * returned report is still accurate.
     */
    #[serde(rename = "unchanged")]
    Unchanged,
}

/**
 * A diagnostic report with a full set of problems.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct FullDocumentDiagnosticReport {
    /**
     * A full document diagnostic report.
     */
    /// default = DocumentDiagnosticReportKind.Full
    pub kind: DocumentDiagnosticReportKind,

    /**
     * An optional result id. If provided it will
     * be sent on the next diagnostic request for the
     * same document.
     */
    pub resultId: Option<String>,

    /**
     * The actual items.
     */
    pub items: Vec<Diagnostic>,
}

/**
 * A diagnostic report indicating that the last returned
 * report is still accurate.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct UnchangedDocumentDiagnosticReport {
    /**
     * A document diagnostic report indicating
     * no changes to the last result. A server can
     * only return `unchanged` if result ids are
     * provided.
     */
    /// default = DocumentDiagnosticReportKind.Unchanged
    pub kind: DocumentDiagnosticReportKind,

    /**
     * A result id which will be sent on the next
     * diagnostic request for the same document.
     */
    pub resultId: String,
}

/**
 * A full diagnostic report with a set of related documents.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedFullDocumentDiagnosticReport {
    /// extends FullDocumentDiagnosticReport
    /**
     * A full document diagnostic report.
     */
    /// default = DocumentDiagnosticReportKind.Full
    pub kind: DocumentDiagnosticReportKind,

    /// extends FullDocumentDiagnosticReport
    /**
     * An optional result id. If provided it will
     * be sent on the next diagnostic request for the
     * same document.
     */
    pub resultId: Option<String>,

    /// extends FullDocumentDiagnosticReport
    /**
     * The actual items.
     */
    pub items: Vec<Diagnostic>,

    /**
     * Diagnostics of related documents. This information is useful
     * in programming languages where code in a file A can generate
     * diagnostics in a file B which A depends on. An example of
     * such a language is C/C++ where marco definitions in a file
     * a.cpp and result in errors in a header file b.hpp.
     *
     * @since 3.17.0
     */
    // relatedDocuments?: {
    //     [uri: String /** DocumentUri */]:
    //         FullDocumentDiagnosticReport | UnchangedDocumentDiagnosticReport,
    // },
    pub relatedDocuments: Option<BTreeMap<DocumentUri, DocumentDiagnosticReportKind>>,
}

/**
 * An unchanged diagnostic report with a set of related documents.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedUnchangedDocumentDiagnosticReport {
    /// extends UnchangedDocumentDiagnosticReport
    /**
     * A document diagnostic report indicating
     * no changes to the last result. A server can
     * only return `unchanged` if result ids are
     * provided.
     */
    /// default = DocumentDiagnosticReportKind.Unchanged
    pub kind: DocumentDiagnosticReportKind,

    /// extends UnchangedDocumentDiagnosticReport
    /**
     * A result id which will be sent on the next
     * diagnostic request for the same document.
     */
    pub resultId: String,
    /**
     * Diagnostics of related documents. This information is useful
     * in programming languages where code in a file A can generate
     * diagnostics in a file B which A depends on. An example of
     * such a language is C/C++ where marco definitions in a file
     * a.cpp and result in errors in a header file b.hpp.
     *
     * @since 3.17.0
     */
    // relatedDocuments?: {
    //     [uri: String /** DocumentUri */]:
    //         FullDocumentDiagnosticReport | UnchangedDocumentDiagnosticReport,
    // },
    pub relatedDocuments: Option<BTreeMap<DocumentUri, DocumentDiagnosticReportKind>>,
}

/**
 * A partial result for a document diagnostic report.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentDiagnosticReportPartialResult {
    //     pub relatedDocuments: {
    //         [uri: String /** DocumentUri */]:
    //             FullDocumentDiagnosticReport | UnchangedDocumentDiagnosticReport,
    //     },
    pub relatedDocuments: Option<BTreeMap<DocumentUri, DocumentDiagnosticReportKind>>,
}

/**
 * Cancellation data returned from a diagnostic request.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct DiagnosticServerCancellationData {
    pub retriggerRequest: Boolean,
}

/**
 * Parameters of the workspace diagnostic request.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceDiagnosticParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,

    /**
     * The additional identifier provided during registration.
     */
    pub identifier: Option<String>,

    /**
     * The currently known diagnostic reports with their
     * previous result ids.
     */
    pub previousResultIds: Vec<PreviousResultId>,
}

/**
 * A previous result id in a workspace pull request.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousResultId {
    /**
     * The URI for which the client knows a
     * result id.
     */
    pub uri: DocumentUri,

    /**
     * The value of the previous result id.
     */
    pub value: String,
}

/**
 * A workspace diagnostic report.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceDiagnosticReport {
    pub items: Vec<WorkspaceDocumentDiagnosticReport>,
}

/**
 * A full document diagnostic report for a workspace diagnostic result.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceFullDocumentDiagnosticReport {
    /// extends FullDocumentDiagnosticReport
    /**
     * A full document diagnostic report.
     */
    /// default = DocumentDiagnosticReportKind.Full
    pub kind: DocumentDiagnosticReportKind,

    /// extends FullDocumentDiagnosticReport
    /**
     * An optional result id. If provided it will
     * be sent on the next diagnostic request for the
     * same document.
     */
    pub resultId: Option<String>,

    /// extends FullDocumentDiagnosticReport
    /**
     * The actual items.
     */
    pub items: Vec<Diagnostic>,

    /**
     * The URI for which diagnostic information is reported.
     */
    pub uri: DocumentUri,

    /**
     * The version number for which the diagnostics are reported.
     * If the document is not marked as open `null` can be provided.
     */
    pub version: Option<Integer>,
}

/**
 * An unchanged document diagnostic report for a workspace diagnostic result.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceUnchangedDocumentDiagnosticReport {
    /// extends UnchangedDocumentDiagnosticReport
    /**
     * A document diagnostic report indicating
     * no changes to the last result. A server can
     * only return `unchanged` if result ids are
     * provided.
     */
    /// default = DocumentDiagnosticReportKind.Unchanged
    pub kind: DocumentDiagnosticReportKind,

    /// extends UnchangedDocumentDiagnosticReport
    /**
     * A result id which will be sent on the next
     * diagnostic request for the same document.
     */
    pub resultId: String,

    /**
     * The URI for which diagnostic information is reported.
     */
    pub uri: DocumentUri,

    /**
     * The version number for which the diagnostics are reported.
     * If the document is not marked as open `null` can be provided.
     */
    pub version: Option<Integer>,
}

/**
 * A workspace diagnostic document report.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum WorkspaceDocumentDiagnosticReport {
    WorkspaceFullDocumentDiagnosticReport(WorkspaceFullDocumentDiagnosticReport),
    WorkspaceUnchangedDocumentDiagnosticReport(WorkspaceUnchangedDocumentDiagnosticReport),
}

/**
 * A partial result for a workspace diagnostic report.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceDiagnosticReportPartialResult {
    pub items: Vec<WorkspaceDocumentDiagnosticReport>,
}

/**
 * Workspace client capabilities specific to diagnostic pull requests.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct DiagnosticWorkspaceClientCapabilities {
    /**
     * Whether the client implementation supports a refresh request sent from
     * the server to the client.
     *
     * Note that this event is global and will force the client to refresh all
     * pulled diagnostics currently shown. It should be used with absolute care
     * and is useful for situation where a server for example detects a project
     * wide change that requires such a calculation.
     */
    pub refreshSupport: Option<Boolean>,
}

/// extends from [SignatureHelpClientCapabilitiesSignatureInformation::parameterInformation]
#[derive(Serialize, Deserialize, Debug)]
pub struct SignatureHelpClientCapabilitiesSignatureInformationParameterInformation {
    /**
     * The client supports processing label offsets instead of a
     * simple label String.
     *
     * @since 3.14.0
     */
    pub labelOffsetSupport: Option<Boolean>,
}

/// extends from [SignatureHelpClientCapabilities::signatureInformation]
#[derive(Serialize, Deserialize, Debug)]
pub struct SignatureHelpClientCapabilitiesSignatureInformation {
    /**
     * Client supports the follow content formats for the documentation
     * property. The order describes the preferred format of the client.
     */
    pub documentationFormat: Option<Vec<MarkupKind>>,

    /**
     * Client capabilities specific to parameter information.
     */
    pub parameterInformation:
        Option<SignatureHelpClientCapabilitiesSignatureInformationParameterInformation>,

    /**
     * The client supports the `activeParameter` property on
     * `SignatureInformation` literal.
     *
     * @since 3.16.0
     */
    pub activeParameterSupport: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignatureHelpClientCapabilities {
    /**
     * Whether signature help supports dynamic registration.
     */
    pub dynamicRegistration: Option<Boolean>,

    /**
     * The client supports the following `SignatureInformation`
     * specific properties.
     */
    pub signatureInformation: Option<SignatureHelpClientCapabilitiesSignatureInformation>,

    /**
     * The client supports to send additional context information for a
     * `textDocument/signatureHelp` request. A client that opts into
     * contextSupport will also support the `retriggerCharacters` on
     * `SignatureHelpOptions`.
     *
     * @since 3.15.0
     */
    pub contextSupport: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignatureHelpOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /**
     * The characters that trigger signature help
     * automatically.
     */
    pub triggerCharacters: Option<Vec<String>>,

    /**
     * List of characters that re-trigger signature help.
     *
     * These trigger characters are only active when signature help is already
     * showing. All trigger characters are also counted as re-trigger
     * characters.
     *
     * @since 3.15.0
     */
    pub retriggerCharacters: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignatureHelpRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extend SignatureHelpOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /// extend SignatureHelpOptions
    /**
     * The characters that trigger signature help
     * automatically.
     */
    pub triggerCharacters: Option<Vec<String>>,

    /// extend SignatureHelpOptions
    /**
     * List of characters that re-trigger signature help.
     *
     * These trigger characters are only active when signature help is already
     * showing. All trigger characters are also counted as re-trigger
     * characters.
     *
     * @since 3.15.0
     */
    pub retriggerCharacters: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignatureHelpParams {
    /// extends TextDocumentPositionParams
    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /// extends TextDocumentPositionParams
    /**
     * The position inside the text document.
     */
    pub position: Position,

    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /**
     * The signature help context. This is only available if the client
     * specifies to send this using the client capability
     * `textDocument.signatureHelp.contextSupport === true`
     *
     * @since 3.15.0
     */
    pub context: Option<SignatureHelpContext>,
}

/**
 * How a signature help was triggered.
 *
 * @since 3.15.0
 */
#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum SignatureHelpTriggerKind {
    /**
     * Signature help was invoked manually by the user or by a command.
     */
    Invoked = 1,
    /**
     * Signature help was triggered by a trigger character.
     */
    TriggerCharacter = 2,
    /**
     * Signature help was triggered by the cursor moving or by the document
     * content changing.
     */
    ContentChange = 3,
}

/**
 * Additional information about the context in which a signature help request
 * was triggered.
 *
 * @since 3.15.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct SignatureHelpContext {
    /**
     * Action that caused signature help to be triggered.
     */
    pub triggerKind: SignatureHelpTriggerKind,

    /**
     * Character that caused signature help to be triggered.
     *
     * This is undefined when triggerKind !==
     * SignatureHelpTriggerKind.TriggerCharacter
     */
    pub triggerCharacter: Option<String>,

    /**
     * `true` if signature help was already showing when it was triggered.
     *
     * Retriggers occur when the signature help is already active and can be
     * caused by actions such as typing a trigger character, a cursor move, or
     * document content changes.
     */
    pub isRetrigger: Boolean,

    /**
     * The currently active `SignatureHelp`.
     *
     * The `activeSignatureHelp` has its `SignatureHelp.activeSignature` field
     * updated based on the user navigating through available signatures.
     */
    pub activeSignatureHelp: Option<SignatureHelp>,
}

/**
 * Signature help represents the signature of something
 * callable. There can be multiple signature but only one
 * active and only one active parameter.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct SignatureHelp {
    /**
     * One or more signatures. If no signatures are available the signature help
     * request should return `null`.
     */
    pub signatures: Vec<SignatureInformation>,

    /**
     * The active signature. If omitted or the value lies outside the
     * range of `signatures` the value defaults to zero or is ignore if
     * the `SignatureHelp` as no signatures.
     *
     * Whenever possible implementors should make an active decision about
     * the active signature and shouldn't rely on a default value.
     *
     * In future version of the protocol this property might become
     * mandatory to better express this.
     */
    pub activeSignature: Option<UInteger>,

    /**
     * The active parameter of the active signature. If omitted or the value
     * lies outside the range of `signatures[activeSignature].parameters`
     * defaults to 0 if the active signature has parameters. If
     * the active signature has no parameters it is ignored.
     * In future version of the protocol this property might become
     * mandatory to better express the active parameter if the
     * active signature does have any.
     */
    pub activeParameter: Option<UInteger>,
}

/**
 * Represents the signature of something callable. A signature
 * can have a label, like a function-name, a doc-comment, and
 * a set of parameters.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct SignatureInformation {
    /**
     * The label of this signature. Will be shown in
     * the UI.
     */
    pub label: String,

    /**
     * The human-readable doc-comment of this signature. Will be shown
     * in the UI but can be omitted.
     */
    pub documentation: Option<MarkupContentOrString>,

    /**
     * The parameters of this signature.
     */
    pub parameters: Option<Vec<ParameterInformation>>,

    /**
     * The index of the active parameter.
     *
     * If provided, this is used in place of `SignatureHelp.activeParameter`.
     *
     * @since 3.16.0
     */
    pub activeParameter: Option<UInteger>,
}

/// extracted from [ParameterInformation::label]
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ParameterInformationLabel {
    String(String),
    StartEndOffsets(UInteger, UInteger),
}

/// extracted from [ParameterInformation::documentation] (and several more places)
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MarkupContentOrString {
    String(String),
    MarkupContent(MarkupContent),
}

/**
 * Represents a parameter of a callable-signature. A parameter can
 * have a label and a doc-comment.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct ParameterInformation {
    /**
     * The label of this parameter information.
     *
     * Either a String or an inclusive start and exclusive end offsets within
     * its containing signature label. (see SignatureInformation.label). The
     * offsets are based on a UTF-16 String representation as `Position` and
     * `Range` does.
     *
     * *Note*: a label of type String should be a substring of its containing
     * signature label. Its intended use case is to highlight the parameter
     * label part in the `SignatureInformation.label`.
     */
    // label: String | [UInteger, UInteger],
    pub label: ParameterInformationLabel,

    /**
     * The human-readable doc-comment of this parameter. Will be shown
     * in the UI but can be omitted.
     */
    pub documentation: Option<MarkupContentOrString>,
}

/// extracted from [CodeActionClientCapabilities::resolveSupport]
#[derive(Serialize, Deserialize, Debug)]
pub struct CodeActionClientCapabilitiesResolveSupport {
    /**
     * The properties that a client can resolve lazily.
     */
    pub properties: Vec<String>,
}

/// extracted from [CodeActionClientCapabilities::codeActionLiteralSupport]
#[derive(Serialize, Deserialize, Debug)]
pub struct CodeActionClientCapabilitiesCodeActionKind {
    /**
     * The code action kind values the client supports. When this
     * property exists the client also guarantees that it will
     * handle values outside its set gracefully and falls back
     * to a default value when unknown.
     */
    pub valueSet: Vec<CodeActionKind>,
}

/// extracted from [CodeActionClientCapabilities::codeActionLiteralSupport]
#[derive(Serialize, Deserialize, Debug)]
pub struct CodeActionClientCapabilitiesCodeActionLiteralSupport {
    /**
     * The code action kind is supported with the following value
     * set.
     */
    pub codeActionKind: CodeActionClientCapabilitiesCodeActionKind,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeActionClientCapabilities {
    /**
     * Whether code action supports dynamic registration.
     */
    pub dynamicRegistration: Option<Boolean>,

    /**
     * The client supports code action literals as a valid
     * response of the `textDocument/codeAction` request.
     *
     * @since 3.8.0
     */
    pub codeActionLiteralSupport: Option<CodeActionClientCapabilitiesCodeActionLiteralSupport>,

    /**
     * Whether code action supports the `isPreferred` property.
     *
     * @since 3.15.0
     */
    pub isPreferredSupport: Option<Boolean>,

    /**
     * Whether code action supports the `disabled` property.
     *
     * @since 3.16.0
     */
    pub disabledSupport: Option<Boolean>,

    /**
     * Whether code action supports the `data` property which is
     * preserved between a `textDocument/codeAction` and a
     * `codeAction/resolve` request.
     *
     * @since 3.16.0
     */
    pub dataSupport: Option<Boolean>,
    /**
     * Whether the client supports resolving additional code action
     * properties via a separate `codeAction/resolve` request.
     *
     * @since 3.16.0
     */
    pub resolveSupport: Option<CodeActionClientCapabilitiesResolveSupport>,

    /**
     * Whether the client honors the change annotations in
     * text edits and resource operations returned via the
     * `CodeAction#edit` property by for example presenting
     * the workspace edit in the user struct and asking
     * for confirmation.
     *
     * @since 3.16.0
     */
    pub honorsChangeAnnotations: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeActionOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /**
     * CodeActionKinds that this server may return.
     *
     * The list of kinds may be generic, such as `CodeActionKind.Refactor`,
     * or the server may list out every specific kind they provide.
     */
    pub codeActionKinds: Option<Vec<CodeActionKind>>,

    /**
     * The server provides support to resolve additional
     * information for a code action.
     *
     * @since 3.16.0
     */
    pub resolveProvider: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeActionRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends CodeActionOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /// extends CodeActionOptions
    /**
     * CodeActionKinds that this server may return.
     *
     * The list of kinds may be generic, such as `CodeActionKind.Refactor`,
     * or the server may list out every specific kind they provide.
     */
    pub codeActionKinds: Option<Vec<CodeActionKind>>,

    /// extends CodeActionOptions
    /**
     * The server provides support to resolve additional
     * information for a code action.
     *
     * @since 3.16.0
     */
    pub resolveProvider: Option<Boolean>,
}

/**
 * Params for the CodeActionRequest
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct CodeActionParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,
    /**
     * The document in which the command was invoked.
     */
    pub textDocument: TextDocumentIdentifier,

    /**
     * The range for which the command was invoked.
     */
    pub range: Range,

    /**
     * Context carrying additional information.
     */
    pub context: CodeActionContext,
}

/**
 * The kind of a code action.
 *
 * Kinds are a hierarchical list of identifiers separated by `.`,
 * e.g. `"refactor.extract.function"`.
 *
 * The set of kinds is open and client needs to announce the kinds it supports
 * to the server during initialization.
 */
/**
 * A set of predefined code action kinds.
 */
#[derive(Serialize, Deserialize, Debug)]
pub enum CodeActionKind {
    /**
     * Empty kind.
     */
    #[serde(rename = "")]
    Empty,

    /**
     * Base kind for quickfix actions: 'quickfix'.
     */
    #[serde(rename = "quickfix")]
    QuickFix,

    /**
     * Base kind for refactoring actions: 'refactor'.
     */
    #[serde(rename = "refactor")]
    Refactor,

    /**
     * Base kind for refactoring extraction actions: 'refactor.extract'.
     *
     * Example extract actions:
     *
     * - Extract method
     * - Extract function
     * - Extract variable
     * - Extract struct from class
     * - ...
     */
    #[serde(rename = "refactor.extract")]
    RefactorExtract,

    /**
     * Base kind for refactoring inline actions: 'refactor.inline'.
     *
     * Example inline actions:
     *
     * - Inline function
     * - Inline variable
     * - Inline constant
     * - ...
     */
    #[serde(rename = "refactor.inline")]
    RefactorInline,

    /**
     * Base kind for refactoring rewrite actions: 'refactor.rewrite'.
     *
     * Example rewrite actions:
     *
     * - Convert JavaScript function to class
     * - Add or remove parameter
     * - Encapsulate field
     * - Make method static
     * - Move method to base class
     * - ...
     */
    #[serde(rename = "refactor.rewrite")]
    RefactorRewrite,

    /**
     * Base kind for source actions: `source`.
     *
     * Source code actions apply to the entire file.
     */
    #[serde(rename = "source")]
    Source,

    /**
     * Base kind for an organize imports source action:
     * `source.organizeImports`.
     */
    #[serde(rename = "source.organizeImports")]
    SourceOrganizeImports,

    /**
     * Base kind for a 'fix all' source action: `source.fixAll`.
     *
     * 'Fix all' actions automatically fix errors that have a clear fix that
     * do not require user input. They should not suppress errors or perform
     * unsafe fixes such as generating new types or classes.
     *
     * @since 3.17.0
     */
    #[serde(rename = "source.fixAll")]
    SourceFixAll,
}

/**
 * Contains additional diagnostic information about the context in which
 * a code action is run.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct CodeActionContext {
    /**
     * An array of diagnostics known on the client side overlapping the range
     * provided to the `textDocument/codeAction` request. They are provided so
     * that the server knows which errors are currently presented to the user
     * for the given range. There is no guarantee that these accurately reflect
     * the error state of the resource. The primary parameter
     * to compute code actions is the provided range.
     */
    pub diagnostics: Vec<Diagnostic>,

    /**
     * Requested kind of actions to return.
     *
     * Actions not of this kind are filtered out by the client before being
     * shown. So servers can omit computing them.
     */
    pub only: Option<Vec<CodeActionKind>>,

    /**
     * The reason why code actions were requested.
     *
     * @since 3.17.0
     */
    pub triggerKind: Option<CodeActionTriggerKind>,
}

/**
 * The reason why code actions were requested.
 *
 * @since 3.17.0
 */
#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum CodeActionTriggerKind {
    /**
     * Code actions were explicitly requested by the user or by an extension.
     */
    Invoked = 1,

    /**
     * Code actions were requested automatically.
     *
     * This typically happens when current selection in a file changes, but can
     * also be triggered when file content changes.
     */
    Automatic = 2,
}

/// extracted from CodeAction
#[derive(Serialize, Deserialize, Debug)]
pub struct CodeActionDisabled {
    /**
     * Human readable description of why the code action is currently
     * disabled.
     *
     * This is displayed in the code actions UI.
     */
    pub reason: String,
}

/**
 * A code action represents a change that can be performed in code, e.g. to fix
 * a problem or to refactor code.
 *
 * A CodeAction must set either `edit` and/or a `command`. If both are supplied,
 * the `edit` is applied first, then the `command` is executed.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct CodeAction {
    /**
     * A short, human-readable, title for this code action.
     */
    pub title: String,

    /**
     * The kind of the code action.
     *
     * Used to filter code actions.
     */
    pub kind: Option<CodeActionKind>,

    /**
     * The diagnostics that this code action resolves.
     */
    pub diagnostics: Option<Vec<Diagnostic>>,

    /**
     * Marks this as a preferred action. Preferred actions are used by the
     * `auto fix` command and can be targeted by keybindings.
     *
     * A quick fix should be marked preferred if it properly addresses the
     * underlying error. A refactoring should be marked preferred if it is the
     * most reasonable choice of actions to take.
     *
     * @since 3.15.0
     */
    pub isPreferred: Option<Boolean>,

    /**
     * Marks that the code action cannot currently be applied.
     *
     * Clients should follow the following guidelines regarding disabled code
     * actions:
     *
     * - Disabled code actions are not shown in automatic lightbulbs code
     *   action menus.
     *
     * - Disabled actions are shown as faded out in the code action menu when
     *   the user request a more specific type of code action, such as
     *   refactorings.
     *
     * - If the user has a keybinding that auto applies a code action and only
     *   a disabled code actions are returned, the client should show the user
     *   an error message with `reason` in the editor.
     *
     * @since 3.16.0
     */
    pub disabled: Option<CodeActionDisabled>,

    /**
     * The workspace edit this code action performs.
     */
    pub edit: Option<WorkspaceEdit>,

    /**
     * A command this code action executes. If a code action
     * provides an edit and a command, first the edit is
     * executed and then the command.
     */
    pub command: Option<Command>,

    /**
     * A data entry field that is preserved on a code action between
     * a `textDocument/codeAction` and a `codeAction/resolve` request.
     *
     * @since 3.16.0
     */
    pub data: Option<LSPAny>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentColorClientCapabilities {
    /**
     * Whether document color supports dynamic registration.
     */
    pub dynamicRegistration: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentColorOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentColorRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends StaticRegistrationOptions
    /**
     * The id used to register the request. The id can be used to deregister
     * the request again. See also Registration#id.
     */
    pub id: Option<String>,

    /// extends DocumentColorOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentColorParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,

    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColorInformation {
    /**
     * The range in the document where this color appears.
     */
    pub range: Range,

    /**
     * The actual color value for this color range.
     */
    pub color: Color,
}

/**
 * Represents a color in RGBA space.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Color {
    /**
     * The red component of this color in the range [0-1].
     */
    /// **readonly**
    pub red: Decimal,

    /**
     * The green component of this color in the range [0-1].
     */
    /// **readonly**
    pub green: Decimal,

    /**
     * The blue component of this color in the range [0-1].
     */
    /// **readonly**
    pub blue: Decimal,

    /**
     * The alpha component of this color in the range [0-1].
     */
    /// **readonly**
    pub alpha: Decimal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColorPresentationParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,

    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /**
     * The color information to request presentations for.
     */
    pub color: Color,

    /**
     * The range where the color would be inserted. Serves as a context.
     */
    pub range: Range,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColorPresentation {
    /**
     * The label of this color presentation. It will be shown on the color
     * picker header. By default this is also the text that is inserted when
     * selecting this color presentation.
     */
    pub label: String,
    /**
     * An [edit](#TextEdit) which is applied to a document when selecting
     * this presentation for the color. When omitted the
     * [label](#ColorPresentation.label) is used.
     */
    pub textEdit: Option<TextEdit>,
    /**
     * An optional array of additional [text edits](#TextEdit) that are applied
     * when selecting this color presentation. Edits must not overlap with the
     * main [edit](#ColorPresentation.textEdit) nor with themselves.
     */
    pub additionalTextEdits: Option<Vec<TextEdit>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentFormattingClientCapabilities {
    /**
     * Whether formatting supports dynamic registration.
     */
    pub dynamicRegistration: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentFormattingOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentFormattingRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends DocumentFormattingOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentFormattingParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /**
     * The document to format.
     */
    pub textDocument: TextDocumentIdentifier,

    /**
     * The format options.
     */
    pub options: FormattingOptions,
}

/**
 * Value-object describing what options formatting should use.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct FormattingOptions {
    /**
     * Size of a tab in spaces.
     */
    pub tabSize: UInteger,

    /**
     * Prefer spaces over tabs.
     */
    pub insertSpaces: Boolean,

    /**
     * Trim trailing whitespace on a line.
     *
     * @since 3.15.0
     */
    pub trimTrailingWhitespace: Option<Boolean>,

    /**
     * Insert a newline character at the end of the file if one does not exist.
     *
     * @since 3.15.0
     */
    pub insertFinalNewline: Option<Boolean>,

    /**
     * Trim all newlines after the final newline at the end of the file.
     *
     * @since 3.15.0
     */
    pub trimFinalNewlines: Option<Boolean>,

    /**
     * Signature for further properties.
     */
    /// [key: String]: Boolean | Integer | String,
    #[serde(flatten)]
    pub additional_properties: BTreeMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentRangeFormattingClientCapabilities {
    /**
     * Whether formatting supports dynamic registration.
     */
    pub dynamicRegistration: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentRangeFormattingOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentRangeFormattingRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends DocumentRangeFormattingOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentRangeFormattingParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /**
     * The document to format.
     */
    pub textDocument: TextDocumentIdentifier,

    /**
     * The range to format
     */
    pub range: Range,

    /**
     * The format options
     */
    pub options: FormattingOptions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentOnTypeFormattingClientCapabilities {
    /**
     * Whether on type formatting supports dynamic registration.
     */
    pub dynamicRegistration: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentOnTypeFormattingOptions {
    /**
     * A character on which formatting should be triggered, like `{`.
     */
    pub firstTriggerCharacter: String,

    /**
     * More trigger characters.
     */
    pub moreTriggerCharacter: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentOnTypeFormattingRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends DocumentOnTypeFormattingOptions
    /**
     * A character on which formatting should be triggered, like `{`.
     */
    pub firstTriggerCharacter: String,

    /**
     * More trigger characters.
     */
    pub moreTriggerCharacter: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentOnTypeFormattingParams {
    /**
     * The document to format.
     */
    pub textDocument: TextDocumentIdentifier,

    /**
     * The position around which the on type formatting should happen.
     * This is not necessarily the exact position where the character denoted
     * by the property `ch` got typed.
     */
    pub position: Position,

    /**
     * The character that has been typed that triggered the formatting
     * on type request. That is not necessarily the last character that
     * got inserted into the document since the client could auto insert
     * characters as well (e.g. like automatic brace completion).
     */
    pub ch: String,

    /**
     * The formatting options.
     */
    pub options: FormattingOptions,
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum PrepareSupportDefaultBehavior {
    /**
     * The client's default behavior is to select the identifier
     * according to the language's syntax rule.
     */
    Identifier = 1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RenameClientCapabilities {
    /**
     * Whether rename supports dynamic registration.
     */
    pub dynamicRegistration: Option<Boolean>,

    /**
     * Client supports testing for validity of rename operations
     * before execution.
     *
     * @since version 3.12.0
     */
    pub prepareSupport: Option<Boolean>,

    /**
     * Client supports the default behavior result
     * (`{ defaultBehavior: Boolean }`).
     *
     * The value indicates the default behavior used by the
     * client.
     *
     * @since version 3.16.0
     */
    pub prepareSupportDefaultBehavior: Option<PrepareSupportDefaultBehavior>,

    /**
     * Whether the client honors the change annotations in
     * text edits and resource operations returned via the
     * rename request's workspace edit by for example presenting
     * the workspace edit in the user struct and asking
     * for confirmation.
     *
     * @since 3.16.0
     */
    pub honorsChangeAnnotations: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RenameOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /**
     * Renames should be checked and tested before being executed.
     */
    pub prepareProvider: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RenameRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends RenameOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /**
     * Renames should be checked and tested before being executed.
     */
    pub prepareProvider: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RenameParams {
    /// extends TextDocumentPositionParams
    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /// extends TextDocumentPositionParams
    /**
     * The position inside the text document.
     */
    pub position: Position,

    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /**
     * The new name of the symbol. If the given name is not valid the
     * request must return a [ResponseError](#ResponseError) with an
     * appropriate message set.
     */
    pub newName: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrepareRenameParams {
    /// extends TextDocumentPositionParams
    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /// extends TextDocumentPositionParams
    /**
     * The position inside the text document.
     */
    pub position: Position,

    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkedEditingRangeClientCapabilities {
    /**
     * Whether the implementation supports dynamic registration.
     * If this is set to `true` the client supports the new
     * `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
     * return value for the corresponding server capability as well.
     */
    pub dynamicRegistration: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkedEditingRangeOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkedEditingRangeRegistrationOptions {
    /// extends TextDocumentRegistrationOptions
    /**
     * A document selector to identify the scope of the registration. If set to
     * null the document selector provided on the client side will be used.
     */
    pub documentSelector: Option<DocumentSelector>,

    /// extends LinkedEditingRangeOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /// extends StaticRegistrationOptions
    /**
     * The id used to register the request. The id can be used to deregister
     * the request again. See also Registration#id.
     */
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkedEditingRangeParams {
    /// extends TextDocumentPositionParams
    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,

    /// extends TextDocumentPositionParams
    /**
     * The position inside the text document.
     */
    pub position: Position,

    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkedEditingRanges {
    /**
     * A list of ranges that can be renamed together. The ranges must have
     * identical length and contain identical text content. The ranges cannot
     * overlap.
     */
    pub ranges: Vec<Range>,

    /**
     * An optional word pattern (regular expression) that describes valid
     * contents for the given ranges. If no pattern is provided, the client
     * configuration's word pattern will be used.
     */
    pub wordPattern: Option<String>,
}

/// extracted from WorkspaceSymbolClientCapabilities
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceSymbolClientCapabilitiesSymbolKind {
    /**
     * The symbol kind values the client supports. When this
     * property exists the client also guarantees that it will
     * handle values outside its set gracefully and falls back
     * to a default value when unknown.
     *
     * If this property is not present the client only supports
     * the symbol kinds from `File` to `Array` as defined in
     * the initial version of the protocol.
     */
    pub valueSet: Option<Vec<SymbolKind>>,
}

/// extracted from WorkspaceSymbolClientCapabilities
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceSymbolClientCapabilitiesTagSupport {
    /**
     * The tags supported by the client.
     */
    pub valueSet: Vec<SymbolTag>,
}

/// extracted from WorkspaceSymbolClientCapabilities
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceSymbolClientCapabilitiesResolveSupport {
    /**
     * The properties that a client can resolve lazily. Usually
     * `location.range`
     */
    pub properties: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceSymbolClientCapabilities {
    /**
     * Symbol request supports dynamic registration.
     */
    pub dynamicRegistration: Option<Boolean>,

    /**
     * Specific capabilities for the `SymbolKind` in the `workspace/symbol`
     * request.
     */
    pub symbolKind: Option<WorkspaceSymbolClientCapabilitiesSymbolKind>,

    /**
     * The client supports tags on `SymbolInformation` and `WorkspaceSymbol`.
     * Clients supporting tags have to handle unknown tags gracefully.
     *
     * @since 3.16.0
     */
    pub tagSupport: Option<WorkspaceSymbolClientCapabilitiesTagSupport>,

    /**
     * The client support partial workspace symbols. The client will send the
     * request `workspaceSymbol/resolve` to the server to resolve additional
     * properties.
     *
     * @since 3.17.0 - proposedState
     */
    pub resolveSupport: Option<WorkspaceSymbolClientCapabilitiesResolveSupport>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceSymbolOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /**
     * The server provides support to resolve additional
     * information for a workspace symbol.
     *
     * @since 3.17.0
     */
    pub resolveProvider: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceSymbolRegistrationOptions {
    /// extends WorkspaceSymbolOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,

    /// extends WorkspaceSymbolOptions
    /**
     * The server provides support to resolve additional
     * information for a workspace symbol.
     *
     * @since 3.17.0
     */
    pub resolveProvider: Option<Boolean>,
}

/**
 * The parameters of a Workspace Symbol Request.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceSymbolParams {
    /// extends WorkDoneProgressParams,
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,

    /// extends PartialResultParams
    /**
     * An optional token that a server can use to report partial results (e.g.
     * streaming) to the client.
     */
    pub partialResultToken: Option<ProgressToken>,

    /**
     * A query String to filter symbols by. Clients may send an empty
     * String here to request all symbols.
     */
    pub query: String,
}

/// extracted from [WorkspaceSymbol::location]
#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentUriObject {
    pub uri: DocumentUri,
}

/// extracted from [WorkspaceSymbol::location]
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum WorkspaceSymbolLocation {
    Location(Location),
    DocumentUriObject(DocumentUriObject),
}

/**
 * A special workspace symbol that supports locations without a range
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceSymbol {
    /**
     * The name of this symbol.
     */
    pub name: String,

    /**
     * The kind of this symbol.
     */
    pub kind: SymbolKind,

    /**
     * Tags for this completion item.
     */
    pub tags: Option<Vec<SymbolTag>>,

    /**
     * The name of the symbol containing this symbol. This information is for
     * user struct purposes (e.g. to render a qualifier in the user interface
     * if necessary). It can't be used to re-infer a hierarchy for the document
     * symbols.
     */
    pub containerName: Option<String>,

    /**
     * The location of this symbol. Whether a server is allowed to
     * return a location without a range depends on the client
     * capability `workspace.symbol.resolveSupport`.
     *
     * See also `SymbolInformation.location`.
     */
    pub location: WorkspaceSymbolLocation,

    /**
     * A data entry field that is preserved on a workspace symbol between a
     * workspace symbol request and a workspace symbol resolve request.
     */
    pub data: Option<LSPAny>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigurationParams {
    pub items: Vec<ConfigurationItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigurationItem {
    /**
     * The scope to get the configuration section for.
     */
    pub scopeUri: Option<URI>,

    /**
     * The configuration section asked for.
     */
    pub section: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DidChangeConfigurationClientCapabilities {
    /**
     * Did change configuration notification supports dynamic registration.
     *
     * @since 3.6.0 to support the new pull model.
     */
    pub dynamicRegistration: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DidChangeConfigurationParams {
    /**
     * The actual changed settings
     */
    pub settings: LSPAny,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChangeNotifications {
    String(String),
    Boolean(Boolean),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceFoldersServerCapabilities {
    /**
     * The server has support for workspace folders
     */
    pub supported: Option<Boolean>,

    /**
     * Whether the server wants to receive workspace folder
     * change notifications.
     *
     * If a String is provided, the String is treated as an ID
     * under which the notification is registered on the client
     * side. The ID can be used to unregister for these events
     * using the `client/unregisterCapability` request.
     */
    pub changeNotifications: Option<ChangeNotifications>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceFolder {
    /**
     * The associated URI for this workspace folder.
     */
    pub uri: URI,

    /**
     * The name of the workspace folder. Used to refer to this
     * workspace folder in the user interface.
     */
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DidChangeWorkspaceFoldersParams {
    /**
     * The actual workspace folder change event.
     */
    pub event: WorkspaceFoldersChangeEvent,
}

/**
 * The workspace folder change event.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceFoldersChangeEvent {
    /**
     * The array of added workspace folders
     */
    pub added: Vec<WorkspaceFolder>,

    /**
     * The array of the removed workspace folders
     */
    pub removed: Vec<WorkspaceFolder>,
}

/**
 * The options to register for file operations.
 *
 * @since 3.16.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct FileOperationRegistrationOptions {
    /**
     * The actual filters.
     */
    pub filters: Vec<FileOperationFilter>,
}

/**
 * A pattern kind describing if a glob pattern matches a file a folder or
 * both.
 *
 * @since 3.16.0
 */
/// pub type FileOperationPatternKind = 'file' | 'folder';
#[derive(Serialize, Deserialize, Debug)]
pub enum FileOperationPatternKind {
    /**
     * The pattern matches a file only.
     */
    #[serde(rename = "file")]
    File,
    /**
     * The pattern matches a folder only.
     */
    #[serde(rename = "folder")]
    Folder,
}

/**
 * Matching options for the file operation pattern.
 *
 * @since 3.16.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct FileOperationPatternOptions {
    /**
     * The pattern should be matched ignoring casing.
     */
    pub ignoreCase: Option<Boolean>,
}

/**
 * A pattern to describe in which file operation requests or notifications
 * the server is interested in.
 *
 * @since 3.16.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct FileOperationPattern {
    /**
     * The glob pattern to match. Glob patterns can have the following syntax:
     * - `*` to match one or more characters in a path segment
     * - `?` to match on one character in a path segment
     * - `**` to match any number of path segments, including none
     * - `{}` to group sub patterns into an OR expression. (e.g. `**[FORWARD_SLASH]*.{ts,js}`
     *   matches all TypeScript and JavaScript files)
     * - `[]` to declare a range of characters to match in a path segment
     *   (e.g., `example.[0-9]` to match on `example.0`, `example.1`, …)
     * - `[!...]` to negate a range of characters to match in a path segment
     *   (e.g., `example.[!0-9]` to match on `example.a`, `example.b`, but
     *   not `example.0`)
     */
    pub glob: String,

    /**
     * Whether to match files or folders with this pattern.
     *
     * Matches both if undefined.
     */
    pub matches: Option<FileOperationPatternKind>,

    /**
     * Additional options used during matching.
     */
    pub options: Option<FileOperationPatternOptions>,
}

/**
 * A filter to describe in which file operation requests or notifications
 * the server is interested in.
 *
 * @since 3.16.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct FileOperationFilter {
    /**
     * A Uri like `file` or `untitled`.
     */
    pub scheme: Option<String>,

    /**
     * The actual file operation pattern.
     */
    pub pattern: FileOperationPattern,
}

/**
 * The parameters sent in notifications/requests for user-initiated creation
 * of files.
 *
 * @since 3.16.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFilesParams {
    /**
     * An array of all files/folders created in this operation.
     */
    pub files: Vec<FileCreate>,
}

/**
 * Represents information on a file/folder create.
 *
 * @since 3.16.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct FileCreate {
    /**
     * A file:// URI for the location of the file/folder being created.
     */
    pub uri: String,
}

/**
 * The parameters sent in notifications/requests for user-initiated renames
 * of files.
 *
 * @since 3.16.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct RenameFilesParams {
    /**
     * An array of all files/folders renamed in this operation. When a folder
     * is renamed, only the folder will be included, and not its children.
     */
    pub files: Vec<FileRename>,
}

/**
 * Represents information on a file/folder rename.
 *
 * @since 3.16.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct FileRename {
    /**
     * A file:// URI for the original location of the file/folder being renamed.
     */
    pub oldUri: String,

    /**
     * A file:// URI for the new location of the file/folder being renamed.
     */
    pub newUri: String,
}

/**
 * The parameters sent in notifications/requests for user-initiated deletes
 * of files.
 *
 * @since 3.16.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteFilesParams {
    /**
     * An array of all files/folders deleted in this operation.
     */
    pub files: Vec<FileDelete>,
}

/**
 * Represents information on a file/folder delete.
 *
 * @since 3.16.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct FileDelete {
    /**
     * A file:// URI for the location of the file/folder being deleted.
     */
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DidChangeWatchedFilesClientCapabilities {
    /**
     * Did change watched files notification supports dynamic registration.
     * Please note that the current protocol doesn't support static
     * configuration for file changes from the server side.
     */
    pub dynamicRegistration: Option<Boolean>,

    /**
     * Whether the client has support for relative patterns
     * or not.
     *
     * @since 3.17.0
     */
    pub relativePatternSupport: Option<Boolean>,
}

/**
 * Describe options to be used when registering for file system change events.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct DidChangeWatchedFilesRegistrationOptions {
    /**
     * The watchers to register.
     */
    pub watchers: Vec<FileSystemWatcher>,
}

/**
 * The glob pattern to watch relative to the base path. Glob patterns can have
 * the following syntax:
 * - `*` to match one or more characters in a path segment
 * - `?` to match on one character in a path segment
 * - `**` to match any number of path segments, including none
 * - `{}` to group conditions (e.g. `**[FORWARD_SLASH]*.{ts,js}` matches all TypeScript
 *   and JavaScript files)
 * - `[]` to declare a range of characters to match in a path segment
 *   (e.g., `example.[0-9]` to match on `example.0`, `example.1`, …)
 * - `[!...]` to negate a range of characters to match in a path segment
 *   (e.g., `example.[!0-9]` to match on `example.a`, `example.b`,
 *   but not `example.0`)
 *
 * @since 3.17.0
 */
pub type Pattern = String;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RelativePatternBaseURI {
    WorkspaceFolder(WorkspaceFolder),
    URI(URI),
}

/**
 * A relative pattern is a helper to construct glob patterns that are matched
 * relatively to a base URI. The common value for a `baseUri` is a workspace
 * folder root, but it can be another absolute URI as well.
 *
 * @since 3.17.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct RelativePattern {
    /**
     * A workspace folder or a base URI to which this pattern will be matched
     * against relatively.
     */
    pub baseUri: RelativePatternBaseURI,

    /**
     * The actual glob pattern,
     */
    pub pattern: Pattern,
}

/**
 * The glob pattern. Either a String pattern or a relative pattern.
 *
 * @since 3.17.0
 */
/// pub type GlobPattern = Pattern | RelativePattern;
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum GlobPattern {
    Pattern(Pattern),
    RelativePattern(RelativePattern),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileSystemWatcher {
    /**
     * The glob pattern to watch. See {@link GlobPattern glob pattern}
     * for more detail.
     *
     * @since 3.17.0 support for relative patterns.
     */
    pub globPattern: GlobPattern,

    /**
     * The kind of events of interest. If omitted it defaults
     * to WatchKind.Create | WatchKind.Change | WatchKind.Delete
     * which is 7.
     */
    pub kind: Option<WatchKind>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum WatchKind {
    /**
     * Interested in create events.
     */
    Create = 1,

    /**
     * Interested in change events
     */
    Change = 2,

    /**
     * Interested in delete events
     */
    Delete = 4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DidChangeWatchedFilesParams {
    /**
     * The actual file events.
     */
    pub changes: Vec<FileEvent>,
}

/**
 * An event describing a file change.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct FileEvent {
    /**
     * The file's URI.
     */
    pub uri: DocumentUri,
    /**
     * The change type.
     */
    pub r#type: FileChangeType,
}

/**
 * The file event type.
 */
#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum FileChangeType {
    /**
     * The file got created.
     */
    Created = 1,
    /**
     * The file got changed.
     */
    Changed = 2,
    /**
     * The file got deleted.
     */
    Deleted = 3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecuteCommandClientCapabilities {
    /**
     * Execute command supports dynamic registration.
     */
    pub dynamicRegistration: Option<Boolean>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecuteCommandOptions {
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
    /**
     * The commands to be executed on the server
     */
    pub commands: Vec<String>,
}

/**
 * Execute command registration options.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct ExecuteCommandRegistrationOptions {
    /// extends extends ExecuteCommandOptions
    /// extends WorkDoneProgressOptions
    pub workDoneProgress: Option<Boolean>,
    /// extends extends ExecuteCommandOptions
    /**
     * The commands to be executed on the server
     */
    pub commands: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecuteCommandParams {
    /// extends WorkDoneProgressParams
    /**
     * An optional token that a server can use to report work done progress.
     */
    pub workDoneToken: Option<ProgressToken>,
    /**
     * The identifier of the actual command handler.
     */
    pub command: String,
    /**
     * Arguments that the command should be invoked with.
     */
    pub arguments: Option<Vec<LSPAny>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplyWorkspaceEditParams {
    /**
     * An optional label of the workspace edit. This label is
     * presented in the user struct for example on an undo
     * stack to undo the workspace edit.
     */
    pub label: Option<String>,

    /**
     * The edits to apply.
     */
    pub edit: WorkspaceEdit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplyWorkspaceEditResult {
    /**
     * Indicates whether the edit was applied or not.
     */
    pub applied: Boolean,

    /**
     * An optional textual description for why the edit was not applied.
     * This may be used by the server for diagnostic logging or to provide
     * a suitable error for a request that triggered the edit.
     */
    pub failureReason: Option<String>,

    /**
     * Depending on the client's failure handling strategy `failedChange`
     * might contain the index of the change that failed. This property is
     * only available if the client signals a `failureHandling` strategy
     * in its client capabilities.
     */
    pub failedChange: Option<UInteger>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShowMessageParams {
    /**
     * The message type. See {@link MessageType}.
     */
    pub r#type: MessageType,

    /**
     * The actual message.
     */
    pub message: String,
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum MessageType {
    /**
     * An error message.
     */
    Error = 1,
    /**
     * A warning message.
     */
    Warning = 2,
    /**
     * An information message.
     */
    Info = 3,
    /**
     * A log message.
     */
    Log = 4,
    /**
     * A debug message.
     *
     * @since 3.18.0
     * @proposed
     */
    Debug = 5,
}

/// extracted out for [ShowMessageRequestClientCapabilities::messageActionItem]
#[derive(Serialize, Deserialize, Debug)]
pub struct ShowMessageRequestClientCapabilitiesMessageActionItem {
    /**
     * Whether the client supports additional attributes which
     * are preserved and sent back to the server in the
     * request's response.
     */
    pub additionalPropertiesSupport: Option<Boolean>,
}

/**
 * Show message request client capabilities
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct ShowMessageRequestClientCapabilities {
    /**
     * Capabilities specific to the `MessageActionItem` type.
     */
    pub messageActionItem: Option<ShowMessageRequestClientCapabilitiesMessageActionItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShowMessageRequestParams {
    /**
     * The message type. See {@link MessageType}
     */
    pub r#type: MessageType,

    /**
     * The actual message
     */
    pub message: String,

    /**
     * The message action items to present.
     */
    pub actions: Option<Vec<MessageActionItem>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageActionItem {
    /**
     * A short title like 'Retry', 'Open Log' etc.
     */
    pub title: String,
}

/**
 * Client capabilities for the show document request.
 *
 * @since 3.16.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct ShowDocumentClientCapabilities {
    /**
     * The client has support for the show document
     * request.
     */
    pub support: Boolean,
}

/**
 * Params to show a resource.
 *
 * @since 3.16.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct ShowDocumentParams {
    /**
     * The uri to show.
     */
    pub uri: URI,

    /**
     * Indicates to show the resource in an external program.
     * To show, for example, `https://code.visualstudio.com/`
     * in the default WEB browser set `external` to `true`.
     */
    pub external: Option<Boolean>,

    /**
     * An optional property to indicate whether the editor
     * showing the document should take focus or not.
     * Clients might ignore this property if an external
     * program is started.
     */
    pub takeFocus: Option<Boolean>,

    /**
     * An optional selection range if the document is a text
     * document. Clients might ignore the property if an
     * external program is started or the file is not a text
     * file.
     */
    pub selection: Option<Range>,
}

/**
 * The result of an show document request.
 *
 * @since 3.16.0
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct ShowDocumentResult {
    /**
     * A Boolean indicating if the show was successful.
     */
    pub success: Boolean,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogMessageParams {
    /**
     * The message type. See {@link MessageType}
     */
    pub r#type: MessageType,

    /**
     * The actual message
     */
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkDoneProgressCreateParams {
    /**
     * The token to be used to report progress.
     */
    pub token: ProgressToken,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkDoneProgressCancelParams {
    /**
     * The token to be used to report progress.
     */
    pub token: ProgressToken,
}
