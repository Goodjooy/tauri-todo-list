
class IdIsUndefinedError extends Error{
    constructor(ty:string) {
        super(`The Id of \`${ty}\` is Undefined`);
    }
}

export default IdIsUndefinedError