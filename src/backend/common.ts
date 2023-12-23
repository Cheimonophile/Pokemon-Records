import { invoke } from '@tauri-apps/api';
import { InvokeArgs } from '@tauri-apps/api/tauri';
import z from 'zod';

/**
 * Type of the command function that is returned from the command decorator
 */
export type Command<T extends InvokeArgs> = (command: T) => any


/**
 * Decorator that creates a tauri access function from a command name and zod validator
 * 
 * @param command the tauri command to call
 * @param validator the zod validator for the response
 * @returns 
 */
export function command<TParams extends InvokeArgs, TSchema extends z.ZodTypeAny>(command: string, schema: TSchema) {
  type TResult = z.infer<typeof schema>
  try {
    const func = async (params: TParams): Promise<TResult> => {
      const result = await invoke(command, params);
      const parsed: TResult = schema.parse(result);
      return parsed
    }
    return func;
  }
  catch (error) {
    if (error instanceof z.ZodError) {
      const issue = error.issues[0]
      console.error(`${issue.code} (${issue.path.join('.')}): ${error.message}`);
    }
    throw error
  }
}