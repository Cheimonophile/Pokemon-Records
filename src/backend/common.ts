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
  const func = async (params: TParams): Promise<TResult> => {
    try {
      const result = await invoke(command, params);
      console.log(command)
      console.log(result)
      const parsed: TResult = schema.parse(result);
      return parsed
    }
    catch (error) {
      if (error instanceof z.ZodError) {
        const issue = error.issues[0]
        const message = `${command} ${issue.code} (${issue.path.join('.')}) ${issue.message}`
        throw new Error(message)
      }
      throw error
    }
  }
  return func;
}



/**
 * Zod parser for a version in the database
 */
export const ZodVersion = z.object({
  id: z.number(),
  name: z.string(),
  generation: z.number(),
})