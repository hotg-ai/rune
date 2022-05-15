import { runtime_v1 } from "@hotg-ai/rune-wit-files";

export type LogLevel = Lowercase<keyof typeof runtime_v1.LogLevel>;
export type LogValue = number | boolean | string | null;
export type LogPayload = Record<string, LogValue>;

/**
 * Metadata associated with a single logging event.
 */
export type LogMetadata = {
  /**
   * The log event's verbosity.
   */
  level: LogLevel;
  /**
   * The name of the section of code ("span") this event was emitted from.
   */
  span: string;
  /**
   * A string describing the part of the system where the span or event that
   * this metadata describes occurred.
   *
   * Typically, this is the module path, but alternate targets may be set when
   * spans or events are constructed.
   */
  target: string;
  file?: string;
  line?: number;
  module?: string;
};

/**
 * A logger that receives structured events.
 */
export interface Logger {
  /**
   * Log an event.
   *
   * @param metadata Information about the log message and where it came from.
   * @param message The message.
   * @param payload Additional, structured data that adds context to the log
   * message.
   */
  log(metadata: LogMetadata, message: string, payload: LogPayload): void;

  /**
   * Check if a message *would* be logged based on its metadata.
   *
   * This is an optimisation that consumers can use to avoid doing expensive
   * computations or logging large events that would just be thrown away.
   */
  isEnabled(metadata: LogMetadata): boolean;
}

/**
 * A structured logger intended for use within JavaScript.
 */
export class StructuredLogger {
  /**
   * Create a new structured logger.
   *
   * @param backend The logging backend messages are sent to.
   * @param component The name of the component being logged.
   */
  constructor(public readonly backend: Logger, public component: string) {}

  trace(msg: string, payload?: LogPayload) {
    this.log("trace", msg, payload);
  }

  debug(msg: string, payload?: LogPayload) {
    this.log("debug", msg, payload);
  }

  info(msg: string, payload?: LogPayload) {
    this.log("info", msg, payload);
  }

  warn(msg: string, payload?: LogPayload) {
    this.log("warn", msg, payload);
  }

  error(msg: string, payload?: LogPayload) {
    this.log("error", msg, payload);
  }

  fatal(msg: string, payload?: LogPayload) {
    this.log("fatal", msg, payload);
  }

  /**
   * Enter a named span.
   */
  span(name: string): StructuredLogger {
    return new Span(this.backend, this.component, name);
  }

  protected metadata(level: LogLevel): LogMetadata {
    return {
      level,
      // Note: We aren't within a span by default.
      span: "",
      target: this.component,
    };
  }

  log(level: LogLevel, msg: string, payload?: LogPayload) {
    const meta = this.metadata(level);

    if (this.backend.isEnabled(meta)) {
      this.backend.log(meta, msg, payload || {});
    }
  }
}

class Span extends StructuredLogger {
  constructor(backend: Logger, component: string, public name: string) {
    super(backend, component);
  }

  protected metadata(level: LogLevel): LogMetadata {
    const meta = super.metadata(level);
    meta.span = this.name;
    return meta;
  }
}

/**
 * A simple logger implementation that does the equivalent of console.log() for
 * each log level.
 */
export function consoleLogger(
  metadata: LogMetadata,
  message: string,
  payload: Record<string, string | number | boolean | null>
): void {
  const { level } = metadata;

  const fatal = (msg: string, ...args: any[]) => {
    console.error(msg, ...args);
    throw new Error(msg);
  };

  const logger = level == "fatal" ? fatal : console[level];

  logger(message, { metadata, payload });
}
